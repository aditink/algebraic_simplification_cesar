use crate::cesar::language::PropLang;
use crate::cesar::config;
use egg::*;

/// This pass does multiplication distribution, i.e. the transformation x*a + x*b = x*(a+b).
pub struct Pass7;

pub static mut ASSUMPTIONS: String =  String::new();

impl Pass7 {

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
            // Times 1 is just the number.
            rewrite!("times-one"; "(* ?x 1)" => "?x"),
        ]
    }

    pub fn simplify(problem: String, assumptions: String) -> String {
        unsafe { ASSUMPTIONS = assumptions };

        // Parse the problem, the assumptions, and the rules
        let problem = problem.parse().unwrap();
        let rules = Pass7::make_rules();

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