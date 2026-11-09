//! Writes the starting inputs of every target into `seeds/<target>/`.

use std::fs;

use lunamodel_translate::model::{LpTranslator, MpsTranslator};

/// Each LP model also seeds the MPS reader, through the MPS writer.
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
    }
    for (i, mps) in MPS.iter().enumerate() {
        MpsTranslator::translate(mps.to_string()).unwrap();
        write("mps_reader", &format!("{i}"), mps);
    }
}

fn write(target: &str, name: &str, content: impl AsRef<[u8]>) {
    let dir = format!("seeds/{target}");
    fs::create_dir_all(&dir).unwrap();
    fs::write(format!("{dir}/{name}"), content).unwrap();
}
