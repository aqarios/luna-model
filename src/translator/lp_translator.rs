use strum_macros::Display;

use crate::core::{
    expression::{BiasConstraints, IndexConstraints},
    Model,
};

use super::base::{BackTranslator, Translator};
use std::{fs::File, io::Read, marker::PhantomData, path::PathBuf};

#[derive(Debug, Copy, Clone, PartialEq, Display)]
enum Section {
    /// Single-Objective Case
    ///
    /// Let us consider single-objective models first, where this header is followed by
    /// a single linear or quadratic expression that captures the objective function.
    ///
    /// The objective optionally begins with a label. A label consists of a name,
    /// followed by a colon character, following by a space. A space is allowed between
    /// the name and the colon, but not required.
    ///
    /// The objective then continues with a list of linear terms, separated by the + or
    /// - operators. A term can contain a coefficient and a variable (e.g., 4.5 x), or
    /// just a variable (e.g., x). The objective can be spread over many lines, or it
    /// may be listed on a single line. Line breaks can come between tokens, but never
    /// within tokens.
    ///
    /// The objective may optionally continue with a list of quadratic terms. The
    /// quadratic portion of the objective expression begins with a [ symbol and ends
    /// with a ] symbol, followed by / 2. These brackets should enclose one or more
    /// quadratic terms. Either squared terms (e.g., 2 x ^ 2) or product terms
    /// (e.g., 3 x * y) are accepted. Coefficients on the quadratic terms are optional.
    ///
    /// For variables with piecewise-linear objective functions, the objective section
    /// will include a __pwl(x) term, where x is the name of the variable. The actual
    /// piecewise-linear expressions are pulled from the later PWLObj section.
    ///
    /// The objective expression must always end with a line break.
    ///
    /// An objective section might look like the following:
    ///
    /// Minimize
    ///   obj: 3.1 x + 4.5 y + 10 z + [ x ^ 2 + 2 x * y + 3 y ^ 2 ] / 2
    ///
    Objective,
    // /// NOT SUPPORTED RIGHT NOW.
    // MultiObjective,
    /// The next section is the constraints section. It begins with one of the following
    /// headers, on its own line: subject to, such that, st, or s.t..
    /// Capitalization is ignored.
    ///
    /// The constraint section can have an arbitrary number of constraints. Each
    /// constraint starts with an optional label (constraint name, followed by a colon,
    /// followed by a space), continues with a linear expression, followed by an optional
    /// quadratic expression (enclosed in square brackets), and ends with a comparison
    /// operator, followed by a numerical value, followed by a line break. Valid comparison
    /// operators are =, <=, <, >=, or >. Note that LP format does not distinguish between
    /// strict and non-strict inequalities, so for example < and <= are equivalent.
    ///
    /// Note that the left-hand side of a constraint may not contain a constant term;
    /// the constant must appear on the right-hand side.
    ///
    /// The following is a simple example of a valid linear constraint:
    ///
    /// c0: 2.5 x + 2.3 y + 5.3 z <= 8.1
    ///
    /// The following is a valid quadratic constraint:
    ///
    /// qc0: 3.1 x + 4.5 y + 10 z + [ x ^ 2 + 2 x * y + 3 y ^ 2 ] <= 10
    ///
    /// The constraint section may also contain another constraint type: the so-called
    /// indicator constraint. Indicator constraints start with an optional label
    /// (constraint name, followed by a colon, followed by a space), followed by a
    /// binary variable, a space, a =, again a space and a value, either 0 or 1.
    /// They continue with a space, followed by ->, and again a space and finally a
    /// linear constraint (without a label).
    ///
    /// For example:
    ///
    /// c0: b1 = 1 -> 2.5 x + 2.3 y + 5.3 z <= 8.1
    ///
    /// This example constraint requires the given linear constraint to be satisfied if
    /// the variable b1 takes a value of 1.
    ///
    /// Every LP format file must have a constraints section.
    Constraints,
    // /// NOT SUPPORTED RIGHT NOW.
    // LazyConstraints,
    // /// NOT SUPPORTED RIGHT NOW.
    // UserCuts,
    /// Bounds Section
    /// The next section is the bounds section. It begins with the word Bounds, on its
    /// own line, and is followed by a list of variable bounds. Each line specifies the
    /// lower bound, the upper bound, or both for a single variable. The keywords inf or
    /// infinity can be used in the bounds section to specify infinite bounds. A bound
    /// line can also indicate that a variable is free, meaning that it is unbounded in
    /// either direction.
    ///
    /// Here are examples of valid bound lines:
    ///
    /// Bounds
    ///   0 <= x0 <= 1
    ///   x1 <= 1.2
    ///   x2 >= 3
    ///   x3 free
    ///   x2 >= -Inf
    ///
    /// It is not necessary to specify bounds for all variables; by default, each
    /// variable has a lower bound of 0 and an infinite upper bound. In fact, the entire
    /// bounds section is optional.
    Bounds,
    /// Variable Type Section
    ///
    /// The next section is the variable types section. Variables can be designated as
    /// being either binary, general integer, or semi-continuous. In all cases, the
    /// designation is applied by first providing the appropriate header (on its own line),
    /// and then listing the variables that have the associated type. For example:
    ///
    /// Binary
    ///   x y z
    ///
    /// Variable type designations don’t need to appear in any particular order
    /// (e.g., general integers can either precede or follow binaries). If a variable is
    /// included in multiple sections, the last one determines the variable type.
    ///
    /// Valid keywords for variable type headers are: binary, binaries, bin, general,
    /// generals, gen, semi-continuous, semis, or semi.
    ///
    /// The variable types section is optional. By default, variables are assumed to be
    /// continuous.
    VariableType,
    // /// SOS Section (NOT SUPPORTED)
    // /// An LP file can contain a section that captures SOS constraints of type 1 or
    // /// type 2. The SOS section begins with the SOS header on its own line
    // /// (capitalization isn’t important). An arbitrary number of SOS constraints can
    // /// follow. An SOS constraint starts with a name, followed by a colon
    // /// (unlike linear constraints, the name is not optional here). Next comes the SOS
    // /// type, which can be either S1 or S2. The type is followed by a pair of colons.
    // ///
    // /// Next come the members of the SOS set, along with their weights. Each member is
    // /// captured using the variable name, followed by a colon, followed by the associated
    // /// weight. Spaces can optionally be placed before and after the colon.
    // /// An SOS constraint must end with a line break.
    // ///
    // /// Here’s an example of an SOS section containing two SOS constraints:
    // ///
    // /// SOS
    // ///   sos1: S1 :: x1 : 1  x2 : 2  x3 : 3
    // ///   sos2: S2 :: x4:8.5  x5:10.2  x6:18.3
    // ///
    // /// The SOS section is optional.
    // SOS,
    /// End Statement
    /// The last line in an LP format file should be an `End` statement.
    End,
}

#[derive(Display)]
enum ObjectiveKeywords {
    #[strum(to_string = "minimize")]
    Minimize,
    #[strum(to_string = "maximize")]
    Maximize,
    #[strum(to_string = "minimum")]
    Minimum,
    #[strum(to_string = "maximum")]
    Maximum,
    #[strum(to_string = "min")]
    Min,
    #[strum(to_string = "max")]
    Max,
}

pub struct LPTranslator<Index, Bias> {
    _phantom_index: PhantomData<Index>,
    _phantom_bias: PhantomData<Bias>,
}

impl<Index, Bias> Translator for LPTranslator<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type TranslateIn = PathBuf;
    type TranslateOut = Model<Index, Bias>;

    fn translate(filepath: Self::TranslateIn) -> Self::TranslateOut {
        let display = filepath.display();
        let mut file = match File::open(&filepath) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}", display, s),
        }
        // println!("In LP comment is marked with: {}", Section::Comment);
        todo!()
    }
}

impl<Index, Bias> BackTranslator for LPTranslator<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn back_translate(data: Self::TranslateOut) -> Self::TranslateIn {
        todo!()
    }
}
