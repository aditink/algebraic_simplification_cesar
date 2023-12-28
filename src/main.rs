use std::env;
mod cesar;

fn main() {
    // More info when there are errors.
    env::set_var("RUST_BACKTRACE", "full");

    // If command line flag -c is present, run cesar tests.
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "-c" {
        cesar::tests::run_all();
    }

    // If command line flag -s is present, we expect two further arguments:
    // The first is the expression to simplify, the second is assumptions.
    else if args.len() > 1 && args[1] == "-s" {
        if args.len() < 4 {
            println!("Expected two arguments after -s: expression and assumptions.");
            return;
        }
        let expr = args[2].clone();
        let assumptions = args[3].clone();
        cesar::simplify::simplify(expr, assumptions);
    }

    // If the command line flag -l is present, we expect two further arguments:
    // The first is the expression to simplify, the second is assumptions.
    else if args.len() > 1 && args[1] == "-l" {
        if args.len() < 4 {
            println!("Expected two arguments after -l: expression and assumptions.");
            return;
        }
        let expr = args[2].clone();
        let assumptions = args[3].clone();
        cesar::simplify::light_simplify(expr, assumptions);
    }

    // If the command line flag -a is present, we expect two further arguments:
    // The first is the expression to simplify, the second is assumptions.
    else if args.len() > 1 && args[1] == "-a" {
        if args.len() < 4 {
            println!("Expected two arguments after -a: expression and assumptions.");
            return;
        }
        let expr = args[2].clone();
        let assumptions = args[3].clone();
        cesar::simplify::aggressive_simplify(expr, assumptions);
    }

    // If the command line flag -r is present, we expect two further arguments:
    // The first is the expression to rearrange, the second is the numerator variable.
    else if args.len() > 1 && args[1] == "-r" {
        if args.len() < 4 {
            println!("Expected two arguments after -r: expression and numerator variable.");
            return;
        }
        let expr = args[2].clone();
        let numerator_var = args[3].clone();
        cesar::simplify::rearrange(expr, numerator_var);
    }

    else{
        println!("Expected a command line flag.");
    }
}
