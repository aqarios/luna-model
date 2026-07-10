//! Built-in `IfElse` control-flow implementation.

use std::{collections::BTreeSet, sync::Arc};

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::{
    ControlFlowPass, ControlFlowPlan, DisplaySteps, PassContext, PipelineStep, PipelineStepMethods,
    TranspileKindResult, control_flow,
};
use pad::PadStr;

use crate::utils::unique_name;

pub trait ConditionPredicate: Send + Sync {
    fn eval(&self, model: &Model, ctx: &PassContext) -> LunaModelResult<bool>;
}
impl<F> ConditionPredicate for F
where
    F: Fn(&Model, &PassContext) -> LunaModelResult<bool> + Send + Sync,
{
    fn eval(&self, model: &Model, ctx: &PassContext) -> LunaModelResult<bool> {
        self(model, ctx)
    }
}

#[control_flow]
#[derive(Clone)]
pub struct IfElsePass {
    name: String,
    predicate: Arc<dyn ConditionPredicate>,
    then_steps: Vec<PipelineStep>,
    else_steps: Vec<PipelineStep>,
    requires: Vec<String>,
    invalidates: Vec<String>,
    provides: Vec<String>,
}

impl IfElsePass {
    pub fn new(
        predicate: Arc<dyn ConditionPredicate>,
        then_steps: Vec<PipelineStep>,
        else_steps: Vec<PipelineStep>,
        name: Option<String>,
    ) -> Self {
        let mut req_set = BTreeSet::new();
        req_set.extend(then_steps.collect_requires());
        req_set.extend(else_steps.collect_requires());

        let mut inv_set = BTreeSet::new();
        inv_set.extend(then_steps.collect_invalidates());
        inv_set.extend(else_steps.collect_invalidates());

        let mut prov_set = BTreeSet::new();
        prov_set.extend(then_steps.collect_provides());
        prov_set.extend(else_steps.collect_provides());

        Self {
            requires: req_set.into_iter().collect(),
            invalidates: inv_set.into_iter().collect(),
            provides: prov_set.into_iter().collect(),
            predicate,
            then_steps,
            else_steps,
            name: name.unwrap_or_else(|| unique_name("if-else")),
        }
    }
}

impl ControlFlowPass for IfElsePass {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self, model: &Model, ctx: &PassContext) -> TranspileKindResult<ControlFlowPlan> {
        let cond = self.predicate.eval(model, ctx)?;

        let (steps, name) = if cond {
            (self.then_steps.clone(), format!("{}-then", self.name))
        } else {
            (self.else_steps.clone(), format!("{}-else", self.name))
        };

        Ok(ControlFlowPlan::new(name, steps))
    }

    fn requires(&self) -> &[String] {
        &self.requires
    }

    fn provides(&self) -> &[String] {
        &self.provides
    }

    fn invalidates(&self) -> &[String] {
        &self.invalidates
    }

    fn display(&self) -> String {
        let mut out = String::default();

        if (self.then_steps.is_empty()) && (self.else_steps.is_empty()) {
            out.push_str(&format!("❔ {} (empty)", self.name));
            return out;
        }

        let str_then = self.then_steps.display();
        let str_otherwise = self.else_steps.display();

        let mut then: Vec<_> = str_then.split("\n").collect();
        let mut otherwise: Vec<_> = str_otherwise.split("\n").collect();

        let maybe_then_width_max = then.iter().max_by(|a, b| a.len().cmp(&b.len()));
        if maybe_then_width_max.is_none() {
            return out;
        }
        let target_width = maybe_then_width_max.unwrap().len();

        if then.len() > otherwise.len() {
            otherwise.resize(then.len(), "");
        } else if then.len() < otherwise.len() {
            then.resize(otherwise.len(), "");
        }

        let final_then: Vec<_> = then.iter().map(|s| s.pad_to_width(target_width)).collect();
        let final_otherwise: Vec<_> = otherwise.iter().map(|s| s.to_string()).collect();

        let title_then = CHECK_MARK
            .to_string()
            .pad_to_width_with_alignment(target_width - 1, pad::Alignment::Left);
        let title_otherwise = BALLOT_X
            .to_string()
            .pad_to_width_with_alignment(target_width, pad::Alignment::Left);

        let ext_then = format!("{U_AND_R} {title_then}");
        let ext_a_else = format!(
            "{}{D_AND_L}",
            H_BAR.repeat(target_width - self.name.len() + 6)
        );
        let ext_b_else = format!("  {title_otherwise}");

        out.push_str(&format!("❔ {} {ext_a_else}\n", self.name));
        out.push_str(&format!("   {ext_then}   {ext_b_else}\n"));
        for (i, (t, o)) in final_then.iter().zip(&final_otherwise).enumerate() {
            let end = if i < final_then.len() - 1 { "\n" } else { "" };
            let limiter_a = match (t.is_empty(), &final_then.get(i + 1)) {
                (false, Some(x)) => {
                    if x.is_empty() {
                        U_AND_R
                    } else {
                        V_AND_R
                    }
                }
                (false, None) => U_AND_R,
                (true, None) => "",
                _ => "",
            };
            let limiter_b = match (o.is_empty(), &final_otherwise.get(i + 1)) {
                (false, Some(x)) => {
                    if x.is_empty() {
                        U_AND_R
                    } else {
                        V_AND_R
                    }
                }
                (false, None) => U_AND_R,
                (true, None) => "",
                _ => "",
            };
            out.push_str(&format!("        {limiter_a} {t}  {limiter_b} {o}{end}"));
        }

        out
    }
}

static H_BAR: &str = "\u{2500}";
static D_AND_L: &str = "\u{2510}";
static U_AND_R: &str = "\u{2514}";
static V_AND_R: &str = "\u{251C}";
static CHECK_MARK: &str = "\u{2714}";
static BALLOT_X: &str = "\u{274C}";
