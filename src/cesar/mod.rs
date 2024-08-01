// Module: cesar exposes tests.rs.
pub mod simplify;
pub mod tests;
// Language definition.
mod language;
// Rules for intersection benchmark.
// mod intersection;
// First pass.
mod pass1;
// Second pass for intersection.
mod pass2;
// Third pass that does redundant or case elimination.
mod pass3;
// Fourth pass that does aggressive or redundancy elimination.
mod pass4;
// Fifth pass cleans up things like x=x.
mod pass5;
// Sixth pass that eliminates redundant conjuncts.
mod pass6;
// Seventh pass that does multiplication distribution, i.e. the transformation x*a + x*b = x*(a+b).
mod pass7;
// Eighth pass that moves division to multiplication positions accounting for sign.
mod pass8;
// Ninth pass that does more aggressive disjunct elimination.
mod pass9;
// Tenth pass tries another case of disjunct elimination.
mod pass10;
// Rearrange pass that moves a variable to numerator.
mod rearrange_pass;
// Z3 utility like checking implication.
mod z3utils;
// Configuration values.
mod config;
// Base functions
mod base;
