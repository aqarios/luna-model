use lunamodel_transpiler::{DisplaySteps, PassManager, Pipeline};
use std::fmt;

use crate::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for PassManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, _: &FormatOpt) -> fmt::Result {
        let basename = "PassManager\n";
        write!(f, "{basename}{}", self.steps().display())
    }
}

impl CustomFormat<FormatOpt> for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, _: &FormatOpt) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}
