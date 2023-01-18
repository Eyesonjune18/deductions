#![allow(dead_code, unused_variables)]

mod propositions;
mod deductions;
mod history;

use propositions::Proposition;
use deductions::Deduction;
use deductions::ValueMap;
use history::EvaluationHistory;

fn main() {
    let propositions = ["(m & b) > j", "(f | s) > m", "b > t", "f > !t", "f"].to_vec();
    let deduction = Deduction::from_strs(propositions);

    let history = EvaluationHistory::new();
}
