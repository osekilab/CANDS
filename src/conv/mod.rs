use crate::prelude::*;



pub fn converges_ci(deriv: &[Stage]) -> bool {
    if let Some(stage) = deriv.last() {
        if stage.w.0.len() == 1 {
            if let SyntacticObject::Transfer { .. } = stage.w.0.first().unwrap() {
                return true;
            }
        }
    }

    false
}



pub fn converges_sm(deriv: &[Stage]) -> bool {
    if let Some(stage) = deriv.last() {
        if stage.w.0.len() == 1 {
            if let SyntacticObject::Transfer { .. } = stage.w.0.first().unwrap() {
                return true;
            }
        }
    }

    false
}



pub fn converges(deriv: &[Stage]) -> bool {
    converges_ci(deriv) && converges_sm(deriv)
}