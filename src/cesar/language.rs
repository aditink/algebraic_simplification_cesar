use egg::*;
use ordered_float::NotNan;

pub type Constant = NotNan<f64>;
pub type VariableName = String;

define_language! {
    pub enum PropLang {
        // string variants with an array of child `Id`s (any static size)
        // any type that implements LanguageChildren may be used here
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
        "^" = Pow([Id; 2]),
        "<=" = Le([Id; 2]),
        "<" = Lt([Id; 2]),
        ">=" = Ge([Id; 2]),
        ">" = Gt([Id; 2]),
        "=" = Eq([Id; 2]),
        "and" = And([Id; 2]),
        "or" = Or([Id; 2]),
        "not" = Not([Id; 1]),
        "distinct" = Distinct([Id; 2]),
        "-"  = Neg(Id),
        Num(Constant),
        "true" = True,
        "false" = False,
        // language items are parsed in order, and we want symbol to
        // be a fallback, so we put it last
        Variable(VariableName),
        // This is the ultimate fallback, it will parse any operator 
        // (as a string) and any number of children.
        // Note that if there were 0 children, the previous branch would have 
        // succeeded.
        Other(Symbol, Vec<Id>),
    }
}

/// The set of logical operators and comparison operators.
// pub static FORMULA_OPERATORS: [&str; 12] = [
//     "and", "or", "not", "distinct", "true", "false", "=>", "=", ">=", "<=", "<", ">"
// ];

// pub static ARITHMETIC_OPERATORS: [&str; 5] = ["+", "-", "*", "/", "^"];

/// The set of reserved symbols: union of arithmetic operators and logical operators.
pub static RESERVED_SYMBOLS: [&str; 17] = [
    "+", "-", "*", "/", "^", "and", "or", "not", "distinct", "true", "false", "=>", "=", ">=",
    "<=", "<", ">",
];

// #[derive(PartialEq)]
// pub enum ExpressionType {
//     Formula,
//     Term,
//     Other
// }

// fn get_first_op(expr: &str) -> &str {
//     // If string does not begin with a parenthesis, return the empty string.
//     if !expr.starts_with("(") {
//         return "";
//     }
//     // Otherwise, find the first token.
//     let first_token = expr.split_once(" ").unwrap().0;
//     // Remove the opening parenthesis.
//     first_token.trim_start_matches("(")
// }

// pub fn get_expression_type(expr: &str) -> ExpressionType {
//     let first_op = get_first_op(expr);
//     if FORMULA_OPERATORS.contains(&first_op) {
//         ExpressionType::Formula
//     } else if ARITHMETIC_OPERATORS.contains(&first_op) {
//         ExpressionType::Term
//     } else {
//         ExpressionType::Other
//     }
// }