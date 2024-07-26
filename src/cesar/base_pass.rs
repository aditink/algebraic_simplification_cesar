use crate::{language::PropLang};
use crate::config;
use egg::*;

pub static mut ASSUMPTIONS: String = String::new();

/// A trait containing functions for simplification of formulas.
///
/// This trait contains the default implementation for the simplify function, which is used by the
/// passes when simplifyig formulas. 
/// It also defines the make_rules function, which has unique implementation for all the passes,
/// which is why it is missing a default implementation.
pub trait BasePass {
    
    fn make_rules() -> Vec<Rewrite<PropLang, ()>>;

    fn get_runner(has_node_limit: bool) -> Runner<PropLang, ()> {
        // create default runner.
        let runner = Runner::<PropLang, ()>::default();

        if has_node_limit {
            runner
                .with_node_limit(100_000)
                .with_iter_limit(100_000)
        } else {
            runner
        }

    }


    fn simplify(problem: String, assumptions: String, has_node_limit: bool, timeout_multiplier: u64) -> String {
        unsafe {ASSUMPTIONS = assumptions};
        
        // Parse the problem, the assumptions, and the rules.
        let problem = problem.parse().unwrap();
        let rules = Self::make_rules();

        // Run the rules
        let mut runner = Self::get_runner(has_node_limit)
            .with_time_limit(std::time::Duration::from_secs(config::TIMEOUT * timeout_multiplier))
            .with_explanations_enabled()
            .with_expr(&problem).run(&rules);
        
        // Extract the best expression
        let extractor  = Extractor::new(&runner.egraph, AstSize);
        let simplified = extractor.find_best(runner.roots[0]);

        // Debug Info
        if config::DEBUG {
            println!();
            println!("--+--+--+--+");
            println!("Original Problem: {}", problem);
            println!("Simplified Problem: {}", simplified.1);
            println!("Cost of Simplification: {}", simplified.0);
            println!("Runner Stop Reason: {:?}", runner.stop_reason);
            // Get the explanataion
            let explanation = runner.explain_equivalence(&problem, &simplified.1).get_flat_string();
            println!("Explanations: {}", explanation);
            println!("--+--+--+--+");
        }
        
        simplified.1.to_string()
    }
    
}
