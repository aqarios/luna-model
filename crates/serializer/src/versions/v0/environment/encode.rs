use lunamodel_core::prelude::Environment;
use lunamodel_types::{Bound, Vtype};
use prost::Message;

use crate::encode::BytesEncodable;

use super::SerEnvironment;

impl BytesEncodable for SerEnvironment {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl SerEnvironment {
    pub fn fill(mut self, env: &Environment) -> Self {
        self.varcount = env.len() as u32;
        self.next_idx = env.next_idx();

        for idx in env.vars() {
            let var = &env[idx];
            match var.vtype() {
                Vtype::Binary => {
                    self.binary.push(idx);
                    self.binary_names.push(var.name().into());
                    match var.inverted {
                        Some(inv) => {
                            self.binary_is_inverted.push(true);
                            self.inverted_binary.push(inv);
                        }
                        None => self.binary_is_inverted.push(false),
                    }
                }
                Vtype::InvertedBinary => {
                    // taken care of in the Binary match.
                    continue;
                }
                Vtype::Spin => {
                    self.spin.push(idx);
                    self.spin_names.push(var.name().into());
                }
                Vtype::Integer => {
                    self.integer.push(idx);
                    self.integer_names.push(var.name().into());
                    let vbounds = var.bounds();
                    match vbounds.lower() {
                        Bound::Bounded(l) => {
                            self.integer_bounds_has_lower.push(true);
                            self.integer_bounds_lower.push(l);
                        }
                        Bound::Unbounded => self.integer_bounds_has_lower.push(false),
                    }
                    match vbounds.upper() {
                        Bound::Bounded(u) => {
                            self.integer_bounds_has_upper.push(true);
                            self.integer_bounds_upper.push(u);
                        }
                        Bound::Unbounded => self.integer_bounds_has_upper.push(false),
                    }
                }
                Vtype::Real => {
                    self.real.push(idx);
                    self.real_names.push(var.name().into());
                    let vbounds = var.bounds();
                    match vbounds.lower() {
                        Bound::Bounded(l) => {
                            self.real_bounds_has_lower.push(true);
                            self.real_bounds_lower.push(l);
                        }
                        Bound::Unbounded => self.real_bounds_has_lower.push(false),
                    }
                    match vbounds.upper() {
                        Bound::Bounded(u) => {
                            self.real_bounds_has_upper.push(true);
                            self.real_bounds_upper.push(u);
                        }
                        Bound::Unbounded => self.real_bounds_has_upper.push(false),
                    }
                }
            }
        }

        self
    }
}
