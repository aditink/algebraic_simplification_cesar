use crate::cesar::base;
use crate::cesar::config;
use crate::cesar::{language::PropLang, z3utils};
use egg::*;

pub struct Pass2;

pub static mut ASSUMPTIONS: String = String::new();

fn var(s: &str) -> Var {
    s.parse().unwrap()
}

impl Pass2 {
    // reference: https://docs.rs/egg/latest/egg/macro.rewrite.html.
    fn make_rules() -> Vec<Rewrite<PropLang, ()>> {
        fn _implied_by_assumptions() -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            move |egraph, id, _| unsafe {
                let assumptions = ASSUMPTIONS.clone();
                let extractor = Extractor::new(&egraph, AstSize);
                let representative = extractor.find_best(id);
                let subexpr = representative.1.to_string();
                z3utils::imply((assumptions).to_string(), subexpr)
            }
        }

        fn _neg_implied_by_assumptions() -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            move |egraph, id, _| unsafe {
                let assumptions = ASSUMPTIONS.clone();
                let extractor = Extractor::new(&egraph, AstSize);
                let representative = extractor.find_best(id);
                let subexpr = representative.1.to_string();
                z3utils::imply((assumptions).to_string(), format!("(not {})", subexpr))
            }
        }

        /// Return true if (assumptions and common and cond) <-> (assumption and common and predicate).
        fn _cell_collapse_guard(
            var_common: Var,
            var_cond: Var,
            var_predicate: Var,
        ) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            // fn cell_collapse_guard(common: &'static str, cond: &'static str, predicate: &'static str) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            // fn cell_collapse_guard(common: [&str; 3]) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            // let var_common: Var = common.parse().unwrap();
            // let var_cond: Var = cond.parse().unwrap();
            // let var_predicate: Var = predicate.parse().unwrap();
            move |egraph, _, subst| {
                let common = subst[var_common];
                let cond = subst[var_cond];
                let predicate = subst[var_predicate];
                let extractor = Extractor::new(&egraph, AstSize);
                let common_fml = extractor.find_best(common).1.to_string();
                let cond_fml = extractor.find_best(cond).1.to_string();
                let predicate_fml = extractor.find_best(predicate).1.to_string();
                let assumptions = unsafe { ASSUMPTIONS.clone() };
                let forward_implies = z3utils::imply(
                    format!("(and (and {} {}) {})", common_fml, cond_fml, assumptions),
                    predicate_fml.clone(),
                );
                let backward_implies = z3utils::imply(
                    format!("(and (and {} {}) {})", predicate_fml, cond_fml, assumptions),
                    common_fml,
                );
                forward_implies && backward_implies
            }
        }

        /// Return true if (assumptions and common and cond) -> (predicate), where cond is constructed from x=y.
        fn cell_collapse_guard_equal(
            var_common: Var,
            var_x: Var,
            var_y: Var,
            var_predicate: Var,
        ) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            cell_collapse_guard_fn(var_common, var_x, var_y, var_predicate, |x_trm, y_trm| {
                format!("(= {} {})", x_trm, y_trm)
            })
        }

        /// Return true if (assumptions and common and cond) -> (predicate), where cond is constructed from x<y.
        fn cell_collapse_guard_lt(
            var_common: Var,
            var_x: Var,
            var_y: Var,
            var_predicate: Var,
        ) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            cell_collapse_guard_fn(var_common, var_x, var_y, var_predicate, |x_trm, y_trm| {
                format!("(< {} {})", x_trm, y_trm)
            })
        }

        /// Return true if (assumptions and common and cond) -> (predicate), where cond is constructed from x, y, and cond_fml.
        fn cell_collapse_guard_fn(
            var_common: Var,
            var_x: Var,
            var_y: Var,
            var_predicate: Var,
            cond_fml: impl Fn(String, String) -> String,
        ) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            move |egraph, _, subst| {
                let common = subst[var_common];
                let x = subst[var_x];
                let y = subst[var_y];
                let predicate = subst[var_predicate];
                let extractor = Extractor::new(&egraph, AstSize);
                let common_fml = extractor.find_best(common).1.to_string();
                let x_trm = extractor.find_best(x).1.to_string();
                let y_trm = extractor.find_best(y).1.to_string();
                let cond_fml = cond_fml(x_trm, y_trm);
                let predicate_fml = extractor.find_best(predicate).1.to_string();
                let assumptions = unsafe { ASSUMPTIONS.clone() };
                z3utils::imply(
                    format!("(and {} {})", cond_fml, assumptions),
                    format!("(= {} {})", common_fml, predicate_fml),
                )
            }
        }

        /// Return true if (assumptions and a) -> (b | x<=y).
        fn redundancy_elimination_leq(
            var_a: Var,
            var_b: Var,
            var_x: Var,
            var_y: Var,
        ) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            redundancy_elimination_fn(var_a, var_b, var_x, var_y, |x_trm, y_trm| {
                format!("(<= {} {})", x_trm, y_trm)
            })
        }

        /// Return true if (assumptions and a) -> (b | x>y).
        fn redundancy_elimination_gt(
            var_a: Var,
            var_b: Var,
            var_x: Var,
            var_y: Var,
        ) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            redundancy_elimination_fn(var_a, var_b, var_x, var_y, |x_trm, y_trm| {
                format!("(> {} {})", x_trm, y_trm)
            })
        }

        // Return true if (assumptions and a) -> (b | f(x, y)).
        fn redundancy_elimination_fn(
            var_a: Var,
            var_b: Var,
            var_x: Var,
            var_y: Var,
            op_fn: impl Fn(String, String) -> String,
        ) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
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
                z3utils::imply(
                    format!("(and {} {})", a_fml, assumptions),
                    format!("(or {} {})", b_fml, op_fn(x_trm, y_trm)),
                )
            }
        }

        /// Return true if a and y are not 0, or b contains no divide by x or divide by y.
        fn no_divide_by_0(
            var_x: Var,
            var_y: Var,
            var_b: Var,
        ) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            move |egraph, _, subst| {
                let x = subst[var_x];
                let y = subst[var_y];
                let extractor = Extractor::new(&egraph, AstSize);
                let x_trm = extractor.find_best(x).1.to_string();
                let y_trm = extractor.find_best(y).1.to_string();
                if y_trm != "0".to_string() && x_trm != "0".to_string() {
                    return true;
                }
                let b = subst[var_b];
                let b_trm = extractor.find_best(b).1.to_string();
                // Build an egraph out of b.
                let mut egraph_copy = EGraph::<PropLang, ()>::default();
                let _ = egraph_copy.add_expr(&b_trm.parse().unwrap());
                egraph_copy.rebuild();

                let divide_by_x: Pattern<PropLang> = format!("(^ {} (-1))", x_trm).parse().unwrap(); //format!("(/ ?a {})", x_trm).parse().unwrap();
                let divide_by_y: Pattern<PropLang> = format!("(^ {} (-1))", y_trm).parse().unwrap();
                let matches_x = divide_by_x.search_with_limit(&egraph_copy, 2);
                let matches_y = divide_by_y.search_with_limit(&egraph_copy, 2);
                // If no matches, return true.
                let num_matched_x = matches_x.iter().count();
                let num_matched_y = matches_y.iter().count();
                (num_matched_x == 0 || y_trm != "0") && (num_matched_y == 0 || x_trm != "0")
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
            // rewrite!("and-distrib"; "(and ?a (or ?b ?c))" => "(or (and ?a ?b) (and ?a ?c))"),
            // rewrite!("or-distrib"; "(or ?a (and ?b ?c))" => "(and (or ?a ?b) (or ?a ?c))"),
            // rewrite!("and-factor"; "(or (and ?a ?b) (and ?a ?c))" => "(and ?a (or ?b ?c))"),
            // rewrite!("or-factor"; "(and (or ?a ?b) (or ?a ?c))" => "(or ?a (and ?b ?c))"),
            // If a formula is implied by the assumptions, replace it with true.
            // rewrite!("assumption-true";"?a" => "true"
            //     if implied_by_assumptions()),
            // If not(a) is implied by the assumptions, replace it with false.
            // rewrite!("assumption-false";"?a" => "false"
            //     if neg_implied_by_assumptions()),
            // Arithmetic Rules.
            // rewrite!("div-one"; "(/ ?x 1)" => "?x"),
            // rewrite!("cancel-denominator"; "(* (/ ?a ?b) ?b)" => "?a"),
            // rewrite!("times-zero"; "(* ?a 0)" => "0"),
            // // Abbreviation rules.
            // rewrite!("not-true"; "(not true)" => "false"),
            // rewrite!("not-false"; "(not false)" => "true"),
            // rewrite!("not-not"; "(not (not ?a))" => "?a"),
            rewrite!("less-equals"; "(or (< ?a ?b) (= ?a ?b))" => "(<= ?a ?b)"),
            rewrite!("greater-equals"; "(or (> ?a ?b) (= ?a ?b))" => "(>= ?a ?b)"),
            rewrite!("comparison-true-lt"; "(or (< ?a ?b) (>= ?a ?b))" => "true"),
            rewrite!("comparison-true-gt"; "(or (> ?a ?b) (<= ?a ?b))" => "true"),
            rewrite!("equal-commutes"; "(= ?a ?b)" => "(= ?b ?a)"),
            // // Special reduction rules.
            // rewrite!("subexp-1"; "(or (and a b) (and c b))" => "(and b (or a c))"),
            // Cell Collapse Rules.
            // rewrite!("cell-collapse-generic"; "(or (and ?a ?b) (and ?c ?d))" => "(and (or ?a ?c) ?b)"
            // if cell_collapse_guard(var("?b"), var("?c"), var("?d"))),
            // Casing on symbol split.
            rewrite!("cell-collapse-special-equal-1"; "(or (and (= ?x ?y) ?b) (and (> ?x ?y) ?d))" => "(and (>= ?x ?y) ?d)"
                if cell_collapse_guard_equal(var("?d"), var("?x"), var("?y"), var("?b"))
                if no_divide_by_0(var("?x"), var("?y"), var("?d"))),
            rewrite!("cell-collapse-special-equal-2"; "(or (and (= ?x ?y) ?b) (and (< ?x ?y) ?d))" => "(and (<= ?x ?y) ?d)"
                if cell_collapse_guard_equal(var("?d"), var("?x"), var("?y"), var("?b"))
                if no_divide_by_0(var("?x"), var("?y"), var("?d"))),
            rewrite!("cell-collapse-special-lessthan"; "(or (and (< ?x ?y) ?b) (and (>= ?x ?y) ?d))" => "?d"
                if cell_collapse_guard_lt(var("?d"), var("?x"), var("?y"), var("?b"))),
            rewrite!("cell-collapse-special-greaterthan"; "(or (and (> ?x ?y) ?b) (and (<= ?x ?y) ?d))" => "?d"
                if cell_collapse_guard_lt(var("?d"), var("?y"), var("?x"), var("?b"))),
            // Removing redundant disjuncts.
            rewrite!("redundant-casing"; "(and (or ?a (> ?x ?y)) (or ?b (<= ?x ?y)))" => "(or ?a ?b)"
                if redundancy_elimination_leq(var("?a"), var("?b"), var("?x"), var("?y"))
                if redundancy_elimination_gt(var("?b"), var("?a"), var("?x"), var("?y"))),
        ]
    }

    /// This function returns the simplification for a given formula.
    ///
    /// # Parameters
    ///
    /// - 'problem': The problem to be simplified. Must be a `String` value.
    /// - 'assumptions': The assumptions to be associated with the problem.
    ///
    /// # Returns
    ///
    /// A `String` of the simplified problem.

    fn simplify(problem: String, assumptions: String) -> String {
        unsafe { ASSUMPTIONS = assumptions };

        base::simplify(problem, true, config::TIMEOUT, Self::make_rules())
    }
}
