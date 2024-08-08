use crate::language::PropLang;
use crate::config;
use egg::*;

/// Moves all instances of numerator_var to numerator, assuming that is is >0.
pub struct RearrangePass;

pub static mut NUMERATOR_VAR: String =  String::new();

fn var(s: &str) -> Var {
    s.parse().unwrap()
}

impl RearrangePass {


    // reference: https://docs.rs/egg/latest/egg/macro.rewrite.html.
    fn make_rules() -> Vec<Rewrite<PropLang, ()>> {
        
        fn is_numerator_var(var: Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            move |egraph, _, subst| {
                let numerator_var = unsafe { NUMERATOR_VAR.clone() };
                let var = subst[var];
                let extractor = Extractor::new(&egraph, AstSize);
                let var_fml = extractor.find_best(var).1.to_string();
                var_fml == numerator_var
            }
        }
    
        vec![
            // Moving numerator_var to numerator.
            rewrite!("eq-numerator"; "(= ?x (* (^ ?v (- 1)) ?y))" => "(= (* ?x ?v) ?y)"
                if is_numerator_var(var("?v"))),
            rewrite!("leq-numerator"; "(<= ?x (* (^ ?v (- 1)) ?y))" => "(<= (* ?x ?v) ?y)"
                if is_numerator_var(var("?v"))),
            rewrite!("geq-numerator"; "(>= ?x (* (^ ?v (- 1)) ?y))" => "(>= (* ?x ?v) ?y)"
                if is_numerator_var(var("?v"))),
            rewrite!("lt-numerator"; "(< ?x (* (^ ?v (- 1)) ?y))" => "(< (* ?x ?v) ?y)"
                if is_numerator_var(var("?v"))),
            rewrite!("gt-numerator"; "(> ?x (* (^ ?v (- 1)) ?y))" => "(> (* ?x ?v) ?y)"
                if is_numerator_var(var("?v"))),
            rewrite!("neq-numerator"; "(distinct ?x (* (^ ?v (- 1)) ?y))" => "(distinct (* ?x ?v) ?y)"
                if is_numerator_var(var("?v"))),
            ////
            //// Above rules with position of ?x commuted.
            rewrite!("eq-numerator-comm"; "(= (* (^ ?v (- 1)) ?y) ?x)" => "(= (* ?x ?v) ?y)"
                if is_numerator_var(var("?v"))),
            rewrite!("leq-numerator-comm"; "(<= (* (^ ?v (- 1)) ?y) ?x)" => "(<= (* ?x ?v) ?y)"
                if is_numerator_var(var("?v"))),
            rewrite!("geq-numerator-comm"; "(>= (* (^ ?v (- 1)) ?y) ?x)" => "(>= (* ?x ?v) ?y)"
                if is_numerator_var(var("?v"))),
            rewrite!("lt-numerator-comm"; "(< (* (^ ?v (- 1)) ?y) ?x)" => "(< (* ?x ?v) ?y)" 
                if is_numerator_var(var("?v"))),
            rewrite!("gt-numerator-comm"; "(> (* (^ ?v (- 1)) ?y) ?x)" => "(> (* ?x ?v) ?y)"
                if is_numerator_var(var("?v"))),
            rewrite!("neq-numerator-comm"; "(distinct (* (^ ?v (- 1)) ?y) ?x)" => "(distinct (* ?x ?v) ?y)"
                if is_numerator_var(var("?v"))),
            // Multiplication commutes.
            rewrite!("mul-comm"; "(* ?x ?y)" => "(* ?y ?x)"),
            // Multiplication associates.
            rewrite!("mul-assoc"; "(* ?x (* ?y ?z))" => "(* (* ?x ?y) ?z)"),
        ]
    }

    /// Rewrite problem with all instances of numerator_var moved to numerator, 
    /// assuming that numerator_var>0.
    pub fn rearrange(problem: String, numerator_var: String) -> String {
        unsafe { NUMERATOR_VAR = numerator_var };

        // Parse the problem, the assumptions, and the rules
        let problem = problem.parse().unwrap();
        let rules = RearrangePass::make_rules();

        // Run the rules
        let runner = Runner::<PropLang, ()>::default()
        .with_time_limit(std::time::Duration::from_secs(config::TIMEOUT))
            .with_explanations_enabled().with_expr(&problem).run(&rules);

        // Extract the best expression
        let extractor = Extractor::new(&runner.egraph, AstSize);
        let simplified = extractor.find_best(runner.roots[0]);

        simplified.1.to_string()
    }
}