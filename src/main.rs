#![allow(dead_code, unused_variables)]

mod deductions;
mod expressions;
mod history;

use deductions::Deduction;
use deductions::ValueMap;
use expressions::Expression;
use expressions::ExpressionNode;

fn main() {
    let propositions = ["(m & b) > j", "(f | s) > m", "b > t", "f > !t", "f"].to_vec();
    let deduction = Deduction::from_strs(propositions);

    println!("{}", &deduction);
}

/*
(m ∧ ¬b) → j
(f ∨ s) → m
b → t
f → ¬t
f
∴ j

=>
(m ∧ ¬b) → j
(f ∨ s) → m
b → t
f → ¬t
f = TRUE

=>
(m ∧ ¬b) → j
(TRUE ∨ s) → m [SUBSTITUTE]
b → t
TRUE → ¬t [SUBSTITUTE]

=>
(m ∧ ¬b) → j
TRUE → m [EVALUATE]
b → t
t = FALSE [EVALUATE]

=>
(m ∧ ¬b) → j
m = TRUE [EVALUATE]
b → FALSE [SUBSTITUTE]

=>
(TRUE ∧ ¬b) → j [SUBSTITUTE]
b = FALSE [EVALUATE]

=>
(TRUE ∧ ¬FALSE) → j [SUBSTITUTE]

=>
(TRUE ∧ TRUE) → j [EVALUATE]

=>
TRUE → j [EVALUATE]

=>
j = TRUE [EVALUATE]
*/
