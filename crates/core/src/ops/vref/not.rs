use std::ops::Not;

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Vtype;

use crate::{Environment, prelude::VarRef};

impl Not for &VarRef {
    type Output = LunaModelResult<VarRef>;

    fn not(self) -> Self::Output {
        self.check_living()?;
        let vtype = self.env.read_arc()[self.id].vtype;
        match vtype {
            Vtype::Binary => {
                // First, we need to check that this variable is not already inverted.
                let var = self.env.read_arc()[self.id].clone();
                let inv = match var.inverted {
                    Some(inverted) => inverted,
                    None => {
                        let mutenv: &mut Environment = &mut self.env.write_arc();
                        let invid = mutenv.insert_inverted(self)?;
                        mutenv[invid].inverted = Some(self.id);
                        mutenv[self.id].inverted = Some(invid);
                        invid
                    }
                };
                Ok(VarRef::new(inv, self.env.clone()))
            }
            Vtype::InvertedBinary => Ok(VarRef::new(
                self.env.read_arc()[self.id].inverted.unwrap(),
                self.env.clone(),
            )),
            _ => Err(LunaModelError::UnsupportedOperation("not".into())),
        }
    }
}
