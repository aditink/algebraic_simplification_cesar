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
    fn test_aggressive_simplify_case_1() {
        // input formula: (and (or (> B (* (^ (+ (* 2 e) (* (- 2) p)) (- 1)) (^ v 2))) (<= v 0)) (> e p))
        // input assumptions: (and (> A 0) (and (> B 0) (and (> T 0) (>= v 0))))
        // expected output: (> (* B (* 2 (- e p))) (^ v 2))
        let problem = "(and (or (> B (* (^ (+ (* 2 e) (* (- 2) p)) (- 1)) (^ v 2))) (<= v 0)) (> e p))".to_string();
        let assumptions = "(and (> A 0) (and (> B 0) (and (> T 0) (>= v 0))))".to_string();
        let expected_output = "(> (* B (* 2 (- e p))) (^ v 2))".to_string();
        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }
    #[test]
    fn test_aggressive_simplify_case_2() {
        let problem = "(< (+ (* A T) v) (^ (* (* B (^ (+ A B) (- 1))) (+ (+ (* (* 2 A) e) (* (* (- 2) A) p)) (^ v 2))) (/ 1 2)))".to_string();
        let assumptions = "(and (> (* B (* 2 (- e p))) (^ v 2)) (and (> A 0) (and (> B 0) (and (> T 0) (>= v 0)))))".to_string();
        let expected_output = "(< (+ (* A T) v) (^ (* B (* (^ (+ A B) (- 1)) (+ (* (* A 2) (- e p)) (^ v 2)))) (/ 1 2)))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_3() {
        let problem = "(> (+ (+ e (* (* (/ 1 2) B) (^ t 2))) (* (* (- 1) t) v)) p)".to_string();
        let assumptions = "(and (or (<= B (* (^ t (- 1)) v)) (<= t 0)) (and (> A 0) (and (> B 0) (and (> T 0) (and (>= v 0) (>= t 0))))))".to_string();
        let expected_output = "(> (- (+ e (* (/ 1 2) (* B (^ t 2)))) (* t v)) p)".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_4() {
        let problem = "(> (* B (+ (+ (* 2 e) (* (- 2) p)) (* (* (- 1) t) (+ (* A t) (* 2 v))))) (^ (+ (* A t) v) 2))".to_string();
        let assumptions = "(and (>= t 0) (and (>= v 0) (and (> A 0) (and (> B 0) (and (<= t T) (> T 0))))))".to_string();
        let expected_output = "(> (* B (- (* 2 (- e p)) (* t (+ (* t A) (* 2 v))))) (^ (+ (* t A) v) 2))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_5() {
        let problem = "(or (and (= e 0) (or (and (= v 0) (< (+ (* (* 2 B) p) (* (* A (+ A B)) (^ T 2))) 0)) (and (> v 0) (< (+ (+ (* (* 2 B) p) (^ (+ (* A T) v) 2)) (* (* B T) (+ (* A T) (* 2 v)))) 0)))) (and (distinct e 0) (or (and (= v 0) (< (+ p (* (* (* (* (/ 1 2) A) (^ B (- 1))) (+ A B)) (^ T 2))) e)) (and (> v 0) (< (+ (^ (+ (* A T) v) 2) (* B (+ (+ (+ (* (- 2) e) (* 2 p)) (* A (^ T 2))) (* (* 2 T) v)))) 0))))".to_string();
        let assumptions = "(and (> (* 2 (* B (- e p))) (^ v 2)) (and (> A 0) (and (> B 0) (and (> T 0) (>= v 0)))))".to_string();
        let expected_output = "(or (and (= e 0) (< (+ (+ (* 2 (* B p)) (^ (+ (* A T) v) 2)) (* (* B T) (+ (* A T) (* 2 v)))) 0)) (and (distinct e 0) (< (+ (^ (+ (* A T) v) 2) (* B (+ (+ (* 2 (- p e)) (* A (^ T 2))) (* v (* 2 T))))) 0)))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_6() {
        let problem = "(or (and (< (+ (+ Tx (* (* 2 T) V)) y) (+ Ty x)) (or (and (= Tx x) (> Ty y)) (or (and (> (+ Tx Ty) (+ x y)) (< Tx x)) (and (> Tx x) (and (> x 0) (< (+ Tx y) (+ Ty x))))))) (or (and (= x 0) (< (+ Tx y) Ty)) (or (and (< Tx x) (and (< (+ Tx (* T V)) x) (<= (+ Tx Ty) (+ x y)))) (or (and (< (+ (+ Tx x) y) Ty) (or (and (> (+ Tx x) 0) (< x 0)) (or (<= Tx x) (> x 0)))) (or (< (+ Tx x) 0) (and (> Ty y) (<= (+ Tx x) 0)))))))".to_string();
        let assumptions = "(and (or (and (< y Ty) (or (> Ty (+ (+ Tx x) y)) (> (+ Ty x) (+ Tx y)))) (or (< Tx x) (< (+ Tx x) 0))) (and (> Tx 0) (and (> Ty 0) (and (> V 0) (> T 0)))))".to_string();
        let expected_output = "(or (and (< (+ (+ Tx (* 2 (* T V))) y) (+ Ty x)) (or (and (> Tx x) (> x 0)) (and (> (+ Tx Ty) (+ x y)) (<= Tx x)))) (or (or (= x 0) (or (< (+ Tx x) 0) (and (> Ty y) (<= (+ Tx x) 0)))) (or (and (< (+ (+ Tx x) y) Ty) (or (> x 0) (and (> (+ Tx x) 0) (< x 0)))) (and (< (+ Tx (* T V)) x) (<= (+ Tx Ty) (+ x y))))))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_7() {
        let problem = "(or (and (< (+ (+ (+ Tx (* (* 2 T) V)) x) y) Ty) (or (and (= (+ Tx x) 0) (> Ty y)) (or (and (> (+ Tx x) 0) (and (< x 0) (< (+ (+ Tx x) y) Ty))) (and (> (+ (+ Tx Ty) x) y) (< (+ Tx x) 0))))) (or (< Tx x) (or (and (< (+ Tx y) (+ Ty x)) (or (< x 0) (or (> Tx x) (<= (+ Tx x) 0)))) (or (and (< (+ Tx x) 0) (and (< (+ (+ Tx (* T V)) x) 0) (<= (+ (+ Tx Ty) x) y))) (and (<= Tx x) (> Ty y))))))".to_string();
        let assumptions = "(and (or (and (< y Ty) (or (> Ty (+ (+ Tx x) y)) (> (+ Ty x) (+ Tx y)))) (or (< Tx x) (< (+ Tx x) 0))) (and (> Tx 0) (and (> Ty 0) (and (> V 0) (> T 0)))))".to_string();
        let expected_output = "(or (and (< (+ (+ (+ Tx (* 2 (* T V))) x) y) Ty) (or (and (> (+ Tx x) 0) (< x 0)) (and (> (+ (+ Tx Ty) x) y) (<= (+ Tx x) 0)))) (or (< Tx x) (or (and (< (+ Tx y) (+ Ty x)) (> Tx x)) (or (and (< (+ (+ Tx (* T V)) x) 0) (<= (+ (+ Tx Ty) x) y)) (and (> Ty y) (<= Tx x))))))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_8() {
        let problem = "(or (and (= x 0) (< (* T v) R)) (or (and (< (* T v) (+ R x)) (and (or (<= Tx x) (< (+ Tx x) 0))) (and (> Ty y) (<= (+ Tx x) 0)))) (and (> x 0) (and (> Tx x) (or (and (< (* T v) (+ R x)) (or (and (> Tx x) (<= (+ Tx x) 0))) (and (>= Tx x) (> Ty y)))))))".to_string();
        let assumptions = "(and (or (and (< y Ty) (or (> Ty (+ (+ Tx x) y)) (> (+ Ty x) (+ Tx y)))) (or (< Tx x) (< (+ Tx x) 0))) (and (> Tx 0) (and (> Ty 0) (and (> V 0) (> T 0)))))".to_string();
        let expected_output = "(or (and (< (+ (+ Tx (* 2 (* T V))) y) (+ Ty x)) (or (and (> Tx x) (> x 0)) (and (> (+ Tx Ty) (+ x y)) (<= Tx x)))) (or (or (= x 0) (or (< (+ Tx x) 0) (and (> Ty y) (<= (+ Tx x) 0)))) (or (and (< (+ (+ Tx x) y) Ty) (or (> x 0) (and (> (+ Tx x) 0) (< x 0)))) (and (< (+ Tx (* T V)) x) (<= (+ Tx Ty) (+ x y))))))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_9() {
        let problem = "(or (and (> (+ (* 3000 (+ (- 51) (* 50 gt))) produced) 0) (and (> (+ (+ (* 7000000 gt) (* 48 produced)) stored) 7344000) (or (and (> gt 1) (and (< (* 35 gt) 36) (<= produced 3000))) (and (>= (* 35 gt) 36) (< (+ (* 437500 gt) (* 3 produced)) 459000))))) (and (or (>= (* 35 gt) 36) (> produced 3000)) (and (or (< (* 35 gt) 36) (>= (+ (* 437500 gt) (* 3 produced)) 459000)) (> stored 0))))".to_string();
        let assumptions = "(and (or (and (<= produced 3000) (or (and (> (+ (* 3000 (- (* 50 gt) 51)) produced) 0) (and (> (+ (+ (* gt 7000000) (* produced 48)) stored) 7344000) (< (+ (* gt 437500) (* produced 3)) 459000))) (and (> (+ (* gt 150000) produced) 3000) (> (+ (* produced 4) (* stored 3)) 612000)))) (and (> stored 0) (or (>= (+ (* gt 437500) (* produced 3)) 459000) (> produced 3000)))) (and (= T 1) (>= i 0)))".to_string();
        let expected_output = "(or (and (> (+ (* 3000 (- (* 50 gt) 51)) produced) 0) (or (and (> gt 1) (and (< (* gt 35) 36) (<= produced 3000))) (and (>= (* gt 35) 36) (< (+ (* gt 437500) (* produced 3)) 459000)))) (and (or (>= (* gt 35) 36) (> produced 3000)) (or (< (* gt 35) 36) (>= (+ (* gt 437500) (* produced 3)) 459000))))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_10() {
        let problem = "(or (and (> gt 1) (> (+ (* 3000 (+ (- 51) (* 50 gt))) produced) 0)) (or (and (>= (* 35 gt) 36) (>= (+ (* 437500 gt) (* 3 produced)) 459000)) (and (< (* 35 gt) 36) (> produced 3000))))".to_string();
        let assumptions = "(and (or (and (<= produced 3000) (or (and (> (+ (* 3000 (- (* 50 gt) 51)) produced) 0) (and (> (+ (+ (* gt 7000000) (* produced 48)) stored) 7344000) (< (+ (* gt 437500) (* produced 3)) 459000))) (and (> (+ (* gt 150000) produced) 3000) (> (+ (* produced 4) (* stored 3)) 612000)))) (and (> stored 0) (or (>= (+ (* gt 437500) (* produced 3)) 459000) (> produced 3000)))) (and (= T 1) (>= i 0)))".to_string();
        let expected_output = "(or (and (> gt 1) (> (+ (* 3000 (- (* gt 50) 51)) produced) 0)) (and (< (* gt 35) 36) (> produced 3000)))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_11() {
        let problem = "(or (and (> (+ (* 4 produced) (* 3 stored)) (+ 612000 (* (* (* 20 (+ (- 400) i)) i) t))) (and (<= produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))) (> (+ (* 150000 gt) produced) (+ 3000 (* (* (* 5 (+ (- 300) i)) (+ (- 100) i)) t))))) (and (> produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))) (> (+ stored (* slope t)) 0)))".to_string();
        let assumptions = "(and (= T 1) (and (>= i 0) (>= t 0)))".to_string();
        let expected_output = "(or (and (> (+ (* 150000 gt) produced) (+ 3000 (* t (* (* 5 (- i 300)) (- i 100))))) (and (> (+ (* 4 produced) (* 3 stored)) (+ 612000 (* t (* 20 (* i (- i 400)))))) (<= produced (+ 3000 (* t (* i (* (- i 400) 5))))))) (and (> produced (+ 3000 (* t (* i (* (- i 400) 5))))) (> (+ stored (* slope t)) 0)))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_12() {
        let problem = "(or (and (> (+ (* 4 produced) (* 3 stored)) (+ 612000 (* (* 20 (+ (- 1050000) (* (+ (- 400) i) i))) t))) (and (> (+ (* 150000 gt) produced) (+ 3000 (* (* (* 5 (+ (- 300) i)) (+ (- 100) i)) t))) (<= produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))))) (and (> produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))) (> (+ stored (* 7000000 t)) 0)))".to_string();
        let assumptions = "(and (= T 1) (and (>= i 0) (and (>= s 0) (and (<= s t) (<= t (+ s T))))))".to_string();
        let expected_output = "(or (and (> (+ (* 4 produced) (* 3 stored)) (+ 612000 (* (- (* i (- i 400)) 1050000) (* 20 t)))) (and (> (+ (* 150000 gt) produced) (+ 3000 (* t (* 5 (* (- i 300) (- i 100)))))) (<= produced (+ 3000 (* i (* (- i 400) (* t 5))))))) (and (> produced (+ 3000 (* i (* (- i 400) (* t 5))))) (> (+ stored (* t 7000000)) 0)))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_13() {
        let problem = "(or (and (> (+ (* 4 produced) (* 3 stored)) (+ 612000 (* (* (* 20 (+ (- 100) i)) i) t))) (and (> (+ (* 150000 gt) produced) (+ 3000 (* (* (* 5 (+ (- 300) i)) (+ (- 100) i)) t))) (<= produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))))) (and (> produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))) (> stored (* (* 2000 i) t))))".to_string();
        let assumptions = "(and (= T 1) (and (>= i 0) (and (>= s 0) (and (<= s t) (<= t (+ s T))))))".to_string();
        let expected_output = "(or (and (> (+ (* 4 produced) (* 3 stored)) (+ 612000 (* i (* 20 (* (- i 100) t))))) (and (> (+ (* 150000 gt) produced) (+ 3000 (* (- i 100) (* t (* 5 (- i 300)))))) (<= produced (+ 3000 (* i (* t (* 5 (- i 400)))))))) (and (> produced (+ 3000 (* i (* t (* 5 (- i 400)))))) (> stored (* i (* t 2000)))))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_14() {
        let problem = "(or (and (> (+ (* 4 produced) (* 3 stored)) 612000) (and (> (+ (* 150000 gt) produced) 3000) (<= produced (* (- 3000) (+ (- 1) (* 50 t)))))) (and (> produced (* (- 3000) (+ (- 1) (* 50 t)))) (> stored (* 200000 t))))".to_string();
        let assumptions = "(and (= T 1) (and (>= i 0) (and (>= s 0) (and (<= s t) (<= t (+ s T))))))".to_string();
        let expected_output = "(or (and (<= produced (* (- 3000) (- (* 50 t) 1))) (and (> (+ (* 4 produced) (* 3 stored)) 612000) (> (+ (* 150000 gt) produced) 3000))) (and (> produced (* (- 3000) (- (* 50 t) 1))) (> stored (* t 200000))))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_15() {
        let problem = "(or (and (> (+ (+ (* 4 produced) (* 3 stored)) (* (* 3 slope) t)) (+ 612000 (* (* (* 20 (+ (- 400) i)) i) t))) (and (<= produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))) (> (+ (* 150000 gt) produced) (+ 3000 (* (* (* 5 (+ (- 300) i)) (+ (- 100) i)) t))))) (and (> produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))) (> (+ stored (* slope t)) 0)))".to_string();
        let assumptions = "(and (<= t 1) (and (= 1 T) (and (>= i 0) (>= t 0))))".to_string();
        let expected_output = "(or (and (> (+ (* 150000 gt) produced) (+ 3000 (* t (* (* 5 (- i 300)) (- i 100))))) (and (> (+ (* 4 produced) (* 3 stored)) (+ 612000 (* t (* 20 (* i (- i 400)))))) (<= produced (+ 3000 (* t (* i (* (- i 400) 5))))))) (and (> produced (+ 3000 (* t (* i (* (- i 400) 5))))) (> (+ stored (* slope t)) 0)))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_16() {
        let problem = "(or (and (= y 0) (and (or (< (* T v) R) (< x R)) (or (>= x R) (> (+ R x) 0)))) (or (and (= (+ R y) 0) (or (and (< (* T v) (+ R x)) (and (< y (+ R (* T v))) (and (< y (+ (* 2 R) x)) (<= x y)))) (and (> (+ R (* T v)) y) (and (< x R) (< y x))))) (or (and (or (and (>= x R) (< (* T v) (+ R y))) (and (> R x) (> (+ R x) 0))) (or (and (> y 0) (< y R)) (and (> (+ R y) 0) (< y 0)))) (and (< (+ R y) 0) (and (> R x) (> (+ R x) 0))))))".to_string();
        let assumptions = "(and (or (and (<= x 0) (and (> (+ R x) 0) (< y R))) (or (and (> x 0) (and (< y R) (> R x))) (or (and (< y 0) (or (and (= R (* T v)) (and (> x 0) (or (and (= 0 (+ R y)) (< x R)) (> (+ R y) 0)))) (or (and (< (* T v) (+ R x)) (and (< x R) (or (and (= 0 (+ R y)) (not (= R (* T v)))) (< (+ R y) 0)))) (and (> (+ R x) (* T v)) (and (> (+ R y) 0) (or (and (< R (* T v)) (< (* T v) (* R 2))) (< (* T v) R))))))) (and (>= y 0) (and (> R y) (or (and (> x 0) (= R (* T v))) (and (> (+ R x) (* T v)) (or (and (< R (* T v)) (< (* T v) (* R 2))) (< (* T v) R))))))))) (and (> v 0) (and (> T 0) (> R 0))))".to_string();
        let expected_output = "(or (and (= y 0) (or (< (* T v) R) (< x R))) (or (and (or (and (>= x R) (< (* T v) (+ R y))) (> R x)) (or (> y 0) (and (> (+ R y) 0) (< y 0)))) (<= (+ R y) 0)))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_17() {
        let problem = "(or (and (<= (+ R y) 0) (or (and (= x 0) (< (* T v) R)) (and (< x (+ R (* T v))) (and (< (* T v) (+ R x)) (or (and (> x 0) (< x R)) (< x 0)))))) (and (> (+ R y) 0) (and (< y R) (or (and (= x 0) (< (* T v) R)) (or (and (< (* T v) (+ R x)) (or (and (> x 0) (< x R)) (< x 0))) (and (< (* T v) (* 2 R)) (>= x R)))))))".to_string();
        let assumptions = "(and (or (and (<= x 0) (and (> (+ R x) 0) (< y R))) (or (and (> x 0) (and (< y R) (> R x))) (or (and (< y 0) (or (and (= R (* T v)) (and (> x 0) (or (and (= 0 (+ R y)) (< x R)) (> (+ R y) 0)))) (or (and (< (* T v) (+ R x)) (and (< x R) (or (and (= 0 (+ R y)) (not (= R (* T v)))) (< (+ R y) 0)))) (and (> (+ R x) (* T v)) (and (> (+ R y) 0) (or (and (< R (* T v)) (< (* T v) (* R 2))) (< (* T v) R))))))) (and (>= y 0) (and (> R y) (or (and (> x 0) (= R (* T v))) (and (> (+ R x) (* T v)) (or (and (< R (* T v)) (< (* T v) (* R 2))) (< (* T v) R))))))))) (and (> v 0) (and (> T 0) (> R 0))))".to_string();
        let expected_output = "(or (and (= 0 x) (< (* T v) R)) (or (and (< (* T v) (+ R x)) (or (and (> x 0) (< x R)) (< x 0))) (>= x R)))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_18() {
        let problem = "(or (and (= y 0) (> R (* T v))) (or (> R x) (or (and (distinct y 0) (> (+ R y) (* T v))) (<= (+ R y) 0))))".to_string();
        let assumptions = "(and (or (and (<= x 0) (and (> (+ R x) 0) (< y R))) (or (and (> x 0) (and (< y R) (> R x))) (or (and (< y 0) (or (and (= R (* T v)) (and (> x 0) (or (and (= 0 (+ R y)) (< x R)) (> (+ R y) 0)))) (or (and (< (* T v) (+ R x)) (and (< x R) (or (and (= 0 (+ R y)) (not (= R (* T v)))) (< (+ R y) 0)))) (and (> (+ R x) (* T v)) (and (> (+ R y) 0) (or (and (< R (* T v)) (< (* T v) (* R 2))) (< (* T v) R))))))) (and (>= y 0) (and (> R y) (or (and (> x 0) (= R (* T v))) (and (> (+ R x) (* T v)) (or (and (< R (* T v)) (< (* T v) (* R 2))) (< (* T v) R))))))))) (and (> v 0) (and (> T 0) (> R 0))))".to_string();
        let expected_output = "(or (> R x) (or (and (= y 0) (> R (* T v))) (and (distinct y 0) (> (+ R y) (* T v)))))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_19() {
        let problem = "(or (and (= x 0) (< (* T v) R)) (or (and (< (* T v) (+ R x)) (distinct x 0)) (>= x R)))".to_string();
        let assumptions = "(and (or (and (<= x 0) (and (> (+ R x) 0) (< y R))) (or (and (> x 0) (and (< y R) (> R x))) (or (and (< y 0) (or (and (= R (* T v)) (and (> x 0) (or (and (= 0 (+ R y)) (< x R)) (> (+ R y) 0)))) (or (and (< (* T v) (+ R x)) (and (< x R) (or (and (= 0 (+ R y)) (not (= R (* T v)))) (< (+ R y) 0)))) (and (> (+ R x) (* T v)) (and (> (+ R y) 0) (or (and (< R (* T v)) (< (* T v) (* R 2))) (< (* T v) R))))))) (and (>= y 0) (and (> R y) (or (and (> x 0) (= R (* T v))) (and (> (+ R x) (* T v)) (or (and (< R (* T v)) (< (* T v) (* R 2))) (< (* T v) R))))))))) (and (> v 0) (and (> T 0) (> R 0))))".to_string();
        let expected_output = "(or (and (= x 0) (< (* T v) R)) (and (< (* T v) (+ R x)) (distinct x 0)))".to_string();
        
        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_20() {
        let problem = "(or (and (> (+ (* 3000 (+ (- 51) (* 50 gt))) produced) 0) (and (> (+ (+ (* 7000000 gt) (* 48 produced)) stored) 7344000) (or (and (> gt 1) (and (< (* 35 gt) 36) (<= produced 3000))) (and (>= (* 35 gt) 36) (< (+ (* 437500 gt) (* 3 produced)) 459000))))) (and (or (>= (* 35 gt) 36) (> produced 3000)) (and (or (< (* 35 gt) 36) (>= (+ (* 437500 gt) (* 3 produced)) 459000)) (> stored 0))))".to_string();
        let assumptions = "(and (or (and (<= produced 3000) (or (and (> (+ (* 3000 (- (* 50 gt) 51)) produced) 0) (and (> (+ (+ (* gt 7000000) (* produced 48)) stored) 7344000) (< (+ (* gt 437500) (* produced 3)) 459000))) (and (> (+ (* gt 150000) produced) 3000) (> (+ (* produced 4) (* stored 3)) 612000)))) (and (> stored 0) (or (>= (+ (* gt 437500) (* produced 3)) 459000) (> produced 3000)))) (and (= T 1) (>= i 0)))".to_string();
        let expected_output = "(or (and (> (+ (* 3000 (- (* 50 gt) 51)) produced) 0) (or (and (> gt 1) (and (< (* gt 35) 36) (<= produced 3000))) (and (>= (* gt 35) 36) (< (+ (* gt 437500) (* produced 3)) 459000)))) (and (or (>= (* gt 35) 36) (> produced 3000)) (or (< (* gt 35) 36) (>= (+ (* gt 437500) (* produced 3)) 459000))))".to_string();
        
        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_21() {
        let problem = "(or (and (or (> gt 1) (<= produced 3000)) (and (or (<= gt 1) (<= (+ (* 3000 (+ (- 51) (* 50 gt))) produced) 0)) (and (> (+ (* 150000 gt) produced) 3000) (> (+ (* 4 produced) (* 3 stored)) 612000)))) (or (and (> gt 1) (or (and (> (+ (* 3000 (+ (- 51) (* 50 gt))) produced) 0) (and (> (+ (+ (* 7000000 gt) (* 48 produced)) stored) 7344000) (< (+ (* 437500 gt) (* 3 produced)) 446500))) (and (> stored 200000) (>= (+ (* 437500 gt) (* 3 produced)) 446500)))) (and (<= gt 1) (and (> produced 3000) (> stored 200000)))))".to_string();
        let assumptions = "(and (or (and (<= produced 3000) (or (and (> (+ (* 3000 (- (* 50 gt) 51)) produced) 0) (and (> (+ (+ (* gt 7000000) (* produced 48)) stored) 7344000) (< (+ (* gt 437500) (* produced 3)) 459000))) (and (> (+ (* gt 150000) produced) 3000) (> (+ (* produced 4) (* stored 3)) 612000)))) (and (> stored 0) (or (>= (+ (* gt 437500) (* produced 3)) 459000) (> produced 3000)))) (and (= T 1) (>= i 0)))".to_string();
        let expected_output = "(or (and (<= produced 3000) (<= (+ (* 3000 (- (* 50 gt) 51)) produced) 0)) (or (and (> gt 1) (or (and (> (+ (* 3000 (- (* 50 gt) 51)) produced) 0) (< (+ (* gt 437500) (* produced 3)) 446500)) (and (> stored 200000) (>= (+ (* gt 437500) (* produced 3)) 446500)))) (and (<= gt 1) (and (> stored 200000) (> produced 3000)))))".to_string();
        
        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_22() {
        let problem = "(or (and (or (> gt 1) (<= produced 3000)) (and (or (<= gt 1) (<= (+ (* 3000 (+ (- 51) (* 50 gt))) produced) 0)) (and (> (+ (* 150000 gt) produced) 3000) (> (+ (* 4 produced) (* 3 stored)) 612000)))) (or (and (> gt 1) (or (and (> (+ (* 3000 (+ (- 51) (* 50 gt))) produced) 0) (and (> (+ (+ (* 7000000 gt) (* 48 produced)) stored) 7344000) (< (+ (* 437500 gt) (* 3 produced)) 446500))) (and (> stored 200000) (>= (+ (* 437500 gt) (* 3 produced)) 446500)))) (and (<= gt 1) (and (> produced 3000) (> stored 200000)))))".to_string();
        let assumptions = "(and (or (and (<= produced 3000) (or (and (> (+ (* 3000 (- (* 50 gt) 51)) produced) 0) (and (> (+ (+ (* gt 7000000) (* produced 48)) stored) 7344000) (< (+ (* gt 437500) (* produced 3)) 459000))) (and (> (+ (* gt 150000) produced) 3000) (> (+ (* produced 4) (* stored 3)) 612000)))) (and (> stored 0) (or (>= (+ (* gt 437500) (* produced 3)) 459000) (> produced 3000)))) (and (= T 1) (>= i 0)))".to_string();
        let expected_output = "(or (and (<= produced 3000) (<= (+ (* 3000 (- (* 50 gt) 51)) produced) 0)) (or (and (> gt 1) (or (and (> (+ (* 3000 (- (* 50 gt) 51)) produced) 0) (< (+ (* gt 437500) (* produced 3)) 446500)) (and (> stored 200000) (>= (+ (* gt 437500) (* produced 3)) 446500)))) (and (<= gt 1) (and (> stored 200000) (> produced 3000)))))".to_string();
        
        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_23() {
        let problem = "(or (and (> gt 1) (> (+ (* 3000 (+ (- 51) (* 50 gt))) produced) 0)) (or (and (>= (* 35 gt) 36) (>= (+ (* 437500 gt) (* 3 produced)) 459000)) (and (< (* 35 gt) 36) (> produced 3000))))".to_string();
        let assumptions = "(and (or (and (<= produced 3000) (or (and (> (+ (* 3000 (- (* 50 gt) 51)) produced) 0) (and (> (+ (+ (* gt 7000000) (* produced 48)) stored) 7344000) (< (+ (* gt 437500) (* produced 3)) 459000))) (and (> (+ (* gt 150000) produced) 3000) (> (+ (* produced 4) (* stored 3)) 612000)))) (and (> stored 0) (or (>= (+ (* gt 437500) (* produced 3)) 459000) (> produced 3000)))) (and (= T 1) (>= i 0)))".to_string();
        let expected_output = "(or (and (> gt 1) (> (+ (* 3000 (- (* gt 50) 51)) produced) 0)) (and (< (* gt 35) 36) (> produced 3000)))".to_string();
        
        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_24() {
        let problem = "(or (and (> (+ (+ (* 4 produced) (* 3 stored)) (* (* 3 slope) t)) (+ 612000 (* (* (* 20 (+ (- 400) i)) i) t))) (and (<= produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))) (> (+ (* 150000 gt) produced) (+ 3000 (* (* (* 5 (+ (- 300) i)) (+ (- 100) i)) t))))) (and (> produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))) (> (+ stored (* slope t)) 0)))".to_string();
        let assumptions = "(and (= T 1) (and (>= i 0) (>= t 0)))".to_string();
        let expected_output = "(or (and (> (+ (* 150000 gt) produced) (+ 3000 (* t (* (* 5 (- i 300)) (- i 100))))) (and (> (+ (+ (* 4 produced) (* 3 stored)) (* 3 (* slope t))) (+ 612000 (* t (* 20 (* i (- i 400)))))) (<= produced (+ 3000 (* t (* i (* (- i 400) 5))))))) (and (> produced (+ 3000 (* t (* i (* (- i 400) 5))))) (> (+ stored (* slope t)) 0)))".to_string();
        
        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_25() {
        let problem = "(or (and (> (+ (* 4 produced) (* 3 stored)) (+ 612000 (* (* 20 (+ (- 1050000) (* (+ (- 400) i) i))) t))) (and (> (+ (* 150000 gt) produced) (+ 3000 (* (* (* 5 (+ (- 300) i)) (+ (- 100) i)) t))) (<= produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))))) (and (> produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))) (> (+ stored (* 7000000 t)) 0)))".to_string();
        let assumptions = "(and (= T 1) (and (>= i 0) (and (>= s 0) (and (<= s t) (<= t (+ s T))))))".to_string();
        let expected_output = "(or (and (> (+ (* 4 produced) (* 3 stored)) (+ 612000 (* (- (* i (- i 400)) 1050000) (* 20 t)))) (and (> (+ (* 150000 gt) produced) (+ 3000 (* t (* 5 (* (- i 300) (- i 100)))))) (<= produced (+ 3000 (* i (* (- i 400) (* t 5))))))) (and (> produced (+ 3000 (* i (* (- i 400) (* t 5))))) (> (+ stored (* t 7000000)) 0)))".to_string();

        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_26() {
        let problem = "(or (and (> (+ (* 4 produced) (* 3 stored)) (+ 612000 (* (* 20 (+ (- 1050000) (* (+ (- 400) i) i))) t))) (and (> (+ (* 150000 gt) produced) (+ 3000 (* (* (* 5 (+ (- 300) i)) (+ (- 100) i)) t))) (<= produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))))) (and (> produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))) (> (+ stored (* 7000000 t)) 0)))".to_string();
        let assumptions = "(and (= T 1) (and (>= i 0) (and (>= s 0) (and (<= s t) (<= t (+ s T))))))".to_string();
        let expected_output = "(or (and (> (+ (* 4 produced) (* 3 stored)) (+ 612000 (* (- (* i (- i 400)) 1050000) (* 20 t)))) (and (> (+ (* 150000 gt) produced) (+ 3000 (* t (* 5 (* (- i 300) (- i 100)))))) (<= produced (+ 3000 (* i (* (- i 400) (* t 5))))))) (and (> produced (+ 3000 (* i (* (- i 400) (* t 5))))) (> (+ stored (* t 7000000)) 0)))".to_string();
        
        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_27() {
        let problem = "(or (and (> (+ (* 4 produced) (* 3 stored)) (+ 612000 (* (* (* 20 (+ (- 100) i)) i) t))) (and (> (+ (* 150000 gt) produced) (+ 3000 (* (* (* 5 (+ (- 300) i)) (+ (- 100) i)) t))) (<= produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))))) (and (> produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))) (> stored (* (* 2000 i) t))))".to_string();
        let assumptions = "(and (= T 1) (and (>= i 0) (and (>= s 0) (and (<= s t) (<= t (+ s T))))))".to_string();
        let expected_output = "(or (and (> (+ (* 4 produced) (* 3 stored)) (+ 612000 (* i (* 20 (* (- i 100) t))))) (and (> (+ (* 150000 gt) produced) (+ 3000 (* (- i 100) (* t (* 5 (- i 300)))))) (<= produced (+ 3000 (* i (* t (* 5 (- i 400)))))))) (and (> produced (+ 3000 (* i (* t (* 5 (- i 400)))))) (> stored (* i (* t 2000)))))".to_string();
        
        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_28() {
        let problem = "(or (and (> (+ (* 4 produced) (* 3 stored)) 612000) (and (> (+ (* 150000 gt) produced) 3000) (<= produced (* (- 3000) (+ (- 1) (* 50 t)))))) (and (> produced (* (- 3000) (+ (- 1) (* 50 t)))) (> stored (* 200000 t))))".to_string();
        let assumptions = "(and (= T 1) (and (>= i 0) (and (>= s 0) (and (<= s t) (<= t (+ s T))))))".to_string();
        let expected_output = "(or (and (<= produced (* (- 3000) (- (* 50 t) 1))) (and (> (+ (* 4 produced) (* 3 stored)) 612000) (> (+ (* 150000 gt) produced) 3000))) (and (> produced (* (- 3000) (- (* 50 t) 1))) (> stored (* t 200000))))".to_string();
        
        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_29() {
        let problem = "(or (and (> (+ (+ (* 4 produced) (* 3 stored)) (* (* 3 slope) t)) (+ 612000 (* (* (* 20 (+ (- 400) i)) i) t))) (and (<= produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))) (> (+ (* 150000 gt) produced) (+ 3000 (* (* (* 5 (+ (- 300) i)) (+ (- 100) i)) t))))) (and (> produced (+ 3000 (* (* (* 5 (+ (- 400) i)) i) t))) (> (+ stored (* slope t)) 0)))".to_string();
        let assumptions = "(and (<= t 1) (and (= 1 T) (and (>= i 0) (>= t 0))))".to_string();
        let expected_output = "(or (and (> (+ (* 150000 gt) produced) (+ 3000 (* t (* (* 5 (- i 300)) (- i 100))))) (and (> (+ (+ (* 4 produced) (* 3 stored)) (* 3 (* slope t))) (+ 612000 (* t (* 20 (* i (- i 400)))))) (<= produced (+ 3000 (* t (* i (* (- i 400) 5))))))) (and (> produced (+ 3000 (* t (* i (* (- i 400) 5))))) (> (+ stored (* slope t)) 0)))".to_string();
        
        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aggressive_simplify_case_30() {
        let problem = "(and (> stored 200000) (or (> produced 3000) (and (> (+ (* 150000 gt) produced) 3000) (> (+ (* 4 produced) (* 3 stored)) 612000))))".to_string();
        let assumptions = "(and (or (and (> (+ (* produced 4) (* 3 stored)) 612000) (and (<= produced 3000) (> (+ (* 150000 gt) produced) 3000))) (and (> produced 3000) (> stored 0))) (and (= T 1) (>= i 0)))".to_string();
        let expected_output = "(> stored 200000)".to_string();
        
        let output = simplify::aggressive_simplify(problem, assumptions.clone());
        assert_eq!(output, expected_output);
    }

}