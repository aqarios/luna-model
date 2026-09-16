use crate::{
    Expression,
    expression::term::{HigherOrder, Linear, Quadratic},
    variable::VarRef,
};

impl Expression {
    pub fn derivative(&self, var: &VarRef) -> Self {
        let mut offset: f64 = 0.0;
        for (k, v) in self.linear_items() {
            if k.id() == var.id() {
                offset += v;
            }
        }

        let mut linear: Linear = Linear::default();
        for (k1, k2, v) in self.quadratic_items() {
            if k1.id() == var.id() || k2.id() == var.id() {
                let k = if k1.id() == var.id() { k2 } else { k1 };
                linear += (k.id(), v);
            }
        }

        let (quadratic, higher_order) = {
            let mut quadratic: Quadratic = Quadratic::default();
            let mut higher_order: HigherOrder = HigherOrder::default();
            for (ks, v) in self.higher_order_items() {
                if let Some((i, _)) = ks.iter().enumerate().find(|(_, x)| x.id() == var.id()) {
                    let idxs: Vec<u32> = ks
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .map(|(_, x)| x.id())
                        .collect();
                    if idxs.len() == 2 {
                        quadratic += (idxs[0], idxs[1], v);
                    } else {
                        higher_order += (idxs, v);
                    }
                }
            }

            (
                if !quadratic.is_empty() {
                    Some(quadratic)
                } else {
                    None
                },
                if !higher_order.is_empty() {
                    Some(higher_order)
                } else {
                    None
                },
            )
        };

        Expression {
            env: self.env.clone(),
            offset,
            linear,
            quadratic,
            higher_order,
        }
    }
}
