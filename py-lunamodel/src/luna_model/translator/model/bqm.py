import numpy as np
from dimod import BinaryQuadraticModel

from luna_model._lm import PyBqmTranslator
from luna_model.model.model import Model
from luna_model.variable.vtype import Vtype


class BqmTranslator:
    @staticmethod
    def to_lm(bqm: BinaryQuadraticModel, *, name: str | None = None) -> Model:
        if not isinstance(bqm, BinaryQuadraticModel):
            raise TypeError(f"Expected bqm to be of type BQM, received: {type(bqm)}")
        bqm_vars_ser = bqm.variables.to_serializable()
        for v in bqm_vars_ser:
            if not isinstance(v, str):
                raise TypeError(
                    f"All BQM variables have to be of type str, received: {type(v)}"
                )
        vars = np.array(bqm_vars_ser)
        vars_pos = {var: i for i, var in enumerate(vars)}

        linears = []
        linear_indices = []
        for var, val in bqm.linear.items():
            linears.append(val)
            linear_indices.append(vars_pos[var])
        quads = []
        quad_row = []
        quad_col = []
        for (var1, var2), val in bqm.quadratic.items():
            quads.append(val)
            quad_row.append(vars_pos[var1])
            quad_col.append(vars_pos[var2])

        vartype = Vtype(bqm.vartype.name.title())
        offset = float(bqm.offset)
        return Model._from_pym(
            PyBqmTranslator.to_lm(
                vars=vars,
                vtype=vartype._val,
                offset=offset,
                linears=np.array(linears, dtype=np.float64),
                linear_indices=np.array(linear_indices, dtype=np.uint64),
                quads=np.array(quads, dtype=np.float64),
                quads_rows=np.array(quad_row, dtype=np.uint64),
                quads_cols=np.array(quad_col, dtype=np.uint64),
                name=name,
            )
        )

    @staticmethod
    def from_lm(model: Model) -> BinaryQuadraticModel:
        offset, linear, quad, rows, cols, vtype, variables = PyBqmTranslator.from_lm(
            model._m
        )
        vtype = Vtype._from_pyvtype(vtype).value.upper()
        return BinaryQuadraticModel.from_numpy_vectors(
            linear,
            (rows, cols, quad),
            offset,
            vtype,
            variable_order=variables,
        )
