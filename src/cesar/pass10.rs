use crate::cesar::{language::PropLang, z3utils};
use crate::cesar::simplification::Simplification;
use egg::*;

/// Another disjunct elimination heuristic that rarely applies but gives a lot of simplification when it does:
/// ?x<?y & ?b | ?x>=?y & ?c -> ?b if  assumptions -> (b equiv original)
pub struct Pass10;

pub static mut ASSUMPTIONS: String =  String::new();

fn var(s: &str) -> Var {
    s.parse().unwrap()
}

impl Simplification for Pass10 {

    // reference: https://docs.rs/egg/latest/egg/macro.rewrite.html.
    fn make_rules() -> Vec<Rewrite<PropLang, ()>> {

        fn equiv_lt(var_x: Var, var_y:Var, var_b:Var, var_c:Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            equiv(var_x, var_y, var_b, var_c, |x, y| { format!("(< {} {})", x, y) }, |x, y| { format!("(>= {} {})", x, y) })
        }

        fn equiv_gt(var_x: Var, var_y:Var, var_b:Var, var_c:Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            equiv(var_x, var_y, var_b, var_c, |x, y| { format!("(> {} {})", x, y) }, |x, y| { format!("(<= {} {})", x, y) })
        }

        fn equiv_leq(var_x: Var, var_y:Var, var_b:Var, var_c:Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            equiv(var_x, var_y, var_b, var_c, |x, y| { format!("(<= {} {})", x, y) }, |x, y| { format!("(> {} {})", x, y) })
        }

        fn equiv_geq(var_x: Var, var_y:Var, var_b:Var, var_c:Var) -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            equiv(var_x, var_y, var_b, var_c, |x, y| { format!("(>= {} {})", x, y) }, |x, y| { format!("(< {} {})", x, y) })
        }

        fn equiv(var_x: Var, var_y:Var, var_b:Var, var_c:Var,
            op_fn_1: impl Fn(String, String) -> String, op_fn_2: impl Fn(String, String) -> String)
            -> impl Fn(&mut EGraph<PropLang, ()>, Id, &Subst) -> bool {
            move |egraph, _, subst| {
                let x = subst[var_x];
                let y = subst[var_y];
                let b = subst[var_b];
                let c = subst[var_c];
                let extractor = Extractor::new(&egraph, AstSize);
                let x_trm = extractor.find_best(x).1.to_string();
                let y_trm = extractor.find_best(y).1.to_string();
                let b_fml = extractor.find_best(b).1.to_string();
                let c_fml = extractor.find_best(c).1.to_string();
                let assumptions = unsafe { ASSUMPTIONS.clone() };
                let inequality_1 = op_fn_1(x_trm.clone(), y_trm.clone());
                let inequality_2 = op_fn_2(x_trm, y_trm);
                let original_fml = format!("(or (and {} {}) (and {} {}))", inequality_1, b_fml, inequality_2, c_fml);
                let equiv_fml = format!("(= {} {})", original_fml, b_fml);
                z3utils::imply(assumptions, equiv_fml)
            }
        }

        vec![
            rewrite!("elim-lt"; "(or (and (< ?x ?y) ?b) (and (>= ?x ?y) ?c))" => "?b"
                if equiv_lt(var("?x"), var("?y"), var("?b"), var("?c"))),
            rewrite!("elim-gt"; "(or (and (> ?x ?y) ?b) (and (<= ?x ?y) ?c))" => "?b"
                if equiv_gt(var("?x"), var("?y"), var("?b"), var("?c"))),
            rewrite!("elim-leq"; "(or (and (<= ?x ?y) ?b) (and (> ?x ?y) ?c))" => "?b"
                if equiv_leq(var("?x"), var("?y"), var("?b"), var("?c"))),
            rewrite!("elim-geq"; "(or (and (>= ?x ?y) ?b) (and (< ?x ?y) ?c))" => "?b"
                if equiv_geq(var("?x"), var("?y"), var("?b"), var("?c"))),
            // Commute the or, then rewrite the rules.
            rewrite!("elim-lt-comm"; "(or (and (>= ?x ?y) ?c) (and (< ?x ?y) ?b))" => "?b"
                if equiv_lt(var("?x"), var("?y"), var("?b"), var("?c"))),
            rewrite!("elim-gt-comm"; "(or (and (<= ?x ?y) ?c) (and (> ?x ?y) ?b))" => "?b"
                if equiv_gt(var("?x"), var("?y"), var("?b"), var("?c"))),
            rewrite!("elim-leq-comm"; "(or (and (> ?x ?y) ?c) (and (<= ?x ?y) ?b))" => "?b"
                if equiv_leq(var("?x"), var("?y"), var("?b"), var("?c"))),
            rewrite!("elim-geq-comm"; "(or (and (< ?x ?y) ?c) (and (>= ?x ?y) ?b))" => "?b"
                if equiv_geq(var("?x"), var("?y"), var("?b"), var("?c"))),
            // And commutes, applied when there is something like (<= ?x ?y).
            rewrite!("and-comm-leq"; "(and ?b (<= ?x ?y))" => "(and (<= ?x ?y) ?b)"),
            rewrite!("and-comm-geq"; "(and ?b (>= ?x ?y))" => "(and (>= ?x ?y) ?b)"),
            rewrite!("and-comm-lt"; "(and ?b (< ?x ?y))" => "(and (< ?x ?y) ?b)"),
            rewrite!("and-comm-gt"; "(and ?b (> ?x ?y))" => "(and (> ?x ?y) ?b)"),
        ]
    }
}
