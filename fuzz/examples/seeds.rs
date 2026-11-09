//! Writes the starting inputs of every target into `seeds/<target>/`.

use std::fs;

use itertools::Either;
use lunamodel_core::{Expression, Solution};
use lunamodel_fuzz::expression_env;
use lunamodel_serializer::prelude::{Creatable, Encodable};
use lunamodel_translate::model::{LpTranslator, MpsTranslator};

/// Each LP model also seeds the MPS reader through the MPS writer, and the
/// decoders through the encoders.
const LP: &[&str] = &[
    "\\ every section\nMaximize\n obj: 2 x + 3 y - z + [ x * y + 2 z ^ 2 ] / 2 + 5\n\
     Subject To\n c1: x + y + z <= 10\n c2: x - y >= -2\n q: [ x * y ] + z <= 4\n eq: y + z = 3\n\
     Bounds\n x free\n -5 <= y <= 5\n z <= -1\n 0 <= n <= 1\n\
     Generals\n n i\nBinaries\n b\nEnd\n",
    "Minimize\n x.1 + 2 y(0) + 3 x[0,0] + 1.5e-3 w\nst\n c: x.1 + y(0) >= 1\n -1 x[0,0] =< 2.5E+2\n\
     Bounds\n 3 >= w\n x.1 >= -inf\nGeneral\n w\nend\n",
];

/// MPS the writer never produces: fixed format, RANGES, every quadratic section.
const MPS: &[&str] = &[
    "NAME          FIXED\nROWS\n N  COST\n L  LIM1\n G  LIM2\nCOLUMNS\n    \
     X1        COST         1.0   LIM1         1.0\n    X2        COST         2.0   LIM2         1.0\n\
     RHS\n    RHS       LIM1         4.0   LIM2         1.0\nBOUNDS\n UP BND       X1           4.0\nENDATA\n",
    "NAME t\nOBJSENSE\n    MAX\nROWS\n N obj\n E e\n L l\n G g\nCOLUMNS\n m 'MARKER' 'INTORG'\n x obj 1 e 1\n \
     m 'MARKER' 'INTEND'\n y l 1 g 1\nRHS\n rhs e 2 l 3\n rhs g 1 obj 4\nRANGES\n rng e -1 l 2\n rng g 3\n\
     BOUNDS\n UP bnd y -1\n MI bnd x\nENDATA\n",
    "NAME q\nROWS\n N obj\n L c\nCOLUMNS\n x obj 1 c 1\n y obj 1 c 1\nRHS\n rhs c 1\n\
     QUADOBJ\n x x 2\n x y 1\nQCMATRIX c\n x y 0.5\n y x 0.5\nENDATA\n",
    "NAME q\nROWS\n N obj\nCOLUMNS\n x obj 1\n y obj 1\nQMATRIX\n x x 2\n x y 1\n y x 1\nENDATA\n",
    "NAME q\nROWS\n N obj\nCOLUMNS\n x obj 1\n y obj 1\nQSECTION obj\n x x 2\n x y 1\nENDATA\n",
];

fn main() {
    for (i, lp) in LP.iter().enumerate() {
        let model = LpTranslator::translate(lp.to_string()).unwrap();
        let written_lp = LpTranslator::back_translate(&model, None).unwrap().unwrap();
        let written_mps = MpsTranslator::back_translate(&model, None).unwrap().unwrap();
        write("lp_reader", &format!("{i}"), lp);
        write("lp_reader", &format!("{i}_written"), &written_lp);
        write("mps_reader", &format!("{i}_written"), &written_mps);

        let env = model.environment.read_arc().clone();
        for compress in [false, true] {
            let name = format!("{i}_{compress}");
            write("decode_model", &name, encode(&model, compress));
            write("decode_environment", &name, encode(&env, compress));
        }
    }

    // Random samples need bounded variables.
    let bounded = "Minimize\n x + 2 y - 3 i\nSubject To\n c: x + y + i <= 2\n\
                   Bounds\n 0 <= x <= 1.5\n -2 <= i <= 2\nGenerals\n i\nBinaries\n y\nEnd\n";
    let model = LpTranslator::translate(bounded.to_string()).unwrap();
    let solution = Solution::from_random(4, Some(0), Either::Right(model), None, None).unwrap();
    for compress in [false, true] {
        write("decode_solution", &format!("{compress}"), encode(&solution, compress));
    }

    let dense = [1.0, -2.0, 0.0, 0.5, 0.0, 3.0, 1.5, 0.0, 0.0, 0.0, -1.0, 2.0, 0.0, 0.0, 0.0, 4.0];
    let expr = Expression::from_dense_quadratic(&dense, 4, Some(7.0), expression_env()).unwrap();
    for compress in [false, true] {
        write("decode_expression", &format!("{compress}"), encode(&expr, compress));
    }
    for (i, mps) in MPS.iter().enumerate() {
        MpsTranslator::translate(mps.to_string()).unwrap();
        write("mps_reader", &format!("{i}"), mps);
    }
}

fn encode<T: Encodable<S>, S: Creatable<T>>(value: &T, compress: bool) -> Vec<u8> {
    value.encode(Some(compress), None).unwrap()
}

fn write(target: &str, name: &str, content: impl AsRef<[u8]>) {
    let dir = format!("seeds/{target}");
    fs::create_dir_all(&dir).unwrap();
    fs::write(format!("{dir}/{name}"), content).unwrap();
}
