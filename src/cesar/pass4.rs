use crate::cesar::base;
use crate::cesar::config;
use crate::cesar::{language::PropLang, z3utils};
use egg::*;

pub struct Pass4;

pub static mut ASSUMPTIONS: String = String::new();

fn var(s: &str) -> Var {
    s.parse().unwrap()
}

/// This pass performs aggressive nested or redundancy elimination.
impl Pass4 {
    // reference: https://docs.rs/egg/latest/egg/macro.rewrite.html.
    fn make_rules() -> Vec<Rewrite<PropLang, ()>> {
        // Return true if (assumptions and a) -> (or b c).
        fn redundant_disjunct(
            var_a: Var,
            var_b: Var,
            var_c: Var,
        ) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            move |egraph, _, subst| {
                let a = subst[var_a];
                let b = subst[var_b];
                let c = subst[var_c];
                let extractor = Extractor::new(&egraph, AstSize);
                let a_fml = extractor.find_best(a).1.to_string();
                let b_fml = extractor.find_best(b).1.to_string();
                let c_trm = extractor.find_best(c).1.to_string();
                let assumptions = unsafe { ASSUMPTIONS.clone() };
                z3utils::imply(
                    format!("(and {} {})", a_fml, assumptions),
                    format!("(or {} {})", b_fml, c_trm),
                )
            }
        }

        // Return true if (assumptions and (AND_i a_i)) -> (b).
        fn implies_lst(
            var_ante: Vec<Var>,
            var_b: Var,
        ) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            move |egraph, _, subst| {
                let antes = var_ante.iter().map(|v| subst[*v]).collect::<Vec<Id>>();
                let b = subst[var_b];
                let extractor = Extractor::new(&egraph, AstSize);
                let ante_fml = antes
                    .iter()
                    .map(|a| extractor.find_best(*a).1)
                    .map(|f| f.to_string());
                let ante_fml_str = ante_fml
                    .fold("true".to_string(), |acc, f| format!("(and {} {})", acc, f))
                    .to_string();
                let b_fml = extractor.find_best(b).1.to_string();
                let assumptions = unsafe { ASSUMPTIONS.clone() };
                z3utils::imply(format!("(and {} {})", ante_fml_str, assumptions), b_fml)
            }
        }

        vec![
            //// These rules are expensive but potentially useful.
            rewrite!("or-comm"; "(or ?a ?b)" => "(or ?b ?a)"),
            rewrite!("or-assoc"; "(or ?a (or ?b ?c))" => "(or (or ?a ?b) ?c)"),
            //
            // Removing redundant disjuncts.
            rewrite!("redundant-disjunct-1"; "(or ?a ?b)" => "?a"
                if implies_lst(vec![var("?b")], var("?a"))),
            rewrite!("redundant-disjunct-2"; "(or ?a (or ?b ?c))" => "(or ?b ?c)"
                if redundant_disjunct(var("?a"), var("?b"), var("?c"))),
            // with AND based assumptions.
            rewrite!("redundant-disjunct-3"; "(and ?c (or ?a ?b))" => "(and ?c ?a)"
                if implies_lst(vec![var("?b"), var("?c")], var("?a"))),
            rewrite!("redundant-disjunct-3-comm"; "(and (or ?a ?b) ?c)" => "(and ?a ?c)"
                if implies_lst(vec![var("?b"), var("?c")], var("?a"))),
            // Depth 4.
            rewrite!("redundant-disjunct-4"; "(and ?d (and ?c (or ?a ?b)))" => "(and ?d (and ?c ?a))"
                if implies_lst(vec![var("?b"), var("?c"), var("?d")], var("?a"))),
            // commute innermost and.
            rewrite!("redundant-disjunct-4-comm-inner"; "(and ?d (and (or ?a ?b) ?c))" => "(and ?d (and ?a ?c))"
                if implies_lst(vec![var("?b"), var("?c"), var("?d")], var("?a"))),
            // commute outermost and.
            rewrite!("redundant-disjunct-4-comm-outer"; "(and (and ?d (or ?a ?b)) ?c)" => "(and (and ?d ?a) ?c)"
                if implies_lst(vec![var("?b"), var("?c"), var("?d")], var("?a"))),
            // commute both ands.
            rewrite!("redundant-disjunct-4-comm-both"; "(and (and (or ?a ?b) ?d) ?c)" => "(and (and ?a ?d) ?c)"
                if implies_lst(vec![var("?b"), var("?c"), var("?d")], var("?a"))),
            // Intersperse an or in the inner ands.
            rewrite!("redundant-disjunct-4-intersperse"; "(and ?d (or (and ?c (or ?a ?b)) ?e))" => "(and ?d (or (and ?c ?a) ?e))"
                if implies_lst(vec![var("?b"), var("?c"), var("?d")], var("?a"))),
            // Commute the inner and.
            rewrite!("redundant-disjunct-4--intersperse-comm-inner"; "(and ?d (or (and (or ?a ?b) ?c) ?e))" => "(and ?d (or (and ?a ?c) ?e))"
                if implies_lst(vec![var("?b"), var("?c"), var("?d")], var("?a"))),
            // Commute the outer and.
            rewrite!("redundant-disjunct-4--intersperse-comm-outer"; "(and (or (and ?c (or ?a ?b)) ?e) ?d)" => "(and (or (and ?c ?a) ?e) ?d)"
                if implies_lst(vec![var("?b"), var("?c"), var("?d")], var("?a"))),
            // Commute both ands.
            rewrite!("redundant-disjunct-4--intersperse-comm-both"; "(and (or (and (or ?a ?b) ?c) ?e) ?d)" => "(and (or (and ?a ?c) ?e) ?d)"
                if implies_lst(vec![var("?b"), var("?c"), var("?d")], var("?a"))),
        ]
    }
    /// This function returns the simplification for a given formula.
    ///
    /// # Parameters
    ///
    /// - 'problem': The problem to be simplified. Must be a `String` value.
    /// - 'assumptions': The assumptions to be associated with the problem.
    ///
    /// # Returns
    ///
    /// A `String` of the simplified problem.

    pub fn simplify(problem: String, assumptions: String) -> String {
        unsafe { ASSUMPTIONS = assumptions };

        base::simplify(problem, true, config::SHORT_TIMEOUT, Self::make_rules())
    }
}
