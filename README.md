This repository uses egg (e-graphs good) to perform algebraic simplification of formulas under assumptions.
For example, under the assumptions `B>0 and A>0`, it simplifies the formula `(B>(2*e+(-2)*p)^(-1)*v^2 or v<=0) and e>p` to `B*(2*(e-p))>v^2`.
The tool uses uses the z3 smtlib syntax for formulas.

## Dependencies
Written in rust, uses cargo. In linux you can install this with
```sudo apt-get install cargo```

## Setup
To clone the project:
```git clone https://github.com/aditink/algebraic_simplification_cesar.git```

To obtain the binary, run the following commands (meant for linux shell):
```
git clone https://github.com/aditink/algebraic_simplification_cesar.git
cd algebraic_simplification_cesar
cargo build --release
cd ..
cp algebraic_simplification_cesar/target/release/simplify .
rm -rf algebraic_simplification_cesar
```
This will produce the executable simplify.

## Usage: Binary
Use the produced binary to simplify formula f against assumptions a with the shell command:
`./simplify -a <f> <a>`
For example,
`./simplify -a "(and (or (> B (* (^ (+ (* 2 e) (* (- 2) p)) (- 1)) (^ v 2))) (<= v 0)) (> e p))" "(and (> B 0) (>= v 0))"`
outputs on the commandline the simplified formula
`(> (* B (* 2 (- e p))) (^ v 2))`

## Usage: Recompiling
To recompile and then simplify formula f against assumptions a, use
`cargo run -- -s <f> <a>`.

For example,
`cargo run -- -s "(or (and (= (v) (0)) (distinct (intersection) (x))) (and (> (v) (0)) (or (and (= (v) (vmax)) (or (< (+ (* (* (2) (A)) (intersection)) (^ (v) (2))) (+ (^ (vmax) (2)) (* (* (2) (A)) (x)))) (or (and (> (timeToRed) (0)) (= (+ (* (* (2) (A)) (intersection)) (^ (v) (2))) (+ (^ (vmax) (2)) (* (* (2) (A)) (x))))) (and (> (+ (* (* (2) (A)) (intersection)) (^ (v) (2))) (+ (^ (vmax) (2)) (* (* (2) (A)) (x)))) (or (> (B) (* (^ (v) (2)) (^ (+ (* (2) (intersection)) (* (-2) (x))) (-1)))) (< (* (2) (intersection)) (+ (* (timeToRed) (+ (v) (vmax))) (* (2) (x))))))))) (and (< (v) (vmax)) (or (< (intersection) (x)) (or (and (> (intersection) (x)) (or (> (* (* (2) (B)) (+ (intersection) (* (-1) (x)))) (^ (v) (2))) (< (* (intersection) (v)) (* (v) (+ (* (timeToRed) (v)) (x)))))) (and (> (timeToRed) (0)) (= (intersection) (x)))))))))" "(and (>= v 0) (and (> A 0) (and (> B 0) (and (> T 0) (> vmax 0)))))"`

## Generating a Binary
To generate a binary, in the folder algebraic_simplification_cesar, run
`cargo build --release`.
This creates the ececutable `simplify` in the folder `target`.

## More Information
This tool was designed, to collapse redundant cells resulting from performing [quanitifier elimination](https://reference.wolfram.com/language/ref/Resolve.html), but can work for other simplification applications as well. It is used by the [CESAR](https://arxiv.org/abs/2311.02833) tool.
