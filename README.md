This repository uses egg (e-graphs good) to perform algebraic simplification of formulas under assumptions.
For example, under the assumptions `B>0 && v>=0`, it simplifies the formula `(B>(2*e+(-2)*p)^(-1)*v^2 || v<=0) && e>p` to `B*(2*(e-p))>v^2`.
The tool uses uses the z3 smtlib syntax for formulas.

## Dependencies
This project is written in rust, and uses cargo. In linux you can install this with
`curl--proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
or
`sudo apt-get install cargo`.

If this does not work or you see an error along the lines of `cargo : The term cargo' is not recognized`, see [instructions for installing cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

This repository uses z3-sys, which requires clang. To install clang on linux, use
`sudo apt-get install clang`.

## Usage
To clone the project, run:
```
git clone https://github.com/aditink/algebraic_simplification_cesar.git
```

### Usage: Generating a Binary
To generate a binary, in the folder algebraic_simplification_cesar, run
`cargo build --release`.
This creates the executable `simplify` in the folder `target`.

#### Quickstart
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
The last line, `rm -rf algebraic_simplification_cesar`, deletes the source code, so do not run it if you want to modify the code.

### Usage: Binary
Use the produced binary to simplify formula f against assumptions a with the shell command:
`./simplify -a <f> <a>`
For example,
`./simplify -a "(and (or (> B (* (^ (+ (* 2 e) (* (- 2) p)) (- 1)) (^ v 2))) (<= v 0)) (> e p))" "(and (> B 0) (>= v 0))"`
outputs on the commandline the simplified formula
`(> (* B (* 2 (- e p))) (^ v 2))`

This command should be used in whichever folder you have copied the binary `simplify` to.
If you used the commands in the setup section above, this is the directory into which you cloned the repository.

### Usage: Developing and Recompiling
To recompile and then simplify formula f against assumptions a, use
`cargo run -- -s <f> <a>`.

For example,
`cargo run -- -s "(or (and (= (v) (0)) (distinct (intersection) (x))) (and (> (v) (0)) (or (and (= (v) (vmax)) (or (< (+ (* (* (2) (A)) (intersection)) (^ (v) (2))) (+ (^ (vmax) (2)) (* (* (2) (A)) (x)))) (or (and (> (timeToRed) (0)) (= (+ (* (* (2) (A)) (intersection)) (^ (v) (2))) (+ (^ (vmax) (2)) (* (* (2) (A)) (x))))) (and (> (+ (* (* (2) (A)) (intersection)) (^ (v) (2))) (+ (^ (vmax) (2)) (* (* (2) (A)) (x)))) (or (> (B) (* (^ (v) (2)) (^ (+ (* (2) (intersection)) (* (-2) (x))) (-1)))) (< (* (2) (intersection)) (+ (* (timeToRed) (+ (v) (vmax))) (* (2) (x))))))))) (and (< (v) (vmax)) (or (< (intersection) (x)) (or (and (> (intersection) (x)) (or (> (* (* (2) (B)) (+ (intersection) (* (-1) (x)))) (^ (v) (2))) (< (* (intersection) (v)) (* (v) (+ (* (timeToRed) (v)) (x)))))) (and (> (timeToRed) (0)) (= (intersection) (x)))))))))" "(and (>= v 0) (and (> A 0) (and (> B 0) (and (> T 0) (> vmax 0)))))"`

<<<<<<< HEAD
## Generating a Binary
To generate a binary, in the folder algebraic_simplification_cesar, run
`cargo build --release`.
This creates the executable `simplify` in the folder `target`.

## Calling Rust from Python
Before you begin, ensure you have the following installed:
- Python
- Rust and Cargo (Rust's package manager)
- Maturin (for building and publishing Rust crates as Python packages)

### Step 1: Setting Up Environment

Create a virtual environment:
```
python3 -m venv .env  
source .env/bin/activate
```
### Step 2: Installing Maturin
```
pip install maturin
```

Inside the directory, now run `maturin init`. This will generate the new package source. When given the choice of bindings to use, select pyo3 bindings:
```
$ maturin init
✔ 🤷 Which kind of bindings to use?
  📖 Documentation: https://maturin.rs/bindings.html · pyo3
  ✨ Done! Initialized project /your/path
```

### Step 3: Building and Installing the Python Module

This will build the package and install it into the Python virtualenv previously created and activated. The package is then ready to be used from `python`:
```
maturin develop
```

An example output would be:
```
📦 Including license file "/path/to/project/LICENSE"
🔗 Found pyo3 bindings
🐍 Found CPython 3.12 at /path/to/project/.env/bin/python
📡 Using build options features from pyproject.toml
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
📦 Built wheel for CPython 3.12 to /var/folders/.../algebraic_simplification_cesar-0.1.0-cp312-cp312-macosx_10_12_x86_64.whl
✏️  Setting installed package as editable
🛠 Installed algebraic_simplification_cesar-0.1.0
```

### Step 4: Using the Python Module

Open a Python Shell:
```
python
```
Import and Use the Module:
```
import simplify
simplify.aggressive_simplify("formula","assumption")
```

### References

For more detailed information on using PyO3, refer to the [official PyO3 guide](https://github.com/PyO3/pyo3).

=======
>>>>>>> 64dad5acc1f7b6a55116882ac25362e6a60898fb
## More Information
This tool was designed to collapse redundant cells resulting from performing [quanitifier elimination](https://reference.wolfram.com/language/ref/Resolve.html), but can work for other simplification applications as well. It is used by the [CESAR](https://arxiv.org/abs/2311.02833) tool.
