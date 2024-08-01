use crate::cesar::base;
use crate::cesar::config;
use crate::cesar::{language::PropLang, z3utils};
use egg::*;

/// This pass moves division to multiplication positions accounting for sign.
pub struct Pass8;

pub static mut ASSUMPTIONS: String = String::new();

fn var(s: &str) -> Var {
    s.parse().unwrap()
}

impl Pass8 {
    // reference: https://docs.rs/egg/latest/egg/macro.rewrite.html.
    fn make_rules() -> Vec<Rewrite<PropLang, ()>> {
        fn is_non_zero(
            var_ante: Vec<Var>,
            var_b: Var,
        ) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            check_b_cond(var_ante, var_b, |b_trm| format!("(distinct {} 0)", b_trm))
        }

        fn less_than_zero(
            var_ante: Vec<Var>,
            var_b: Var,
        ) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            check_b_cond(var_ante, var_b, |b_trm| format!("(< {} 0)", b_trm))
        }

        // Return true if (assumptions and (AND_i a_i)) -> b>0.
        fn greater_than_zero(
            var_ante: Vec<Var>,
            var_b: Var,
        ) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            check_b_cond(var_ante, var_b, |b_trm| format!("(> {} 0)", b_trm))
        }

        // Return true if (assumptions and (AND_i a_i)) -> b>0.
        fn check_b_cond(
            var_ante: Vec<Var>,
            var_b: Var,
            b_cond: impl Fn(String) -> String,
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
                let b_trm = extractor.find_best(b).1.to_string();
                let b_fml = b_cond(b_trm);
                let assumptions = unsafe { ASSUMPTIONS.clone() };
                z3utils::imply(format!("(and {} {})", ante_fml_str, assumptions), b_fml)
            }
        }

        vec![
            // For the case of equality.
            rewrite!("eq-numerator"; "(= ?x (* (^ ?v (- 1)) ?y))" => "(= (* ?x ?v) ?y)"
                if is_non_zero(vec![], var("?v"))),
            // Inequalities with positive numbers.
            rewrite!("gt-numerator"; "(> ?x (* (^ ?v (- 1)) ?y))" => "(> (* ?x ?v) ?y)"
                if greater_than_zero(vec![], var("?v"))),
            rewrite!("lt-numerator"; "(< ?x (* (^ ?v (- 1)) ?y))" => "(< (* ?x ?v) ?y)"
                if greater_than_zero(vec![], var("?v"))),
            rewrite!("geq-numerator"; "(>= ?x (* (^ ?v (- 1)) ?y))" => "(>= (* ?x ?v) ?y)"
                if greater_than_zero(vec![], var("?v"))),
            rewrite!("leq-numerator"; "(<= ?x (* (^ ?v (- 1)) ?y))" => "(<= (* ?x ?v) ?y)"
                if greater_than_zero(vec![], var("?v"))),
            // Inequalities with negative numbers.
            rewrite!("gt-numerator-neg"; "(> ?x (* (^ ?v (- 1)) ?y))" => "(< (* ?x ?v) ?y)"
                if less_than_zero(vec![], var("?v"))),
            rewrite!("lt-numerator-neg"; "(< ?x (* (^ ?v (- 1)) ?y))" => "(> (* ?x ?v) ?y)"
                if less_than_zero(vec![], var("?v"))),
            rewrite!("geq-numerator-neg"; "(>= ?x (* (^ ?v (- 1)) ?y))" => "(<= (* ?x ?v) ?y)"
                if less_than_zero(vec![], var("?v"))),
            rewrite!("leq-numerator-neg"; "(<= ?x (* (^ ?v (- 1)) ?y))" => "(>= (* ?x ?v) ?y)"
                if less_than_zero(vec![], var("?v"))),
            // With conjuncts in context.
            rewrite!("eq-numerator-conj"; "(and (= ?x (* (^ ?v (- 1)) ?y)) ?a)" => "(and (= (* ?x ?v) ?y) ?a)"
                if is_non_zero(vec![var("?a")], var("?v"))),
            rewrite!("gt-numerator-conj"; "(and (> ?x (* (^ ?v (- 1)) ?y)) ?a)" => "(and (> (* ?x ?v) ?y) ?a)"
                if greater_than_zero(vec![var("?a")], var("?v"))),
            rewrite!("lt-numerator-conj"; "(and (< ?x (* (^ ?v (- 1)) ?y)) ?a)" => "(and (< (* ?x ?v) ?y) ?a)"
                if greater_than_zero(vec![var("?a")], var("?v"))),
            rewrite!("geq-numerator-conj"; "(and (>= ?x (* (^ ?v (- 1)) ?y)) ?a)" => "(and (>= (* ?x ?v) ?y) ?a)"
                if greater_than_zero(vec![var("?a")], var("?v"))),
            rewrite!("leq-numerator-conj"; "(and (<= ?x (* (^ ?v (- 1)) ?y)) ?a)" => "(and (<= (* ?x ?v) ?y) ?a)"
                if greater_than_zero(vec![var("?a")], var("?v"))),
            // Inequalities with negative numbers.
            rewrite!("gt-numerator-conj-neg"; "(and (> ?x (* (^ ?v (- 1)) ?y)) ?a)" => "(and (< (* ?x ?v) ?y) ?a)"
                if less_than_zero(vec![var("?a")], var("?v"))),
            rewrite!("lt-numerator-conj-neg"; "(and (< ?x (* (^ ?v (- 1)) ?y)) ?a)" => "(and (> (* ?x ?v) ?y) ?a)"
                if less_than_zero(vec![var("?a")], var("?v"))),
            rewrite!("geq-numerator-conj-neg"; "(and (>= ?x (* (^ ?v (- 1)) ?y)) ?a)" => "(and (<= (* ?x ?v) ?y) ?a)"
                if less_than_zero(vec![var("?a")], var("?v"))),
            rewrite!("leq-numerator-conj-neg"; "(and (<= ?x (* (^ ?v (- 1)) ?y)) ?a)" => "(and (>= (* ?x ?v) ?y) ?a)"
                if less_than_zero(vec![var("?a")], var("?v"))),
            // Multiplication commutes.
            rewrite!("mul-comm"; "(* ?x (^ ?v (- 1)))" => "(* (^ ?v (- 1)) ?x)"),
            // And commutes.
            rewrite!("and-comm"; "(and ?x ?y)" => "(and ?y ?x)"),
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

        base::simplify(problem, true, config::LONG_TIMEOUT, Self::make_rules())
    }
}
