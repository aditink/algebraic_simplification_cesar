This repository uses egg (e-graphs good) to perform arithmetic simplification. It is used by the CESAR tool.

t## Dependencies
Written rust, uses cargo.

## Simplification
To simplify formula f against assumptions a, use
`cargo run -- -s <f> <a>`.

For example,
`cargo run -- -s "(or (and (= (v) (0)) (distinct (intersection) (x))) (and (> (v) (0)) (or (and (= (v) (vmax)) (or (< (+ (* (* (2) (A)) (intersection)) (^ (v) (2))) (+ (^ (vmax) (2)) (* (* (2) (A)) (x)))) (or (and (> (timeToRed) (0)) (= (+ (* (* (2) (A)) (intersection)) (^ (v) (2))) (+ (^ (vmax) (2)) (* (* (2) (A)) (x))))) (and (> (+ (* (* (2) (A)) (intersection)) (^ (v) (2))) (+ (^ (vmax) (2)) (* (* (2) (A)) (x)))) (or (> (B) (* (^ (v) (2)) (^ (+ (* (2) (intersection)) (* (-2) (x))) (-1)))) (< (* (2) (intersection)) (+ (* (timeToRed) (+ (v) (vmax))) (* (2) (x))))))))) (and (< (v) (vmax)) (or (< (intersection) (x)) (or (and (> (intersection) (x)) (or (> (* (* (2) (B)) (+ (intersection) (* (-1) (x)))) (^ (v) (2))) (< (* (intersection) (v)) (* (v) (+ (* (timeToRed) (v)) (x)))))) (and (> (timeToRed) (0)) (= (intersection) (x)))))))))" "(and (>= v 0) (and (> A 0) (and (> B 0) (and (> T 0) (> vmax 0)))))"`

## Build
To generate a binary, run
`cargo build --release`.
This creates `simplify` in the folder `target`.
