
use z3::{SatResult, Solver};

use super::language::RESERVED_SYMBOLS;

/// Return a vector of all the variables in the formulas a and b.
fn extract_all_vars(a: String) -> Vec<String> {
    let a_symbols = a.split_whitespace().collect::<Vec<&str>>();
    // If a symbol begins or ends with a parenthesis, trim out the parenthesis.
    let trimmed_a_symbols = a_symbols.iter().map(|x| {
        if x.starts_with("(") && x.ends_with(")") {
            x.trim_start_matches("(").trim_end_matches(")")
        } else if x.starts_with("(") {
            x.trim_start_matches("(")
        } else if x.ends_with(")") {
            x.trim_end_matches(")")
        } else {
            x
        }
    }).collect::<Vec<&str>>();
    // Filter out all reserved symbols, numbers, and empty strings.
    let filtered_a_symbols = trimmed_a_symbols.iter().filter(|x| {
        let op = **x;
        !RESERVED_SYMBOLS.contains(&op) && !(op).parse::<f64>().is_ok() && op != ""
    }).map(|x| {*x}.to_string())
    .collect::<Vec<String>>();
  filtered_a_symbols
}

/// Return true if propositional logic formula a implies b.
pub fn imply(a: String, b: String) -> bool {
    // Temporary code for experimentation.
    let a_vars = extract_all_vars(a.clone());
    let b_vars = extract_all_vars(b.clone());
    let dup_vars = a_vars.iter().chain(b_vars.iter()).cloned().collect::<Vec<String>>();
    // deduplicate.
    let vars = dup_vars.iter().cloned().collect::<std::collections::HashSet<String>>();

    // First check if b is even a formula, i.e. the top level operator is one of
    // the following: and, or, not, true, false.
    // The form of the string that b is in is ( op ... ).
    let b_for_analysis = b.clone();
    let b_symbols = b_for_analysis.split_whitespace().collect::<Vec<&str>>();
    if b_symbols.len() < 1 {
        return false;
    }
    let b_op = b_symbols[0];
    if b_op != "(and" && b_op != "(or" && b_op != "(not" && b_op != "true" && b_op != "false"
    && b_op != "(=>" && b_op != "(=" && b_op != "(>=" && b_op != "(<=" 
    && b_op != "(<" && b_op != "(>" && b_op != "(distinct" {
        return false;
    }

    // Now attempt to solve.
    // See https://github.com/prove-rs/z3.rs/blob/master/z3/tests/lib.rs ln 236.
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    // Now we construct the problem string.
    // First, we need to declare the variables.
    let prelude = vars.iter().fold(String::new(), |acc, var| {
        acc + "(declare-const " + var + " Real)\n"
    });
    // We construct the string for !(a -> b), i.e., (a and (not b)).
    let not_a_implies_b = format!("(and {} (not {}))", a, b);
    // Now we need to assert not_a_implies_b.
    let problem = prelude + format!("(assert {})", not_a_implies_b).as_str();

    // We return true if the problem is unsatisfiable.
    let solver = Solver::new(&ctx);
    solver.from_string(problem);
    let result = solver.check();
    let output = result == SatResult::Unsat;
    output
}