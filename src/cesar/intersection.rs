// use crate::cesar::{language::PropLang, z3utils};
// use egg::*;

// pub struct Intersection;

// pub static mut ASSUMPTIONS: String =  String::new();

// fn var(s: &str) -> Var {
//     s.parse().unwrap()
// }

// impl Intersection {

//     // reference: https://docs.rs/egg/latest/egg/macro.rewrite.html.
//     fn make_rules() -> Vec<Rewrite<PropLang, ()>> {

//         fn _implied_by_assumptions() -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
//             move |egraph, id, _| {
//                     unsafe {
//                     let assumptions = ASSUMPTIONS.clone();
//                     let extractor = Extractor::new(&egraph, AstSize);
//                     let representative = extractor.find_best(id);
//                     let subexpr = representative.1.to_string();
//                     z3utils::imply((assumptions).to_string(), subexpr)
//                     }
//         }}

//         fn _neg_implied_by_assumptions() -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
//             move |egraph, id, _| {
//                     unsafe {
//                     let assumptions = ASSUMPTIONS.clone();
//                     let extractor = Extractor::new(&egraph, AstSize);
//                     let representative = extractor.find_best(id);
//                     let subexpr = representative.1.to_string();
//                     z3utils::imply((assumptions).to_string(), format!("(not {})", subexpr))
//                     }
//                     }
//         }

//         /// Return true if (assumptions and common and cond) <-> (assumption and common and predicate).
//         fn cell_collapse_guard(var_common: Var, var_cond: Var, var_predicate: Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
//         // fn cell_collapse_guard(common: &'static str, cond: &'static str, predicate: &'static str) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
//         // fn cell_collapse_guard(common: [&str; 3]) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
//             // let var_common: Var = common.parse().unwrap();
//             // let var_cond: Var = cond.parse().unwrap();
//             // let var_predicate: Var = predicate.parse().unwrap();
//             move |egraph, _, subst| {
//                 let common = subst[var_common];
//                 let cond = subst[var_cond];
//                 let predicate = subst[var_predicate];
//                 let extractor = Extractor::new(&egraph, AstSize);
//                 let common_fml = extractor.find_best(common).1.to_string();
//                 let cond_fml = extractor.find_best(cond).1.to_string();
//                 let predicate_fml = extractor.find_best(predicate).1.to_string();
//                 let assumptions = unsafe { ASSUMPTIONS.clone() };
//                 let forward_implies = z3utils::imply(format!("(and (and {} {}) {})", common_fml, cond_fml, assumptions), predicate_fml.clone());
//                 let backward_implies = z3utils::imply(format!("(and (and {} {}) {})", predicate_fml, cond_fml, assumptions), common_fml);
//                 forward_implies && backward_implies
//             }
//         }

//         /// Return true if (assumptions and common and cond) -> (predicate), where cond is constructed from x=y.
//         fn cell_collapse_guard_equal(var_common: Var, var_x: Var, var_y: Var, var_predicate: Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
//             cell_collapse_guard_fn(var_common, var_x, var_y, var_predicate, |x_trm, y_trm| { format!("(= {} {})", x_trm, y_trm) })
//         }

//         /// Return true if (assumptions and common and cond) -> (predicate), where cond is constructed from x<y.
//         fn cell_collapse_guard_lt(var_common: Var, var_x: Var, var_y: Var, var_predicate: Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
//             cell_collapse_guard_fn(var_common, var_x, var_y, var_predicate, |x_trm, y_trm| { format!("(< {} {})", x_trm, y_trm) })
//         }

//         /// Return true if (assumptions and common and cond) -> (predicate), where cond is constructed from x, y, and cond_fml.
//         fn cell_collapse_guard_fn(var_common: Var, var_x: Var, var_y: Var, var_predicate: Var, cond_fml: impl Fn(String, String)->String) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
//             move |egraph, _, subst| {
//                 let common = subst[var_common];
//                 let x = subst[var_x];
//                 let y = subst[var_y];
//                 let predicate = subst[var_predicate];
//                 let extractor = Extractor::new(&egraph, AstSize);
//                 let common_fml = extractor.find_best(common).1.to_string();
//                 let x_trm = extractor.find_best(x).1.to_string();
//                 let y_trm = extractor.find_best(y).1.to_string();
//                 let cond_fml = cond_fml(x_trm, y_trm);
//                 let predicate_fml = extractor.find_best(predicate).1.to_string();
//                 let assumptions = unsafe { ASSUMPTIONS.clone() };
//                 z3utils::imply(format!("(and (and {} {}) {})", common_fml, cond_fml, assumptions), predicate_fml.clone())
//             }
//         }

//         /// Return true if (assumptions and a) -> (b | x<=y).
//         fn redundancy_elimination_leq(var_a: Var, var_b: Var, var_x: Var, var_y: Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
//             redundancy_elimination_fn(var_a, var_b, var_x, var_y, |x_trm, y_trm| { format!("(<= {} {})", x_trm, y_trm) })
//             }

//         /// Return true if (assumptions and a) -> (b | x>y).
//         fn redundancy_elimination_gt(var_a: Var, var_b: Var, var_x: Var, var_y: Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
//             redundancy_elimination_fn(var_a, var_b, var_x, var_y, |x_trm, y_trm| { format!("(> {} {})", x_trm, y_trm) })
//             }

//         // Return true if (assumptions and a) -> (b | f(x, y)).
//         fn redundancy_elimination_fn(var_a: Var, var_b: Var, var_x: Var, var_y: Var, op_fn: impl Fn(String, String) -> String) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
//             move |egraph, _, subst| {
//                 let a = subst[var_a];
//                 let b = subst[var_b];
//                 let x = subst[var_x];
//                 let y = subst[var_y];
//                 let extractor = Extractor::new(&egraph, AstSize);
//                 let a_fml = extractor.find_best(a).1.to_string();
//                 let b_fml = extractor.find_best(b).1.to_string();
//                 let x_trm = extractor.find_best(x).1.to_string();
//                 let y_trm = extractor.find_best(y).1.to_string();
//                 let assumptions = unsafe { ASSUMPTIONS.clone() };
//                 z3utils::imply(format!("(and {} {})", a_fml, assumptions), format!("(or {} {})", b_fml, op_fn(x_trm, y_trm)))
//             }
//         }

//         vec![
//             // Logic and math rules.
//             rewrite!("and-id"; "(and ?a true)" => "?a"),
//             rewrite!("less-equals"; "(or (< ?a ?b) (= ?a ?b))" => "(<= ?a ?b)"),
//             rewrite!("greater-equals"; "(or (> ?a ?b) (= ?a ?b))" => "(>= ?a ?b)"),
//             rewrite!("comparison-true-lt"; "(or (< ?a ?b) (>= ?a ?b))" => "true"),
//             rewrite!("comparison-true-gt"; "(or (> ?a ?b) (<= ?a ?b))" => "true"),
//             rewrite!("equal-commutes"; "(= ?a ?b)" => "(= ?b ?a)"),
//             // Cell Collapse Rules.
//             rewrite!("cell-collapse-generic"; "(or (and ?a ?b) (and ?c ?d))" => "(and (or ?a ?c) ?b)"
//                 if cell_collapse_guard(var("?b"), var("?c"), var("?d"))),
//             // Casing on symbol split.
//             rewrite!("cell-collapse-special-equal-1"; "(or (and (= ?x ?y) ?b) (and (> ?x ?y) ?d))" => "(and (>= ?x ?y) ?d)"
//                 if cell_collapse_guard_equal(var("?d"), var("?x"), var("?y"), var("?b"))),
//             rewrite!("cell-collapse-special-equal-2"; "(or (and (= ?x ?y) ?b) (and (< ?x ?y) ?d))" => "(and (<= ?x ?y) ?d)"
//                 if cell_collapse_guard_equal(var("?d"), var("?x"), var("?y"), var("?b"))),
//             rewrite!("cell-collapse-special-lessthan"; "(or (and (< ?x ?y) ?b) (and (>= ?x ?y) ?d))" => "?d"
//                 if cell_collapse_guard_lt(var("?d"), var("?x"), var("?y"), var("?b"))),
//             // Removing redundant disjuncts.
//             rewrite!("redundant-casing"; "(and (or ?a (> ?x ?y)) (or ?b (<= ?x ?y)))" => "(or ?a ?b)"
//                 if redundancy_elimination_leq(var("?a"), var("?b"), var("?x"), var("?y"))
//                 if redundancy_elimination_gt(var("?b"), var("?a"), var("?x"), var("?y"))),
//         ]
//     }

//     pub fn simplify(problem: String, assumptions: String) -> String {
//         unsafe { ASSUMPTIONS = assumptions };

//         // Parse the problem, the assumptions, and the rules
//         let problem = problem.parse().unwrap();
//         let rules = Intersection::make_rules();

//         // Run the rules
//         let mut runner = Runner::<PropLang, ()>::default()
//         .with_time_limit(std::time::Duration::from_secs(600))
//             .with_explanations_enabled().with_expr(&problem).run(&rules);

//         // Extract the best expression
//         let extractor = Extractor::new(&runner.egraph, AstSize);
//         let simplified = extractor.find_best(runner.roots[0]);

//         // Explain the equivalences
//         let explanation = runner.explain_equivalence(&problem, &simplified.1).get_flat_string();

//         // Print the original problem, the assumptions, the simplified problem,
//         // its cost, and the explanations
//         println!("Original problem: {}", problem);
//         println!("Simplified problem: {}", simplified.1);
//         println!("Cost: {}", simplified.0);
//         println!("Runner stop reason: {:?}", runner.stop_reason);
//         println!("Explanations: {}", explanation);
//         simplified.1.to_string()
//     }
// }