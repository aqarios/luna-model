use std::sync::{Arc, Mutex, MutexGuard};

use derive_more::{Deref, DerefMut};

#[derive(Debug, Deref, DerefMut)]
pub struct Share<T>(Arc<T>);

impl<T> Share<T> {
    pub fn new(t: T) -> Share<T> {
        Share(Arc::new(t))
    }
}

impl<T> Clone for Share<T> {
    fn clone(&self) -> Self {
        Share(self.0.clone())
    }
}

impl<T> From<T> for Share<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

// #[derive(Debug, Deref, DerefMut)]
#[derive(Debug)]
pub struct ShareMut<T>(Arc<Mutex<T>>);

impl<T> ShareMut<T> {
    #[inline]
    pub fn new(t: T) -> ShareMut<T> {
        ShareMut(Arc::new(Mutex::new(t)))
    }

    #[inline]
    pub fn access(&self) -> MutexGuard<'_, T> {
        self.0.lock().unwrap()
    }

    #[inline]
    pub fn access_mut(&self) -> MutexGuard<'_, T> {
        self.0.lock().unwrap()
    }

    #[inline]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Clone for ShareMut<T> {
    #[inline]
    fn clone(&self) -> Self {
        ShareMut(self.0.clone())
    }
}

impl<T> ShareMut<T> {
    pub fn into_inner(obj: Self) -> Option<T> {
        let mutex = Arc::into_inner(obj.0)?;
        mutex.into_inner().ok()
    }
}
