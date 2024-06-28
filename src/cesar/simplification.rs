use crate::cesar::{Language::PropLang};
use crate::cesar::config;
use egg::*;

pub static mut ASSUMPTIONS: String;

trait Simplification {
    
    fn make_rules() -> Vec<Rewrite, <PropLang, ()>>;

    fn simplify(problem: String, assumptions: String) -> String {
        unsafe {ASSUMPTIONS = assumptions};
        
        // Parse the problem, the assumptions, and the rules.
        let problem = problem.parse().unwrap();
        let rules = Self::make_rules();

        // Run the rules
        // TODO Let runner = getRunner(); 
        
        // Extract the best expression
        let Extractor = Extractor::new(&runner.egraph, AstSize);
        let simplified = extractor.find_best(runner.roots[0]);
        
        simplified.1.to_string()
    }

