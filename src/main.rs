#![allow(dead_code, unused_variables)]

mod deductions;
mod history;

use deductions::Deduction;
use history::EvaluationHistory;

fn main() {
    let propositions = ["(m & b) > j", "(f | s) > m", "b > t", "f > !t", "f"].to_vec();
    let deduction = Deduction::from_strs(propositions);

    let history = EvaluationHistory::new();
}