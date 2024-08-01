use crate::cesar::language::PropLang;
use egg::*;
use log::debug;
use std::time::Duration;

/// Helper function to return a new runner with the required properties.
///
/// # Parameters
///
/// - 'has_node_limit': Determines if the returned runner needs to have an iter limit and a node
/// limit. Both limits are 100_000.
///
/// # Returns
///
/// A new Runner to simplify the problem.
fn get_runner(has_node_limit: bool) -> Runner<PropLang, ()> {
    // Default runner
    let runner = Runner::<PropLang, ()>::default();

    if has_node_limit {
        runner.with_node_limit(100_000).with_iter_limit(100_000)
    } else {
        runner
    }
}

/// This function simplifes a problem, given the rules for simplification.
///
/// # Parameters
///
/// - 'problem': The formula to be simplified. Must be a `String` value.
///
/// - 'has_node_limit': Determines if the returned runner needs to have an iter limit and a node
/// limit. Both limits are 100_000.
///
/// - 'timeout': The timeout for the runner. The runner will stop when it has been running for
/// timeout seconds regardless of its progress. Must be a positive `u64` value.
///
/// - 'rules': The rules for simplification. These rules are generated in their respective passes.
/// Must be a `Vec<Rewrite<PropLang, ()>>`.
///
/// # Returns
///
/// A `String` of the simplified formula.
///

pub fn simplify(
    problem: String,
    has_node_limit: bool,
    timeout: u64,
    rules: Vec<Rewrite<PropLang, ()>>,
) -> String {
    debug!(
        "Running simplify with has_node_limit set to {}",
        has_node_limit
    );

    let problem = problem.parse().unwrap();

    // Get the runner with the needed properties.
    let mut runner = get_runner(has_node_limit)
        .with_time_limit(Duration::from_secs(timeout))
        .with_explanations_enabled()
        .with_expr(&problem)
        .run(&rules);

    // Helps to extract the most simplified formula from the graph.
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let simplified = extractor.find_best(runner.roots[0]);

    debug!("Simplification finished.");
    debug!("Original Problem: {}", problem);
    debug!("Simplified Problem: {}", simplified.1);
    debug!("Cost of simplification: {}", simplified.0);
    debug!("Runner stop reason: {:?}", runner.stop_reason);

    let explanation = runner
        .explain_equivalence(&problem, &simplified.1)
        .get_flat_string();
    debug!("Explanations: {}", explanation);

    // Return the simplifed problem as a String.
    simplified.1.to_string()
}
