// Module: cesar exposes tests.rs.
// Rules for intersection benchmark.
// mod intersection;
// First pass.
pub mod pass1;
// Second pass for intersection.
pub mod pass2;
// Third pass that does redundant or case elimination.
pub mod pass3;
// Fourth pass that does aggressive or redundancy elimination.
pub mod pass4;
// Fifth pass cleans up things like x=x.
pub mod pass5;
// Sixth pass that eliminates redundant conjuncts.
pub mod pass6;
// Seventh pass that does multiplication distribution, i.e. the transformation x*a + x*b = x*(a+b).
pub mod pass7;
// Eighth pass that moves division to multiplication positions accounting for sign.
pub mod pass8;
// Ninth pass that does more aggressive disjunct elimination.
pub mod pass9;
// Tenth pass tries another case of disjunct elimination.
pub mod pass10;
// Rearrange pass that moves a variable to numerator.
pub mod base;
pub mod rearrange_pass;
