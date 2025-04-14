import os
import pathlib
import pytest
import tempfile
import gurobipy as gp
import cplex

from random import Random
from aqmodels import LpTranslator
from dimod import lp as dimod_lp
from pytests.test_core.utils import generate_cqms, make_seed

@pytest.mark.translator
def test_cqm_to_model_to_cqm():
    rand = Random(make_seed())
    cqms = generate_cqms(20, rand)
    for cqm in cqms:
        cqm_lp = dimod_lp.dumps(cqm)
        model = LpTranslator.to_model(cqm_lp)
        cqm_lp_back = LpTranslator.from_model(model)
        cqm_back = dimod_lp.loads(cqm_lp_back)
        # this is ugly, but the constraints are weird in dimod.
        # E.g. this: -29 * x_0 + 12 * x_2 <= 0.0
        # will be translated to: x_0 + 12 * x_2 <= -29
        # in the LP file and thus also in the cqm_back.
        # Therefore we need to compare the strings.
        cqm_back_str_lp = dimod_lp.dumps(cqm_back)
        assert cqm_lp == cqm_back_str_lp

@pytest.mark.translator
def test_gurobi_to_model_to_gurobi():
    # Make sure Gurobi does not print to console
    rand = Random(make_seed())
    cqms = generate_cqms(20, rand)
    for cqm in cqms:
        # We use CQM's assuming the LP file is correctly formatted for Gurobi.
        # SETUP
        with tempfile.NamedTemporaryFile(mode="w+", suffix=".lp") as tf:
            dimod_lp.dump(cqm, tf.file) # type: ignore
            tf.flush()
            tf.seek(0)
            gp_model = gp.read(tf.name)

        # ACTUAL
        with tempfile.NamedTemporaryFile(mode="w+", suffix=".lp") as tf:
            gp_model.write(tf.name)
            tf.flush()
            tf.seek(0)
            lp_str = LpTranslator.from_model(LpTranslator.to_model(tf.file.read()))

        with tempfile.NamedTemporaryFile(mode="w+", suffix=".lp") as tf:
            tf.write(lp_str)
            tf.flush()
            tf.seek(0)
            gp_model_back = gp.read(tf.name)

        with tempfile.NamedTemporaryFile(mode="w+", suffix=".mps") as tf:
            gp_model.write(tf.name)
            tf.flush()
            tf.seek(0)
            gp_model_mps = tf.read()

        with tempfile.NamedTemporaryFile(mode="w+", suffix=".mps") as tf:
            gp_model_back.write(tf.name)
            tf.flush()
            tf.seek(0)
            gp_model_back_mps = tf.read()

        assert gp_model_mps == gp_model_back_mps

@pytest.mark.translator
def test_cplex_to_model_to_cplex():
    rand = Random(make_seed())
    cqms = generate_cqms(20, rand)
    for cqm in cqms:
        # We use CQM's assuming the LP file is correctly formatted for Gurobi.
        # SETUP
        tmp_lp = tempfile.NamedTemporaryFile(mode="w+", suffix=".lp")
        tmp_mps = tempfile.NamedTemporaryFile(mode="w+", suffix=".mps")
        dimod_lp.dump(cqm, tmp_lp.file) 
        tmp_lp.flush()
        tmp_lp.seek(0)
        cpx_model = cplex.Cplex(tmp_lp.name)
        cpx_model.set_log_stream(None)
        # ACTUAL
        # build cplex base model (ground truth)
        tmp_lp.seek(0)
        cpx_model.write(tmp_lp.name)
        # store the mps file string from the base
        cpx_model.write(tmp_mps.name)
        tmp_mps.seek(0)
        cpx_mps_str = tmp_mps.read()
        # build aqmodel
        tmp_lp.seek(0)
        aqmodel = LpTranslator.to_model(tmp_lp.file.read())
        lp_str = LpTranslator.from_model(aqmodel)
        # write to lp file
        tmp_lp.seek(0)
        tmp_lp.write(lp_str)
        tmp_lp.seek(0)
        # build cplex model back
        cpx_model_back = cplex.Cplex(tmp_lp.name)
        # store the mps file string from the back
        cpx_model_back.write(tmp_mps.name)
        tmp_mps.seek(0)
        cpx_back_mps_str = tmp_mps.read()
        # compare the two MPS strings
        assert cpx_mps_str == cpx_back_mps_str
