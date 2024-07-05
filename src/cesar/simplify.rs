
use crate::cesar::config;
use crate::cesar::pass1::Pass1;
use crate::cesar::pass10::Pass10;
use crate::cesar::pass2::Pass2;
use crate::cesar::pass3::Pass3;
use crate::cesar::pass4::Pass4;
use crate::cesar::pass5::Pass5;
use crate::cesar::pass6::Pass6;
use crate::cesar::pass7::Pass7;
use crate::cesar::pass8::Pass8;
use crate::cesar::pass9::Pass9;
use crate::cesar::base_pass::BasePass;
use crate::cesar::rearrange_pass::RearrangePass;
use crate::cesar::z3utils;

pub fn simplify(expr: String, assumptions: String) {
    
    let result1 = Pass1::simplify(expr, assumptions.clone(), false, 1);
    let result2 = Pass2::simplify(result1, assumptions.clone(), true, 1);
    let result3 = Pass3::simplify(result2, assumptions.clone(), false, 1);
    let result4 = Pass4::simplify(result3, assumptions.clone(), true, 0);
    let result5 = Pass5::simplify(result4, assumptions.clone(), false, 1);

    println!("{}", result5);
}

/// Use z3 to double check that an initial and final expression are equivalent 
/// under assumptions.
fn check_equiv(initial: String, fin: String, assumptions: String) -> bool {
    let assumptions = assumptions.parse().unwrap();
    let result = z3utils::imply(assumptions, format!("(= {} {})", initial, fin));
    result
}

fn store_if_equiv(old_expr: String, assumptions: String, has_node_limit: bool, timeout_multiplier: u64,
    pass: impl Fn(String, String, bool, u64) -> String) -> String {
    let mut result = pass(old_expr.clone(), assumptions.clone(), has_node_limit, timeout_multiplier);
    if config::DEBUG {
        println!("{}", result.clone());
    }
    // If the result is not equivalent to the original, then return the original.
    if !check_equiv(old_expr.clone(), result.clone(), assumptions.clone()) {
        if config::DEBUG {
            println!("Simplify pass failed.")
        }
        result = old_expr;
    }
    result
}

/// simplify + check for general case redundant disjunct and conjuncts.
pub fn aggressive_simplify(expr: String, assumptions: String) -> String{

    let result1 = store_if_equiv(expr.clone(),
        assumptions.clone(), false, 1, Pass1::simplify);

    let result2 = store_if_equiv(result1.clone(),
        assumptions.clone(), true, 1, Pass2::simplify);
        
    let result3 = store_if_equiv(result2.clone(),
        assumptions.clone(), true, 1, Pass3::simplify);
    
    let result4 = store_if_equiv(result3.clone(),
        assumptions.clone(), true, 1, Pass4::simplify); // Disabled for now.
    
    let result5 = store_if_equiv(result4.clone(),
        assumptions.clone(), false, 1, Pass5::simplify);

    let result6 = store_if_equiv(result5.clone(),
        assumptions.clone(), true, 3, Pass6::simplify);

    let result7 = store_if_equiv(result6.clone(),
        assumptions.clone(), true, 3,  Pass7::simplify);

    let result8 = store_if_equiv(result7.clone(),
        assumptions.clone(), true, 3, Pass8::simplify);

    // Another round of eliminating conjuncts.
    let result6_2 = store_if_equiv(result8.clone(),
        assumptions.clone(), true, 3, Pass6::simplify);

    let result9 = store_if_equiv(result6_2.clone(),
        assumptions.clone(), true, 3, Pass9::simplify);

    let result10 = store_if_equiv(result9.clone(),
        assumptions.clone(), true, 3, Pass10::simplify);


    if config::DEBUG {
        println!("Passes succeeded.");
    }

    println!("{}", result10);

    return result10;
}

/// A function to clean up bad things like 0<0.
pub fn light_simplify(expr: String, assumptions: String) {
    
    let result = Pass5::simplify(expr, assumptions.clone(), false, 1);

    println!("{}", result);
}

/// A function to rewrite expr with numerator_var always in the numerator.
pub fn rearrange(expr:String, numerator_var: String) {
    let result = RearrangePass::rearrange(expr, numerator_var);
    println!("{}", result);
}
