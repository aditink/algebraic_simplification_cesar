use crate::cesar::language::PropLang;
use crate::cesar::simplification::Simplification;
use egg::*;

/// This pass does multiplication distribution, i.e. the transformation x*a + x*b = x*(a+b).
pub struct Pass7;

pub static mut ASSUMPTIONS: String =  String::new();

impl Simplification for Pass7 {

    // reference: https://docs.rs/egg/latest/egg/macro.rewrite.html.
    fn make_rules() -> Vec<Rewrite<PropLang, ()>> {
        vec![
            // Multiplication distribution.
            rewrite!("mul-dist"; "(+ (* ?x ?y) (* ?x ?z))" => "(* ?x (+ ?y ?z))"),
            rewrite!("mul-dist-minus"; "(- (* ?x ?y) (* ?x ?z))" => "(* ?x (- ?y ?z))"),
            // Multiplication commutes.
            rewrite!("mul-comm"; "(* ?x ?y)" => "(* ?y ?x)"),
            // Multiplication associates.
            rewrite!("mul-assoc"; "(* ?x (* ?y ?z))" => "(* (* ?x ?y) ?z)"),
            // x+(-y) is just x-y.
            rewrite!("minus-rewrite"; "(+ (- ?x) ?y)" => "(- ?y ?x)"),
            rewrite!("minus-rewrite-2"; "(+ ?x (- ?y))" => "(- ?x ?y)"),
            // Get minus outside multiplication.
            rewrite!("minus-distribute"; "(* (- ?x) ?y)" => "(- (* ?y ?x))"),
        ]
    }
}
