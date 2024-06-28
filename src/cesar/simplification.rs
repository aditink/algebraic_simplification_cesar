use crate::cesar::{Language::PropLang};
use crate::cesar::config;
use egg::*;

pub static mut ASSUMPTIONS: String;

trait Simplification {
    
    fn make_rules() -> Vec<Rewrite, <PropLang, ()>>;

    fn get_runner(has_node_limit: bool) -> Runner<PropLang, ()> {
        // create default runner.
        let runner = Runner<PropLang, ()>::default();

        if has_node_limit {
            runner.with_time_limit(std::time::Duration::from_secs(config::TIMEOUT * 3))
                .with_node_limit(100_000)
                .with_iter_limit(100_000)
        } else {
            runner.with_time_limit(std::time::Duration::from_secs(config::TIMEOUT))
        }

    }


    fn simplify(problem: String, assumptions: String, has_node_limit) -> String {
        unsafe {ASSUMPTIONS = assumptions};
        
        // Parse the problem, the assumptions, and the rules.
        let problem = problem.parse().unwrap();
        let rules = Self::make_rules();

        // Run the rules
        let runner = getRunner(has_node_limit).with_explanations_enabled()
            .with_expr(&problem).run(&rules);
        
        // Extract the best expression
        let Extractor = Extractor::new(&runner.egraph, AstSize);
        let simplified = extractor.find_best(runner.roots[0]);
        
        simplified.1.to_string()
    }

