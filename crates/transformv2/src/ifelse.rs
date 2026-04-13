use std::{collections::BTreeSet, sync::Arc};

use global_counter::primitive::exact::CounterU64;
use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::{
    ControlFlowPass, ControlFlowPlan, PassContext, PipelineStep, PipelineStepMethods,
};

/// Counter to ensure multiple if-else branches can be used in the same pass.
pub static IF_ELSE_COUNTER: CounterU64 = CounterU64::new(0);

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
            name: name.unwrap_or_else(|| format!("if-else-{}", IF_ELSE_COUNTER.inc())),
        }
    }
}

impl ControlFlowPass for IfElsePass {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self, model: &Model, ctx: &PassContext) -> LunaModelResult<ControlFlowPlan> {
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
}
