// Module: cesar exposes tests.rs.
pub mod tests;
pub mod simplify;
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
// Rearrange pass that moves a variable to numerator.
mod rearrange_pass;
// Z3 utility like checking implication.
mod z3utils;
// Configuration values.
mod config;