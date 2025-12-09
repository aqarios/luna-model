mod exprtree;

use lunamodel_core::Expression;

use crate::{CustomFormat, FormatOpt, expr::exprtree::ExprTree};

impl CustomFormat<FormatOpt> for Expression {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, _: FormatOpt) -> std::fmt::Result {
        let tree: ExprTree = self.into();
        write!(fmt, "{}", tree.to_string())
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, _: FormatOpt) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}
