import subprocess
import time
import os, sys
import argparse

from benchmark import Benchmark

def main():
    parser = argparse.ArgumentParser(description='Benchmark Algebraic Simplification Cesar')
    parser.add_argument('--num-runs', type=int, default=100, help='Number of runs.')
    parser.add_argument('--warm-up-runs', type=int, default=10, help='Number of warm-up runs.')
    parser.add_argument('--problem', type=str, default=None, help='Problem to benchmark.')
    parser.add_argument('--assumptions', type=str, default=None, help='Assumptions associated with the problem.')
    args = parser.parse_args()

    pairs = [
        ("(and (or (> B (* (^ (+ (* 2 e) (* (- 2) p)) (- 1)) (^ v 2))) (<= v 0)) (> e p))",
         "(and (> A 0) (and (> B 0) (and (> T 0) (>= v 0))))"),
        ("(< (+ (* A T) v) (^ (* (* B (^ (+ A B) (- 1))) (+ (+ (* (* 2 A) e) (* (* (- 2) A) p)) (^ v 2))) (/ 1 2)))", "(and (> (* B (* 2 (- e p))) (^ v 2)) (and (> A 0) (and (> B 0) (and (> T 0) (>= v 0)))))"),
        ("(> (+ (+ e (* (* (/ 1 2) B) (^ t 2))) (* (* (- 1) t) v)) p)", "(and (or (<= B (* (^ t (- 1)) v)) (<= t 0)) (and (> A 0) (and (> B 0) (and (> T 0) (and (>= v 0) (>= t 0))))))"),
        ("(> (* B (+ (+ (* 2 e) (* (- 2) p)) (* (* (- 1) t) (+ (* A t) (* 2 v))))) (^ (+ (* A t) v) 2))", "(and (>= t 0) (and (>= v 0) (and (> A 0) (and (> B 0) (and (<= t T) (> T 0))))))"),
        ("(or (and (= e 0) (or (and (= v 0) (< (+ (* (* 2 B) p) (* (* A (+ A B)) (^ T 2))) 0)) (and (> v 0) (< (+ (+ (* (* 2 B) p) (^ (+ (* A T) v) 2)) (* (* B T) (+ (* A T) (* 2 v)))) 0)))) (and (distinct e 0) (or (and (= v 0) (< (+ p (* (* (* (* (/ 1 2) A) (^ B (- 1))) (+ A B)) (^ T 2))) e)) (and (> v 0) (< (+ (^ (+ (* A T) v) 2) (* B (+ (+ (+ (* (- 2) e) (* 2 p)) (* A (^ T 2))) (* (* 2 T) v)))) 0))))", "(and (> (* 2 (* B (- e p))) (^ v 2)) (and (> A 0) (and (> B 0) (and (> T 0) (>= v 0)))))"),
        ("(or (and (< (+ (+ Tx (* (* 2 T) V)) y) (+ Ty x)) (or (and (= Tx x) (> Ty y)) (or (and (> (+ Tx Ty) (+ x y)) (< Tx x)) (and (> Tx x) (and (> x 0) (< (+ Tx y) (+ Ty x))))))) (or (and (= x 0) (< (+ Tx y) Ty)) (or (and (< Tx x) (and (< (+ Tx (* T V)) x) (<= (+ Tx Ty) (+ x y)))) (or (and (< (+ (+ Tx x) y) Ty) (or (and (> (+ Tx x) 0) (< x 0)) (or (<= Tx x) (> x 0)))) (or (< (+ Tx x) 0) (and (> Ty y) (<= (+ Tx x) 0)))))))", "(and (or (and (< y Ty) (or (> Ty (+ (+ Tx x) y)) (> (+ Ty x) (+ Tx y)))) (or (< Tx x) (< (+ Tx x) 0))) (and (> Tx 0) (and (> Ty 0) (and (> V 0) (> T 0)))))"),
        ("(or (and (< (+ (+ (+ Tx (* (* 2 T) V)) x) y) Ty) (or (and (= (+ Tx x) 0) (> Ty y)) (or (and (> (+ Tx x) 0) (and (< x 0) (< (+ (+ Tx x) y) Ty))) (and (> (+ (+ Tx Ty) x) y) (< (+ Tx x) 0))))) (or (< Tx x) (or (and (< (+ Tx y) (+ Ty x)) (or (< x 0) (or (> Tx x) (<= (+ Tx x) 0)))) (or (and (< (+ Tx x) 0) (and (< (+ (+ Tx (* T V)) x) 0) (<= (+ (+ Tx Ty) x) y))) (and (<= Tx x) (> Ty y))))))", "(and (or (and (< y Ty) (or (> Ty (+ (+ Tx x) y)) (> (+ Ty x) (+ Tx y)))) (or (< Tx x) (< (+ Tx x) 0))) (and (> Tx 0) (and (> Ty 0) (and (> V 0) (> T 0)))))")
    ]

    benchmark = Benchmark(args, pairs)
    benchmark.run()
    benchmark.print_results()

if __name__ == "__main__":
    main()
