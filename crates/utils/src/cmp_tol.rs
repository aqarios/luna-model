pub fn float_eq(lhs: f64, rhs: f64, tol: f64) -> bool {
    if lhs == rhs {
        return true;
    }
    if !lhs.is_finite() || !rhs.is_finite() {
        return false;
    }

    (lhs - rhs).abs() <= comparison_tolerance(lhs, rhs, tol)
}

pub fn float_le(lhs: f64, rhs: f64, tol: f64) -> bool {
    if lhs <= rhs {
        return true;
    }
    if !lhs.is_finite() || !rhs.is_finite() {
        return false;
    }

    lhs - rhs <= comparison_tolerance(lhs, rhs, tol)
}

pub fn float_ge(lhs: f64, rhs: f64, tol: f64) -> bool {
    if lhs >= rhs {
        return true;
    }
    if !lhs.is_finite() || !rhs.is_finite() {
        return false;
    }

    rhs - lhs <= comparison_tolerance(lhs, rhs, tol)
}

fn comparison_tolerance(lhs: f64, rhs: f64, tol: f64) -> f64 {
    tol + f64::EPSILON * lhs.abs().max(rhs.abs()).max(1.0)
}
