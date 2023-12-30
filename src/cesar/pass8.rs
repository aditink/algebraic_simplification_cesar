use crate::cesar::{language::PropLang, z3utils};
use crate::cesar::config;
use egg::*;

/// This pass moves division to multiplication positions accounting for sign.
pub struct Pass8;

pub static mut ASSUMPTIONS: String =  String::new();

fn var(s: &str) -> Var {
    s.parse().unwrap()
}

impl Pass8 {

    // reference: https://docs.rs/egg/latest/egg/macro.rewrite.html.
    fn make_rules() -> Vec<Rewrite<PropLang, ()>> {

        fn is_non_zero(var_ante: Vec<Var>, var_b: Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            check_b_cond(var_ante, var_b, |b_trm| { format!("(distinct {} 0)", b_trm) })
        }

        fn less_than_zero(var_ante: Vec<Var>, var_b: Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            check_b_cond(var_ante, var_b, |b_trm| { format!("(< {} 0)", b_trm) })
        }

        // Return true if (assumptions and (AND_i a_i)) -> b>0.
        fn greater_than_zero(var_ante: Vec<Var>, var_b: Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            check_b_cond(var_ante, var_b, |b_trm| { format!("(> {} 0)", b_trm) })
        }

        // Return true if (assumptions and (AND_i a_i)) -> b>0.
        fn check_b_cond(var_ante: Vec<Var>, var_b: Var, b_cond: impl Fn(String) -> String) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            move |egraph, _, subst| {
                let antes = var_ante.iter().map(|v| subst[*v]).collect::<Vec<Id>>();
                let b = subst[var_b];
                let extractor = Extractor::new(&egraph, AstSize);
                let ante_fml = antes.iter()
                    .map(|a| extractor.find_best(*a).1)
                    .map(|f| f.to_string());
                let ante_fml_str = ante_fml.fold("true".to_string(),
                    |acc, f| format!("(and {} {})", acc, f)).to_string();
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

    pub fn simplify(problem: String, assumptions: String) -> String {
        unsafe { ASSUMPTIONS = assumptions };

        // Parse the problem, the assumptions, and the rules
        let problem = problem.parse().unwrap();
        let rules = Pass8::make_rules();

        // Run the rules
        let runner = Runner::<PropLang, ()>::default()
        .with_time_limit(std::time::Duration::from_secs(config::TIMEOUT * 3))
        .with_node_limit(100000)
        .with_iter_limit(100000)
            .with_explanations_enabled().with_expr(&problem).run(&rules);

        // Extract the best expression
        let extractor = Extractor::new(&runner.egraph, AstSize);
        let simplified = extractor.find_best(runner.roots[0]);

        // Explain the equivalences
        // let explanation = runner.explain_equivalence(&problem, &simplified.1).get_flat_string();

        // Print the original problem, the assumptions, the simplified problem,
        // its cost, and the explanations
        // println!("Original problem: {}", problem);
        // println!("Simplified problem: {}", simplified.1);
        // println!("Cost: {}", simplified.0);
        // println!("Runner stop reason: {:?}", runner.stop_reason);
        // println!("Explanations: {}", explanation);
        simplified.1.to_string()
    }
}