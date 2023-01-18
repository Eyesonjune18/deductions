#![allow(dead_code, unused_variables)]

mod deductions;
mod history;
mod propositions;

use deductions::Deduction;
use deductions::ValueMap;
use propositions::Proposition;

fn main() {
    let propositions = ["(m & b) > j", "(f | s) > m", "b > t", "f > !t", "f"].to_vec();
    let mut deduction = Deduction::from_strs(propositions);

    deduction.update_actual_values();
    deduction.substitute_all();

    println!("{}", &deduction);
}
