# Tau
A statically checked unit-aware programming language that prevents dimensional mistakes at compile time.
Tau lets you write variables in SI units so that your bad maths gets caught early on.

# Why Tau?
Do you remember the time your engineering coursework took 2 hours too long because you wrote the wrong unit on a piece of MATLAB code? Well I do. This problem, among others, can be entirely avoided by using a progamming language that enforces explicitly typed units.

Most languages (MATLAB included) will let you do this:
```MATLAB
force = 10  % N? lbs?
mass = 4  % kg? lbs?
acceleration = mass - force % ???
```

In Tau:
```Tau
let force: N = 10;
let mass: kg = 4;
let acceleration = force - mass  # Error: Cannot subtract N and kg
```

# Features
- Compile-time unit checking
- Automatic unit inference
- Mutable (`let`) and immutable (`const`) bindings

## Automatic unit inference
You don't always need to explicitly write the unit:
```Tau
let force: N = 10;
let mass: kg = 4;
let acceleration = force / mass;  # Result: m/s^2
```

Inferred dimensions are also checked against any explicit unit:
```Tau
let force: N = 10;
let mass: kg = 4;
let acceleration: m = force / mass;  # Error: Expected m got m/s^2
```

## Mutable and immutable variables
Some languages will let you change gravity:
```MATLAB
g = 9.81
% ...
g = 2  % They wanted 't = 2'
% ...
mass = 4
force = mass*g  % 8 - silent and hard-to-debug error
```

In Tau this becomes:
```Tau
const g: m/s^2 = 9.81;
# ...
g = 2;  # Error: Assignment to constant 'g'
```

# Installation

1. Clone the repository;
   `$ git clone git@github.com:JimmyBowcott/tau.git`
2. Build the project
   `$ cargo build --release`
3. Run some code:
   `$ cargo run test_file.tau`

# TODO
- [x] Add support for 'const' declaration
- [ ] Control flow
- [ ] Support for matrices
- [ ] String support
- [ ] Imperial unit support
