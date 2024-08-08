use crate::{language::PropLang, z3utils};
use crate::cesar::base_pass::BasePass;
use egg::*;

/// This pass checks if there are redundant conjuncts.
pub struct Pass6;

pub static mut ASSUMPTIONS: String =  String::new();

fn var(s: &str) -> Var {
    s.parse().unwrap()
}

impl BasePass for Pass6 {

    // reference: https://docs.rs/egg/latest/egg/macro.rewrite.html.
    fn make_rules() -> Vec<Rewrite<PropLang, ()>> {

        // Return true if (assumptions and (AND_i a_i)) -> (b).
        fn implies_lst(var_ante: Vec<Var>, var_b: Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            move |egraph, _, subst| {
                let antes = var_ante.iter().map(|v| subst[*v]).collect::<Vec<Id>>();
                let b = subst[var_b];
                let extractor = Extractor::new(&egraph, AstSize);
                let ante_fml = antes.iter()
                    .map(|a| extractor.find_best(*a).1)
                    .map(|f| f.to_string());
                let ante_fml_str = ante_fml.fold("true".to_string(),
                    |acc, f| format!("(and {} {})", acc, f)).to_string();
                let b_fml = extractor.find_best(b).1.to_string();
                let assumptions = unsafe { ASSUMPTIONS.clone() };
                z3utils::imply(format!("(and {} {})", ante_fml_str, assumptions), b_fml)
            }
        }

        
        vec![
            // Logic rules.
            rewrite!("and-id"; "(and ?a true)" => "?a"),
            rewrite!("or-id"; "(or ?a false)" => "?a"),
            rewrite!("and-collapse"; "(and ?a false)" => "false"),
            rewrite!("or-collapse"; "(or ?a true)" => "true"),
            ////
            //// These rules are expensive but potentially useful.
            // rewrite!("or-comm"; "(or ?a ?b)" => "(or ?b ?a)"),
            rewrite!("and-comm"; "(and ?a ?b)" => "(and ?b ?a)"),
            // rewrite!("and-assoc"; "(and ?a (& ?b ?c))" => "(and (& ?a ?b) ?c)"),
            // rewrite!("or-assoc"; "(or ?a (or ?b ?c))" => "(or (or ?a ?b) ?c)"),
            //
            // rewrite!("and-distrib"; "(and ?a (or ?b ?c))" => "(or (and ?a ?b) (and ?a ?c))"),
            // rewrite!("or-distrib"; "(or ?a (and ?b ?c))" => "(and (or ?a ?b) (or ?a ?c))"),
            // rewrite!("and-factor"; "(or (and ?a ?b) (and ?a ?c))" => "(and ?a (or ?b ?c))"),
            // rewrite!("or-factor"; "(and (or ?a ?b) (or ?a ?c))" => "(or ?a (and ?b ?c))"),
            // If a formula is implied by the assumptions, replace it with true.
            rewrite!("redundant-conjunct-1";"(and ?a ?b)" => "?a"
                if implies_lst(vec![var("?a")], var("?b"))),
            rewrite!("redundant-conjunct-2";"(and ?c (and ?a ?b))" => "(and ?c ?a)"
                if implies_lst(vec![var("?c"), var("?a")], var("?b"))),
            rewrite!("redundant-conjunct-3";"(and ?c (and ?d (and ?a ?b)))" => "(and ?c (and ?d ?a))"
                if implies_lst(vec![var("?c"), var("?d"), var("?a")], var("?b"))),
            // If not(a) is implied by the assumptions, replace it with false.
            // rewrite!("assumption-false";"?a" => "false"
            //     if neg_implied_by_assumptions()),
        ]
    }
}
