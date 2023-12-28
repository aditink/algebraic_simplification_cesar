
use crate::cesar::config;
use crate::cesar::pass1::Pass1;
use crate::cesar::pass2::Pass2;
use crate::cesar::pass3::Pass3;
use crate::cesar::pass4::Pass4;
use crate::cesar::pass5::Pass5;
use crate::cesar::pass6::Pass6;
use crate::cesar::rearrange_pass::RearrangePass;
use crate::cesar::z3utils;

pub fn simplify(expr: String, assumptions: String) {
    
    let result1 = Pass1::simplify(expr, assumptions.clone());
    let result2 = Pass2::simplify(result1, assumptions.clone());
    let result3 = Pass3::simplify(result2, assumptions.clone());
    let result4 = Pass4::simplify(result3, assumptions.clone());
    let result5 = Pass5::simplify(result4, assumptions.clone());

    println!("{}", result5);
}

/// Use z3 to double check that an initial and final expression are equivalent 
/// under assumptions.
fn check_equiv(initial: String, fin: String, assumptions: String) -> bool {
    let assumptions = assumptions.parse().unwrap();
    let result = z3utils::imply(assumptions, format!("(= {} {})", initial, fin));
    result
}

/// simplify + check for general case redundant disjunct and conjuncts.
pub fn aggressive_simplify(expr: String, assumptions: String) {

    let mut result1 = Pass1::simplify(expr.clone(), assumptions.clone());
    // If the result is not equivalent to the original, then return the original.
    if !check_equiv(expr.clone(), result1.clone(), assumptions.clone()) {
        if config::DEBUG {
            println!("Simplify Pass1 failed.");
        }
        result1 = expr
    }

    let mut result2 = Pass2::simplify(result1.clone(), assumptions.clone());
    // If the result is not equivalent to the original, then return the original.
    if !check_equiv(result1.clone(), result2.clone(), assumptions.clone()) {
        if config::DEBUG {
            println!("Simplify Pass2 failed.");
        }
        result2 = result1
    }

    let mut result3 = Pass3::simplify(result2.clone(), assumptions.clone());
    // If the result is not equivalent to the original, then return the original.
    if !check_equiv(result2.clone(), result3.clone(), assumptions.clone()) {
        if config::DEBUG {
            println!("Simplify Pass3 failed.");
        }
        result3 = result2
    }

    let mut result4 = Pass4::simplify(result3.clone(), assumptions.clone());
    // If the result is not equivalent to the original, then return the original.
    if !check_equiv(result3.clone(), result4.clone(), assumptions.clone()) {
        if config::DEBUG {
            println!("Simplify Pass4 failed.");
        }
        result4 = result3
    }

    let mut result5 = Pass5::simplify(result4.clone(), assumptions.clone());
    // If the result is not equivalent to the original, then return the original.
    if !check_equiv(result4.clone(), result5.clone(), assumptions.clone()) {
        if config::DEBUG {
            println!("Simplify Pass5 failed.");
        }
        result5 = result4
    }

    let mut result6 = Pass6::simplify(result5.clone(), assumptions.clone());
    // If the result is not equivalent to the original, then return the original.
    if !check_equiv(result5.clone(), result6.clone(), assumptions.clone()) {
        if config::DEBUG {
            println!("Simplify Pass6 failed.");
        }
        result6 = result5
    }

    // if config::DEBUG {
    //     println!("Passes succeeded.");
    // }

    println!("{}", result6);
}

/// A function to clean up bad things like 0<0.
pub fn light_simplify(expr: String, assumptions: String) {
    
    let result = Pass5::simplify(expr, assumptions.clone());

    println!("{}", result);
}

/// A function to rewrite expr with numerator_var always in the numerator.
pub fn rearrange(expr:String, numerator_var: String) {
    let result = RearrangePass::rearrange(expr, numerator_var);
    println!("{}", result);
}