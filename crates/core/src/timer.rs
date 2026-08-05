use std::{sync::Arc, time::SystemTime};

use indexmap::IndexMap;
use lunamodel_error::{LunaModelError, LunaModelResult};
use parking_lot::RwLock;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Timing {
    pub start: Option<SystemTime>,
    pub end: Option<SystemTime>,
    pub total: f64,
    pub timings: IndexMap<String, Vec<f64>>,
}

impl Timing {
    /// Creates a timing record from explicit timestamps.
    pub fn new(total: f64) -> Self {
        Self {
            total,
            ..Default::default()
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: f64) {
        _ = self.timings.insert(key.into(), vec![value])
    }

    pub fn add(&mut self, key: impl Into<String>, value: f64) {
        self.timings.entry(key.into()).or_default().push(value);
    }

    pub fn get(&self, key: impl Into<String>) -> &[f64] {
        self.timings.get(&key.into()).map_or(&[], |v| v.as_slice())
    }

    pub fn total_for(&self, key: impl Into<String>) -> Option<f64> {
        self.timings.get(&key.into()).map(|v| v.iter().sum())
    }

    pub fn merge(&mut self, other: &Self) {
        self.total += other.total;
        for (key, values) in other.timings.iter() {
            self.timings.entry(key.clone()).or_default().extend(values);
        }
    }
}

pub struct SubTimer {
    start: SystemTime,
    key: String,
    reference: Arc<RwLock<TimerState>>,
}

impl SubTimer {
    /// Stops the sub-timer and records its elapsed time on the parent [`Timer`].
    pub fn stop(self) -> LunaModelResult<f64> {
        let elapsed = SystemTime::now()
            .duration_since(self.start)
            .map_err(|e| LunaModelError::Internal(e.to_string().into()))?
            .as_secs_f64();
        self.reference
            .write()
            .timings
            .entry(self.key)
            .or_default()
            .push(elapsed);
        Ok(elapsed)
    }
}

struct TimerState {
    timings: IndexMap<String, Vec<f64>>,
}

/// Lightweight stopwatch used while constructing solution metadata.
///
/// Cheap to clone: clones share the same underlying state via `Arc`, so
/// sub-timers created concurrently (e.g. across threads) all record into
/// the same map.
#[derive(Clone)]
pub struct Timer {
    start: SystemTime,
    state: Arc<RwLock<TimerState>>,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    /// Starts a new wall-clock timer at `SystemTime::now()`.
    pub fn new() -> Self {
        Self {
            start: SystemTime::now(),
            state: Arc::new(RwLock::new(TimerState {
                timings: IndexMap::default(),
            })),
        }
    }

    pub fn start(&self, timing: impl Into<String>) -> SubTimer {
        SubTimer {
            start: SystemTime::now(),
            key: timing.into(),
            reference: Arc::clone(&self.state),
        }
    }

    /// Stops the timer and creates a [`Timing`] record.
    pub fn stop(self) -> LunaModelResult<Timing> {
        let end = SystemTime::now();
        let total = end
            .duration_since(self.start)
            .map_err(|e| LunaModelError::Internal(e.to_string().into()))?
            .as_secs_f64();
        let state = self.state.read();
        Ok(Timing {
            start: Some(self.start),
            end: Some(end),
            total,
            timings: state.timings.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use super::*;

    #[test]
    fn timing_set_overwrites_existing_values() {
        let mut timing = Timing::new(1.0);
        timing.add("a", 1.0);
        timing.add("a", 2.0);
        timing.set("a", 5.0);
        assert_eq!(timing.get("a"), &[5.0]);
    }

    #[test]
    fn timing_add_appends_to_existing_key() {
        let mut timing = Timing::new(1.0);
        timing.add("a", 1.0);
        timing.add("a", 2.0);
        assert_eq!(timing.get("a"), &[1.0, 2.0]);
    }

    #[test]
    fn timing_get_missing_key_returns_empty_slice() {
        let timing = Timing::new(1.0);
        assert_eq!(timing.get("missing"), &[] as &[f64]);
    }

    #[test]
    fn timing_total_without_key_returns_overall_total() {
        let timing = Timing::new(42.0);
        assert_eq!(timing.total, 42.0);
    }

    #[test]
    fn timing_total_with_key_sums_that_keys_values() {
        let mut timing = Timing::new(42.0);
        timing.add("a", 1.0);
        timing.add("a", 2.0);
        assert_eq!(timing.total_for("a"), Some(3.0));
    }

    #[test]
    fn timing_total_with_missing_key_returns_zero() {
        let timing = Timing::new(42.0);
        assert_eq!(timing.total_for("missing"), None);
    }

    #[test]
    fn timer_stop_records_elapsed_total() {
        let timer = Timer::new();
        thread::sleep(Duration::from_millis(5));
        let timing = timer.stop().unwrap();
        assert!(timing.total > 0.0);
        assert!(timing.timings.is_empty());
    }

    #[test]
    fn fine_grained_timer_records_under_its_key() {
        let timer = Timer::new();

        let preprocessing = timer.start("preprocessing");
        thread::sleep(Duration::from_millis(5));
        preprocessing.stop().unwrap();

        let timing = timer.stop().unwrap();
        let recorded = timing.get("preprocessing");
        assert_eq!(recorded.len(), 1);
        assert!(recorded[0] > 0.0);
    }

    #[test]
    fn repeated_sub_timers_accumulate_under_same_key() {
        let timer = Timer::new();

        for _ in 0..3 {
            let sub = timer.start("qpu");
            sub.stop().unwrap();
        }

        let timing = timer.stop().unwrap();
        assert_eq!(timing.get("qpu").len(), 3);
    }

    #[test]
    fn sub_timers_started_concurrently_all_record() {
        let timer = Timer::new();

        let handles: Vec<_> = (0..8)
            .map(|_| {
                let timer = timer.clone();
                thread::spawn(move || {
                    let sub = timer.start("parallel");
                    thread::sleep(Duration::from_millis(1));
                    sub.stop().unwrap();
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let timing = timer.stop().unwrap();
        assert_eq!(timing.get("parallel").len(), 8);
    }
}
