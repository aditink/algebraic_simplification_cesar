use std::time::Instant;

use crate::cesar::simplify;

fn aggressive_simplify_tests() {
    let problem = "(or (and (>= absorbed minAbsorbed) (< discharged maxDischarge)) (or (and (>= absorbed minAbsorbed) (and (< (+ discharged (* F T)) maxDischarge) (<= gt 0))) (and (> gt (* 2 T)) (or (and (< (* gt (+ (+ discharged (* F gt)) (* -1 maxDischarge))) 0) (and (< maxDischarge discharged) (or (>= absorbed minAbsorbed) (<= (+ (* (+ absorbed (* -1 minAbsorbed)) (^ (+ gt (* -1 T)) -1)) (* (* c F) tempDiff)) 0)))) (and (> maxDischarge discharged) (or (and (or (>= absorbed minAbsorbed) (>= (+ (* (+ absorbed (* -1 minAbsorbed)) (^ (+ gt (* -1 T)) -1)) (* (* c F) tempDiff)) 0)) (or (< (* gt (+ (+ discharged (* F gt)) (* -1 maxDischarge))) 0) (< (* (+ gt T) (+ (+ discharged (* -1 maxDischarge)) (* F (+ gt T)))) 0))) (and (< (* (+ (* -1 discharged) maxDischarge) (^ (+ gt (* -1 T)) -1)) F) (and (< (+ discharged (* F T)) maxDischarge) (or (>= absorbed minAbsorbed) (> tempDiff (* (+ absorbed (* -1 minAbsorbed)) (^ (+ (+ (* c discharged) (* (* -1 c) maxDischarge)) (* (* c F) T)) -1))))))))))))".to_string();
    let assumptions = "(and (> T 0) (and (> F 0) (and (> minAbsorbed 0) (and (> maxDischarge 0) (and (> tempDiff 0) (> c 0))))))".to_string();
    print!("Problem: {}\n", problem);
    simplify::aggressive_simplify(problem, assumptions.clone());
}

pub fn run_all() {
    println!("Running cesar tests...");
    // Start timer.
    let t = Instant::now();
    aggressive_simplify_tests();
    // Stop timer.
    let elapsed = t.elapsed();
    println!("Cesar tests took: {:?}", elapsed);
    println!("Cesar tests done.");
}

#[cfg(test)]
mod tests {
    use crate::cesar::simplify;

    #[test]
    fn test_aggressive_simplify() {
        // input formula: (and (or (> B (* (^ (+ (* 2 e) (* (- 2) p)) (- 1)) (^ v 2))) (<= v 0)) (> e p))
        // input assumptions: (and (> A 0) (and (> B 0) (and (> T 0) (>= v 0))))
        // expected output: (> (* B (* 2 (- e p))) (^ v 2))
        let problem = "(and (or (> B (* (^ (+ (* 2 e) (* (- 2) p)) (- 1)) (^ v 2))) (<= v 0)) (> e p))".to_string();
        let assumptions = "(and (> A 0) (and (> B 0) (and (> T 0) (>= v 0))))".to_string();
        let expected_output = "(> (* B (* 2 (- e p))) (^ v 2))".to_string();
        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }
}