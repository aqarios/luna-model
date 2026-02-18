use std::clone::Clone;
use std::fmt::Display;

use dyn_clone::DynClone;
use global_counter::primitive::exact::CounterU64;
use lunamodel_core::{Model, Solution};
use lunamodel_error::{LunaModelError, LunaModelResult};
use pad::PadStr;

use super::abstract_pipeline::AbstractPipeline;
use crate::{
    Pass, base::BasePass, cache::{AnalysisCache, AnalysisCacheElement}, ir::IR, log::ExecutionLog, pass_manager::PassManager, unicode::{BALLOT_X, CHECK_MARK, D_AND_L, H_BAR, U_AND_R, V_AND_R}
};

#[cfg_attr(feature = "py", pyo3::pyclass(get_all))]
#[derive(Debug, Clone)]
pub struct IfElseInfo {
    fulfilled_condition: bool,
}

pub struct IfElseOutcome {
    pub ir: IR,
    pub analysis: AnalysisCacheElement,
}

pub type IfElsePassResult = LunaModelResult<IfElseOutcome>;

/// Counter to ensure multiple if-else branches can be used in the same pass.
pub static IF_ELSE_COUNTER: CounterU64 = CounterU64::new(0);

pub trait Condition: std::fmt::Debug + DynClone {
    fn call(&self, cache: &AnalysisCache) -> LunaModelResult<bool>;
}
dyn_clone::clone_trait_object!(Condition);

#[derive(Debug, Clone)]
pub struct IfElsePass {
    requires: Vec<String>,
    condition: Box<dyn Condition>,
    then: Box<dyn AbstractPipeline>,
    otherwise: Box<dyn AbstractPipeline>,
    // #[py_pass(init_ignore)]
    name: String,
}

impl IfElsePass {
    pub fn new(
        requires: Vec<String>,
        condition: Box<dyn Condition>,
        then: Box<dyn AbstractPipeline>,
        otherwise: Box<dyn AbstractPipeline>,
        name: Option<String>,
    ) -> Self {
        let mut requires = requires;
        requires.append(&mut then.requires());
        requires.append(&mut otherwise.requires());
        IfElsePass {
            requires,
            condition,
            then,
            otherwise,
            name: name.unwrap_or_else(|| format!("if-else-{}", IF_ELSE_COUNTER.inc())),
        }
    }
}

impl BasePass for IfElsePass {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn requires(&self) -> Vec<String> {
        self.requires.clone()
    }
}

impl IfElsePass {
    pub fn run(
        &self,
        model: Model,
        cache: &AnalysisCache,
        executor: &PassManager,
    ) -> IfElsePassResult {
        let is_condition = self
            .condition
            .call(cache)
            .map_err(|err| LunaModelError::IfElsePass(err.to_string().into()))?;
        let ir = if is_condition {
            self.then
                .run(model, &cache, executor)
                .map_err(|err| LunaModelError::IfElsePass(err.to_string().into()))
        } else {
            self.otherwise
                .run(model, &cache, executor)
                .map_err(|err| LunaModelError::IfElsePass(err.to_string().into()))
        }?;
        Ok(IfElseOutcome {
            ir,
            analysis: AnalysisCacheElement::IfElseInfoAnalysis(IfElseInfo {
                fulfilled_condition: is_condition,
            }),
        })
    }

    pub fn backwards(&self, mut solution: Solution, ir: &IR, log: &ExecutionLog) -> Solution {
        match ir.cache.get(&self.name) {
            Some(AnalysisCacheElement::IfElseInfoAnalysis(cache)) => {
                if cache.fulfilled_condition {
                    solution = self.then.backwards(solution, ir, log)
                } else {
                    solution = self.otherwise.backwards(solution, ir, log)
                }
            }
            _ => {}
        }
        solution
    }
}

impl Display for IfElsePass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if (self.then.len() == 0) && (self.otherwise.len() == 0) {
            write!(f, "❔ {} (empty)", self.name)?;
            return Ok(());
        }

        let str_then = self.then.content_string();
        let str_otherwise = self.otherwise.content_string();

        let mut then: Vec<_> = str_then.split("\n").collect();
        let mut otherwise: Vec<_> = str_otherwise.split("\n").collect();

        let maybe_then_width_max = then.iter().max_by(|a, b| a.len().cmp(&b.len()));
        if maybe_then_width_max.is_none() {
            return write!(f, "");
        }
        let target_width = maybe_then_width_max.unwrap().len();

        if then.len() > otherwise.len() {
            otherwise.resize(then.len(), "");
        } else if then.len() < otherwise.len() {
            then.resize(otherwise.len(), "");
        }

        let final_then: Vec<_> = then.iter().map(|s| s.pad_to_width(target_width)).collect();
        let final_otherwise: Vec<_> = otherwise.iter().map(|s| s.to_string()).collect();

        let title_then = format!("{CHECK_MARK}  {}", self.then.name())
            .pad_to_width_with_alignment(target_width - 1, pad::Alignment::Left);
        let title_otherwise = format!("{BALLOT_X} {}", self.otherwise.name())
            .pad_to_width_with_alignment(target_width, pad::Alignment::Left);

        let ext_then = format!("{U_AND_R} {title_then}");
        let ext_a_else = format!(
            "{}{D_AND_L}",
            H_BAR.repeat(target_width - self.name.len() + 6)
        );
        let ext_b_else = format!("  {title_otherwise}");

        write!(f, "❔ {} {ext_a_else}\n", self.name)?;
        write!(f, "   {ext_then}   {ext_b_else}\n")?;
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
            write!(f, "        {limiter_a} {t}  {limiter_b} {o}{end}")?;
        }
        Ok(())
    }
}

impl Into<Pass> for IfElsePass {
    fn into(self) -> Pass {
        Pass::IfElse(self)
    }
}
