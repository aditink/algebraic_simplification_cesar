use crate::cesar::{language::PropLang, z3utils};
use crate::cesar::config;
use egg::*;

pub struct Pass3;

pub static mut ASSUMPTIONS: String =  String::new();

fn var(s: &str) -> Var {
    s.parse().unwrap()
}

impl Pass3 {

    // reference: https://docs.rs/egg/latest/egg/macro.rewrite.html.
    fn make_rules() -> Vec<Rewrite<PropLang, ()>> {

        // Return true if (assumptions and a and (not b)) -> (<= x y).
        fn redundancy_elimination_or_leq(var_a: Var, var_b: Var, var_x: Var, var_y: Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            redundancy_elimination_or_fn(var_a, var_b, var_x, var_y, |x_trm, y_trm| { format!("(<= {} {})", x_trm, y_trm) })
            }

        /// Return true if (assumptions and a and (not b)) -> (> x y).
        fn redundancy_elimination_or_gt(var_a: Var, var_b: Var, var_x: Var, var_y: Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            redundancy_elimination_or_fn(var_a, var_b, var_x, var_y, |x_trm, y_trm| { format!("(> {} {})", x_trm, y_trm) })
            }

        // Return true if (assumptions and a and (not b)) -> (f(x, y)).
        fn redundancy_elimination_or_fn(var_a: Var, var_b: Var, var_x: Var, var_y: Var, op_fn: impl Fn(String, String) -> String) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            move |egraph, _, subst| {
                let a = subst[var_a];
                let b = subst[var_b];
                let x = subst[var_x];
                let y = subst[var_y];
                let extractor = Extractor::new(&egraph, AstSize);
                let a_fml = extractor.find_best(a).1.to_string();
                let b_fml = extractor.find_best(b).1.to_string();
                let x_trm = extractor.find_best(x).1.to_string();
                let y_trm = extractor.find_best(y).1.to_string();
                let assumptions = unsafe { ASSUMPTIONS.clone() };
                z3utils::imply(format!("(and (and {} (not {})) {})",
                  a_fml, b_fml, assumptions), op_fn(x_trm, y_trm))
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
            rewrite!("or-comm"; "(or ?a ?b)" => "(or ?b ?a)"),
            rewrite!("and-comm"; "(and ?a ?b)" => "(and ?b ?a)"),
            rewrite!("and-assoc"; "(and ?a (& ?b ?c))" => "(and (& ?a ?b) ?c)"),
            rewrite!("or-assoc"; "(or ?a (or ?b ?c))" => "(or (or ?a ?b) ?c)"),
            //
            rewrite!("less-equals"; "(or (< ?a ?b) (= ?a ?b))" => "(<= ?a ?b)"),
            rewrite!("greater-equals"; "(or (> ?a ?b) (= ?a ?b))" => "(>= ?a ?b)"),
            rewrite!("comparison-true-lt"; "(or (< ?a ?b) (>= ?a ?b))" => "true"),
            rewrite!("comparison-true-gt"; "(or (> ?a ?b) (<= ?a ?b))" => "true"),
            rewrite!("equal-commutes"; "(= ?a ?b)" => "(= ?b ?a)"),
            // Removing redundant disjuncts.
            rewrite!("redundant-casing-or"; "(or (and ?a (<= ?x ?y)) (and ?b (> ?x ?y)))" => "(or ?a ?b)"
                if redundancy_elimination_or_leq(var("?a"), var("?b"), var("?x"), var("?y"))
                if redundancy_elimination_or_gt(var("?b"), var("?a"), var("?x"), var("?y"))),
        ]
    }

    pub fn simplify(problem: String, assumptions: String) -> String {
        unsafe { ASSUMPTIONS = assumptions };

        // Parse the problem, the assumptions, and the rules
        let problem = problem.parse().unwrap();
        let rules = Pass3::make_rules();

        // Run the rules
        let runner = Runner::<PropLang, ()>::default()
        .with_time_limit(std::time::Duration::from_secs(config::TIMEOUT))
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