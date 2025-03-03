use crate::core::expression::{BiasConstraints, IndexConstraints};
use crate::core::Environment;
use hashbrown::{hash_map::Iter, HashMap};
use std::cell::Ref;
use std::io::BufRead;
use std::{
    marker::PhantomData,
    ops::{Index, IndexMut, MulAssign},
};

static DELIMITER: &str = "-";

#[derive(Clone, Debug)]
pub struct HigherOrder<Index, Bias> {
    biases: HashMap<String, Bias>,
    phantom_data: PhantomData<Index>, // required for compiler to acknowledge the Index
    default_bias: Bias,
}

impl<Index, Bias> HigherOrder<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub fn default() -> Self {
        Self {
            biases: HashMap::default(),
            phantom_data: PhantomData,
            default_bias: Bias::default(),
        }
    }

    fn make_key(index: &Vec<Index>) -> String {
        let mut indices = index.clone();
        indices.sort();
        indices
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(DELIMITER)
    }

    fn key_contributions(key: &String) -> Vec<Index> {
        key.split(DELIMITER)
            .map(|s| Index::from_str(s).ok().unwrap())
            .collect()
        // ok().unwrap() instead of unwrap() to get rid of the error for now. needs
        // fixing ??
    }

    pub fn is_empty(&self) -> bool {
        self.biases.len() == 0
    }

    pub fn iter(&self) -> Iter<String, Bias> {
        self.biases.iter()
    }

    pub fn iter_contrib(&self) -> impl Iterator<Item = (Vec<Index>, &Bias)> {
        self.biases
            .iter()
            .map(|(key, bias)| (HigherOrder::<Index, Bias>::key_contributions(&key), bias))
    }

    pub fn resize(&mut self, _: usize) {}

    pub fn display(&self, env: Ref<Environment<Index>>) -> String {
        let mut out = String::new();
        for (indices, bias) in self.iter_contrib() {
            if *bias != Bias::zero() {
                // This would be nice, but it doesn't seem to work without cloning the strings.
                // let vnames: Vec<_> = indices
                //     .iter()
                //     .map(|&idx| &env.variables[idx.into()].name)
                //     .collect();

                let mut vnames = String::new();
                for idx in indices.iter() {
                    let i = (*idx).into();
                    if i > 0 {
                        vnames += " * ";
                    }
                    vnames += &env.variables[i].name;
                }
                out += &format!(" {} {vnames}", bias.to_bias_string());
            }
        }
        out
    }
}

impl<Index, Bias> MulAssign<Bias> for HigherOrder<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn mul_assign(&mut self, rhs: Bias) {
        for (_, value) in self.biases.iter_mut() {
            *value *= rhs;
        }
    }
}

impl<Idx, Bias> Index<&Vec<Idx>> for HigherOrder<Idx, Bias>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Bias;
    fn index(&self, index: &Vec<Idx>) -> &Self::Output {
        let key = Self::make_key(index);
        self.biases.get(&key).unwrap_or(&self.default_bias)
    }
}

impl<Idx, Bias> IndexMut<&Vec<Idx>> for HigherOrder<Idx, Bias>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
{
    fn index_mut(&mut self, index: &Vec<Idx>) -> &mut Self::Output {
        let key = Self::make_key(index);
        if !self.biases.contains_key(&key) {
            self.biases.insert(key.to_string(), Bias::default());
        }
        self.biases.get_mut(&key).unwrap()
    }
}

impl<Idx, Bias> Index<&String> for HigherOrder<Idx, Bias>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Bias;

    fn index(&self, index: &String) -> &Self::Output {
        self.biases.get(index).unwrap_or(&self.default_bias)
    }
}

impl<Idx, Bias> IndexMut<&String> for HigherOrder<Idx, Bias>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
{
    fn index_mut(&mut self, index: &String) -> &mut Self::Output {
        if !self.biases.contains_key(index) {
            self.biases.insert(index.to_string(), Bias::default());
        }
        self.biases.get_mut(index).unwrap()
    }
}

impl<Index, Bias> PartialEq for HigherOrder<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn eq(&self, other: &Self) -> bool {
        // This basic check is no gurantee for actual equality.
        // As if this is not equal it might be due to different representations,
        // e.g., in one expression the interaction between two variables can be explicitly
        // contained as 0.0, while in an other expression this interaction is not
        // represented directly. The value of the interaction is still 0.0.
        // Thus they are equal... This is not handled by the below trivial comparison.
        //
        // self.biases == other.biases
        for lhs_idx in self.biases.keys() {
            for rhs_idx in other.biases.keys() {
                if self[lhs_idx] != other[rhs_idx] {
                    return false;
                }
            }
        }

        true
    }
}
