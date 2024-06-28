use crate::cesar::language::PropLang;
use crate::cesar::simplification::Simplification;
use egg::*;

/// A function to clean up bad things like 0<0.
pub struct Pass5;

pub static mut ASSUMPTIONS: String =  String::new();

impl Simplification for Pass5 {

    // reference: https://docs.rs/egg/latest/egg/macro.rewrite.html.
    fn make_rules() -> Vec<Rewrite<PropLang, ()>> {

        vec![
            // Logic and math rules.
            rewrite!("and-id"; "(and ?a true)" => "?a"),
            rewrite!("or-id"; "(or ?a false)" => "?a"),
            rewrite!("and-collapse"; "(and ?a false)" => "false"),
            rewrite!("or-collapse"; "(or ?a true)" => "true"),
            //// Commuted version of the above rules.
            rewrite!("and-id-comm"; "(and true ?a)" => "?a"),
            rewrite!("or-id-comm"; "(or false ?a)" => "?a"),
            rewrite!("and-collapse-comm"; "(and false ?a)" => "false"),
            rewrite!("or-collapse-comm"; "(or true ?a)" => "true"),
            ////
            rewrite!("lt-false"; "(< ?x ?x)" => "false"),
            rewrite!("gt-false"; "(> ?x ?x)" => "false"),
            rewrite!("eq-true"; "(= ?x ?x)" => "true"),
            rewrite!("leq-true"; "(<= ?x ?x)" => "true"),
            rewrite!("geq-true"; "(>= ?x ?x)" => "true"),
            // Arithmetic around 0.
            rewrite!("add-zero"; "(+ ?a 0)" => "?a"),
            rewrite!("sub-zero"; "(- ?a 0)" => "?a"),
            rewrite!("mul-zero"; "(* ?a 0)" => "0"),
            // Commuted version of the above rules.
            rewrite!("add-zero-comm"; "(+ 0 ?a)" => "?a"),
            rewrite!("sub-zero-comm"; "(- 0 ?a)" => "?a"),
            rewrite!("mul-zero-comm"; "(* 0 ?a)" => "0"),
            // Times -1.
            rewrite!("mul-neg-one"; "(+ (* ?a -1) ?b)" => "(- ?b ?a)"),
            rewrite!("mul-neg-one-comm"; "(+ ?b (* ?a -1))" => "(- ?b ?a)"),
            // Commuted version of the above rules.
            rewrite!("mul-neg-one-2"; "(+ (* -1 ?a) ?b)" => "(- ?b ?a)"),
            rewrite!("mul-neg-one-2-comm"; "(+ ?b (* -1 ?a))" => "(- ?b ?a)"),
        ]
    }
}
