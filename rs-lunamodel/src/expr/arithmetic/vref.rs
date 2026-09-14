use std::ops::{Add, Mul, Sub};

use crate::{
    Expression, Result, VarRef,
    traits::{TryAddAssign, TryMulAssign, TrySubAssign},
};

// Add (with failable)

impl Add<VarRef> for Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: VarRef) -> Self::Output {
        out!(expr_add_vref(@self, @rhs))
    }
}

impl Add<&VarRef> for Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: &VarRef) -> Self::Output {
        out!(expr_add_vref(@self, @rhs))
    }
}

impl Add<VarRef> for &Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: VarRef) -> Self::Output {
        out!(expr_add_vref(@self, @rhs))
    }
}

impl Add<&VarRef> for &Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: &VarRef) -> Self::Output {
        out!(expr_add_vref(@self, @rhs))
    }
}

impl Add<VarRef> for Result<Expression> {
    type Output = Result<Expression>;

    fn add(self, rhs: VarRef) -> Self::Output {
        let slf = self?;
        out!(expr_add_vref(@slf, @rhs))
    }
}

impl Add<&VarRef> for Result<Expression> {
    type Output = Result<Expression>;

    fn add(self, rhs: &VarRef) -> Self::Output {
        let slf = self?;
        out!(expr_add_vref(@slf, @rhs))
    }
}

impl Add<Result<VarRef>> for Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(expr_add_vref(@self, @rhs))
    }
}

impl Add<Result<VarRef>> for &Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(expr_add_vref(@self, @rhs))
    }
}

/* `vref + expr` commutes to `expr + vref`. */

impl Add<Expression> for VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: Expression) -> Self::Output {
        rhs + self
    }
}

impl Add<&Expression> for VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: &Expression) -> Self::Output {
        rhs + self
    }
}

impl Add<Expression> for &VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: Expression) -> Self::Output {
        rhs + self
    }
}

impl Add<&Expression> for &VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: &Expression) -> Self::Output {
        rhs + self
    }
}

impl Add<Result<Expression>> for VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: Result<Expression>) -> Self::Output {
        rhs? + self
    }
}

impl Add<Result<Expression>> for &VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: Result<Expression>) -> Self::Output {
        rhs? + self
    }
}

impl Add<Expression> for Result<VarRef> {
    type Output = Result<Expression>;

    fn add(self, rhs: Expression) -> Self::Output {
        rhs + self?
    }
}

impl Add<&Expression> for Result<VarRef> {
    type Output = Result<Expression>;

    fn add(self, rhs: &Expression) -> Self::Output {
        rhs + self?
    }
}

// Sub (with failable)

impl Sub<VarRef> for Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        out!(expr_sub_vref(@self, @rhs))
    }
}

impl Sub<&VarRef> for Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        out!(expr_sub_vref(@self, @rhs))
    }
}

impl Sub<VarRef> for &Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        out!(expr_sub_vref(@self, @rhs))
    }
}

impl Sub<&VarRef> for &Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        out!(expr_sub_vref(@self, @rhs))
    }
}

impl Sub<VarRef> for Result<Expression> {
    type Output = Result<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        let slf = self?;
        out!(expr_sub_vref(@slf, @rhs))
    }
}

impl Sub<&VarRef> for Result<Expression> {
    type Output = Result<Expression>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        let slf = self?;
        out!(expr_sub_vref(@slf, @rhs))
    }
}

impl Sub<Result<VarRef>> for Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(expr_sub_vref(@self, @rhs))
    }
}

impl Sub<Result<VarRef>> for &Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(expr_sub_vref(@self, @rhs))
    }
}

impl Sub<Result<Expression>> for VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: Result<Expression>) -> Self::Output {
        self.sub(rhs?)
    }
}

impl Sub<Result<Expression>> for &VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: Result<Expression>) -> Self::Output {
        self.sub(rhs?)
    }
}

impl Sub<Expression> for Result<VarRef> {
    type Output = Result<Expression>;

    fn sub(self, rhs: Expression) -> Self::Output {
        self?.sub(rhs)
    }
}

impl Sub<&Expression> for Result<VarRef> {
    type Output = Result<Expression>;

    fn sub(self, rhs: &Expression) -> Self::Output {
        self?.sub(rhs)
    }
}

// Mul (with failable)

impl Mul<VarRef> for Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: VarRef) -> Self::Output {
        out!(expr_mul_vref(@self, @rhs))
    }
}

impl Mul<&VarRef> for Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        out!(expr_mul_vref(@self, @rhs))
    }
}

impl Mul<VarRef> for &Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: VarRef) -> Self::Output {
        out!(expr_mul_vref(@self, @rhs))
    }
}

impl Mul<&VarRef> for &Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        out!(expr_mul_vref(@self, @rhs))
    }
}

impl Mul<VarRef> for Result<Expression> {
    type Output = Result<Expression>;

    fn mul(self, rhs: VarRef) -> Self::Output {
        let slf = self?;
        out!(expr_mul_vref(@slf, @rhs))
    }
}

impl Mul<&VarRef> for Result<Expression> {
    type Output = Result<Expression>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        let slf = self?;
        out!(expr_mul_vref(@slf, @rhs))
    }
}

impl Mul<Result<VarRef>> for Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(expr_mul_vref(@self, @rhs))
    }
}

impl Mul<Result<VarRef>> for &Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(expr_mul_vref(@self, @rhs))
    }
}

/* `vref * expr` commutes to `expr * vref`. */

impl Mul<Expression> for VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: Expression) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Expression> for VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: &Expression) -> Self::Output {
        rhs * self
    }
}

impl Mul<Expression> for &VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: Expression) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Expression> for &VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: &Expression) -> Self::Output {
        rhs * self
    }
}

impl Mul<Result<Expression>> for VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: Result<Expression>) -> Self::Output {
        rhs? * self
    }
}

impl Mul<Result<Expression>> for &VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: Result<Expression>) -> Self::Output {
        rhs? * self
    }
}

impl Mul<Expression> for Result<VarRef> {
    type Output = Result<Expression>;

    fn mul(self, rhs: Expression) -> Self::Output {
        rhs * self?
    }
}

impl Mul<&Expression> for Result<VarRef> {
    type Output = Result<Expression>;

    fn mul(self, rhs: &Expression) -> Self::Output {
        rhs * self?
    }
}

// I-Add (with failable)

impl TryAddAssign<VarRef> for Expression {
    fn try_add_assign(&mut self, rhs: VarRef) -> Result<()> {
        check!(expr_iadd_vref(@mut self, @rhs))
    }
}

impl TryAddAssign<&VarRef> for Expression {
    fn try_add_assign(&mut self, rhs: &VarRef) -> Result<()> {
        check!(expr_iadd_vref(@mut self, @rhs))
    }
}

impl TryAddAssign<Result<VarRef>> for Expression {
    fn try_add_assign(&mut self, rhs: Result<VarRef>) -> Result<()> {
        let rhs = rhs?;
        check!(expr_iadd_vref(@mut self, @rhs))
    }
}

// I-Sub (with failable)

impl TrySubAssign<VarRef> for Expression {
    fn try_sub_assign(&mut self, rhs: VarRef) -> Result<()> {
        check!(expr_isub_vref(@mut self, @rhs))
    }
}

impl TrySubAssign<&VarRef> for Expression {
    fn try_sub_assign(&mut self, rhs: &VarRef) -> Result<()> {
        check!(expr_isub_vref(@mut self, @rhs))
    }
}

impl TrySubAssign<Result<VarRef>> for Expression {
    fn try_sub_assign(&mut self, rhs: Result<VarRef>) -> Result<()> {
        let rhs = rhs?;
        check!(expr_isub_vref(@mut self, @rhs))
    }
}

// I-Mul (with failable)

impl TryMulAssign<VarRef> for Expression {
    fn try_mul_assign(&mut self, rhs: VarRef) -> Result<()> {
        check!(expr_imul_vref(@mut self, @rhs))
    }
}

impl TryMulAssign<&VarRef> for Expression {
    fn try_mul_assign(&mut self, rhs: &VarRef) -> Result<()> {
        check!(expr_imul_vref(@mut self, @rhs))
    }
}

impl TryMulAssign<Result<VarRef>> for Expression {
    fn try_mul_assign(&mut self, rhs: Result<VarRef>) -> Result<()> {
        let rhs = rhs?;
        check!(expr_imul_vref(@mut self, @rhs))
    }
}
