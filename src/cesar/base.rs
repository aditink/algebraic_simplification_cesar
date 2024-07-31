use crate::cesar::config;
use crate::cesar::language::PropLang;
use egg::*;

/// The function returns a new runner with the required properties.
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

pub fn simplify(
    problem: String,
    has_node_limit: bool,
    timeout: u64,
    rules: Vec<Rewrite<PropLang, ()>>,
) -> String {
}
