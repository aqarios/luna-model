use hashbrown::HashMap;

pub struct HigherOrder<Index, Bias> {
    biases: HashMap<Index, Bias>,
}
