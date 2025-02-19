use std::{
    hash::Hash,
    ops::{AddAssign, MulAssign, SubAssign},
};

use nohash::IsEnabled;

use super::term::Variables;

pub trait Key: IsEnabled + Eq + Hash + Ord + Copy {}
impl<K: IsEnabled + Eq + Hash + Ord + Copy> Key for K {}

pub trait Term<K: Key> {
    fn reset(&mut self);
    fn new_from_other(other: &Self) -> Self;
    fn has_variables(&self) -> bool;
    fn mutable_variables(&mut self) -> &mut Variables<K>;
    fn variables(&self) -> &Variables<K>;
    fn fill_variables(&mut self, variables: Variables<K>) -> &mut Variables<K>;
    // fn mutable_variables(&mut self) -> &mut Variables<K>;
    // fn variables(&self) -> &Variables<K>;
    // fn fill_variables(&mut self, variables: Variables<K>) -> &mut Variables<K>;
}

pub trait TermAddition<T: Key>
where
    Self: Term<T> + Sized,
{
    fn add(&self, rhs: &Self) -> Self {
        // If the `self` variables are not present we can directly return a copy
        // of the `rhs` variables as a new linear term.
        if !self.has_variables() {
            return Self::new_from_other(&rhs);
        }
        // From here, we know that `self` contains values.
        // If the `rhs` variables are not present we can directly return a copy
        // of the `self` variables as a new linear term.
        if !rhs.has_variables() {
            return Self::new_from_other(&self);
        }
        // Now both `self.variables` and `rhs.variables` have values.
        // So we can start from either the `self` or the `rhs` term.
        // We choose the `self` term here.
        let mut out = Self::new_from_other(&self);
        let out_vars = out.mutable_variables();

        // We can now insert the values from `rhs`.
        for (key, value) in rhs.variables().iter() {
            match out_vars.get_mut(key) {
                Some(e) => {
                    e.add_assign(value);
                    if *e == 0.0 {
                        out_vars.remove(key);
                    }
                }
                None => _ = out_vars.insert(*key, *value),
            }
        }
        out
    }
    fn add_assign(&mut self, rhs: &Self) {
        // If other value does not contain variables than we do not need
        // to do anything. Current linear term stays as is.
        if !rhs.has_variables() {
            return;
        }

        // We need to insert the rhs variables into self and the
        // current self does not contain any values itself.
        if !self.has_variables() {
            // println!("self does not have variables");
            let vars = rhs.variables();
            let _ = self.fill_variables(vars.clone());
            return;
        }
        // Now we know that both `self.variables` and `rhs.variables`
        // contain values. We need to merge them using the add operation.
        // mutable variables of self (mutable reference).

        let selfvars = self.mutable_variables();

        // println!("Num. entries: {}", selfvars.len());
        // println!("Num. entries in rhs: {}", rhs.variables().len());
        // println!("Capacity: {}", selfvars.capacity());
        // Naive trial reserving extra size beforehand.
        // if selfvars.len() + 1 >= selfvars.capacity() {
        //     // println!("in here");
        //     selfvars.reserve(10_000);
        // }

        // let iter_start = Instant::now();
        for (key, value) in rhs.variables().iter() {
            // if *value == 0.0 {
            //     // don't need to do anything...
            //     continue;
            // }
            match selfvars.get_mut(key) {
                Some(e) => {
                    // println!("Rhs is contained");
                    e.add_assign(value);
                    if *e == 0.0 {
                        selfvars.remove(key);
                    }
                }
                None => {
                    // println!("Rhs not contained, key is {}", key);
                    _ = selfvars.insert(*key, *value);
                }
            }
        }
        // let iter_elapsed = iter_start.elapsed();
        // println!("Insertion took: {:?}\n-----", iter_elapsed);
    }
}

pub trait TermSubtraction<T: Key>
where
    Self: Term<T> + Sized,
{
    fn sub(&self, rhs: &Self) -> Self {
        // If the `self` variables are not present we can directly return a copy
        // of the `rhs` variables as a new linear term.
        // We subtract the current (`self`) linear term for `0`. Thus we need the
        // sign flipped for all values.
        // Essentially, we compute 0 - `self`
        if !self.has_variables() {
            let mut out = Self::new_from_other(&rhs);
            // We subtract the current (`self`) linear term for `0`. Thus we need the
            // sign flipped for all values, i.e., multiply each value by `-1`.
            // todo: is there something faster/better to achieve this??
            out.mutable_variables().iter_mut().for_each(|(_, value)| {
                value.mul_assign(-1.0);
            });
            return out;
        }
        // From here, we know that `self` contains values.
        // If the `rhs` variables are not present we can directly return a copy
        // of the `self` variables as a new linear term.
        // Basically we subtract `0` from the current (`self`) linear term.
        if !rhs.has_variables() {
            return Self::new_from_other(&self);
        }
        // Now both `self.variables` and `rhs.variables` have values.
        // So we can start from either the `self` or the `rhs` term.
        // We choose the `self` term here and compute `self - rhs`
        let mut out = Self::new_from_other(&self);
        let out_vars = out.mutable_variables();

        // We can now insert the values from `rhs`.
        for (key, value) in rhs.variables().iter() {
            match out_vars.get_mut(key) {
                Some(e) => {
                    e.sub_assign(value);
                    if *e == 0.0 {
                        out_vars.remove(key);
                    }
                }
                None => _ = out_vars.insert(*key, *value),
            }
        }
        out
    }
    fn sub_assign(&mut self, rhs: &Self) {
        // If other value does not contain variables than we do not need
        // to do anything. Current linear term stays as is.
        // self - rhs = self - 0 = self
        if !rhs.has_variables() {
            return;
        }

        // We need to insert the rhs variables into self and the
        // current self does not contain any values itself.
        // self - rhs = 0 - rhs = - rhs;
        if !self.has_variables() {
            let vars = rhs.variables();
            let selfvars = self.fill_variables(vars.clone());
            selfvars.iter_mut().for_each(|(_, value)| {
                value.mul_assign(-1.0);
            });
            return;
        }
        // Now we know that both `self.variables` and `rhs.variables`
        // contain values. We need to merge them using the add operation.
        // mutable variables of self (mutable reference).
        let selfvars = self.mutable_variables();
        for (key, value) in rhs.variables().iter() {
            match selfvars.get_mut(key) {
                Some(e) => {
                    e.sub_assign(value);
                    if *e == 0.0 {
                        selfvars.remove(key);
                    }
                }
                None => _ = selfvars.insert(*key, *value),
            }
        }
    }
}

pub trait TermFloatMultiplication<T: Key>
where
    Self: Term<T> + Sized,
{
    fn mul(&self, rhs: f64) -> Self {
        let mut out = Self::new_from_other(&self);
        if rhs == 1.0 || !out.has_variables() {
            return out;
        }
        let outvars = out.mutable_variables();
        for (_, value) in outvars.iter_mut() {
            value.mul_assign(rhs);
        }
        out
    }

    fn mul_assign(&mut self, rhs: f64) {
        if rhs == 1.0 || !self.has_variables() {
            return;
        }
        let selfvars = self.mutable_variables();
        for (_, value) in selfvars.iter_mut() {
            value.mul_assign(rhs);
        }
    }
}
