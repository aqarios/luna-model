//! Analysis pass that computes minimum achievable values per constraint.

use std::{any::Any, collections::HashMap};

use crate::{analysis::utils::compute_minvalue, error::TransformError};
use lunamodel_capi_plugin_proto::proto::{
    Bound as ProtoBound, MinConstraintValues as ProtoMinConstraintValues,
};
use lunamodel_core::Model;
use lunamodel_transpiler::{
    AnalysisKey, AnalysisPass, PassContext, PipelineStep, TranspileKindResult, analysis,
};
use lunamodel_types::Bound;
use prost::Message;

#[derive(Clone, Debug)]
pub struct MinConstraintValues {
    pub vals: HashMap<String, Bound>,
}

#[analysis]
#[derive(Clone, Default)]
pub struct MinValueForConstraintAnalysis;

impl AnalysisPass for MinValueForConstraintAnalysis {
    type Result = MinConstraintValues;

    const PROVIDES: &'static str = "luna_model::min-value-for-constraint";

    fn name(&self) -> &str {
        "min-value-for-constraint"
    }

    fn key<MinConstraintValues>() -> AnalysisKey<MinConstraintValues> {
        AnalysisKey::new(Self::PROVIDES.to_string())
    }

    fn run(&self, model: &Model, _ctx: &PassContext) -> TranspileKindResult<Self::Result> {
        let mut minvalues = HashMap::new();
        for (name, constr) in model.constraints().iter() {
            if constr.lhs.has_quadratic() {
                Err(TransformError::Analysis {
                    name: self.name().to_owned(),
                    msg: format!(
                        "constraint '{name}' contains quadratic terms. This is not supported, constraints must be linear."
                    ),
                })?;
            }

            if constr.lhs.has_higher_order() {
                Err(TransformError::Analysis {
                    name: self.name().to_owned(),
                    msg: format!(
                        "constraint '{name}' contains higher-order terms. This is not supported, constraints must be linear."
                    ),
                })?;
            }

            let minvalue = compute_minvalue(constr.lhs.linear_items())?;
            minvalues.insert(name.clone(), minvalue);
        }

        Ok(MinConstraintValues { vals: minvalues })
    }
}

impl From<&MinConstraintValues> for ProtoMinConstraintValues {
    fn from(native: &MinConstraintValues) -> Self {
        Self {
            vals: native
                .vals
                .iter()
                .map(|(name, bound)| (name.clone(), to_proto_bound(*bound)))
                .collect(),
        }
    }
}

fn to_proto_bound(bound: Bound) -> ProtoBound {
    ProtoBound {
        value: match bound {
            Bound::Bounded(value) => Some(value),
            Bound::Unbounded => None,
        },
    }
}

/// Serializes a stored `MinConstraintValues` to the shared protobuf wire from a plugin decodes.
pub(crate) fn serialize_min_constraint_values(value: &(dyn Any + Send + Sync)) -> Option<Vec<u8>> {
    let native = value.downcast_ref::<MinConstraintValues>()?;
    Some(ProtoMinConstraintValues::from(native).encode_to_vec())
}
