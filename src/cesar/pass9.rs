use crate::cesar::{language::PropLang, z3utils};
use crate::cesar::config;
use egg::*;

/// This pass does more redundant disjunct elimination.
/// (?a|?b) & ?c | ?a & ?d -> ?b & ?c | ?a & ?d when after the transformation, ?a & ?c -> ?a & ?d
pub struct Pass9;

pub static mut ASSUMPTIONS: String =  String::new();

fn var(s: &str) -> Var {
    s.parse().unwrap()
}

impl Pass9 {

    // reference: https://docs.rs/egg/latest/egg/macro.rewrite.html.
    fn make_rules() -> Vec<Rewrite<PropLang, ()>> {

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
            rewrite!("and-comm"; "(and ?a ?b)" => "(and ?b ?a)"),
            // We only apply and associativity across a disjunction of potential interest to the elimination rules
            rewrite!("and-assoc-1"; "(or (and ?a (and ?b ?c)) ?d)" => "(or (and (and ?a ?b) ?c) ?d)"),
            rewrite!("and-assoc-2"; "(or ?d (and ?a (and ?b ?c)))" => "(or ?d (and (and ?a ?b) ?c))"),
            // The actual elimination rules.
            rewrite!("or-elim-1"; "(or (and (or ?a ?b) ?c) (and ?a ?d))" => "(or (and ?b ?c) (and ?a ?d))"
                if implies_lst(vec![var("?c")], var("?d"))),
            rewrite!("or-elim-2"; "(or (and (or ?b ?a) ?c) (and ?a ?d))" => "(or (and ?b ?c) (and ?a ?d))"
                if implies_lst(vec![var("?c")], var("?d"))),
        ]
    }

    pub fn simplify(problem: String, assumptions: String) -> String {
        unsafe { ASSUMPTIONS = assumptions };

        // Parse the problem, the assumptions, and the rules
        let problem = problem.parse().unwrap();
        let rules = Pass9::make_rules();

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