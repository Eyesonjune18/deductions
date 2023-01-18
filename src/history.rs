use crate::Deduction;

// Stores the history of the evaluation of the propositions in order to show work later
pub struct EvaluationHistory {
    old_deduction_stacks: Vec<Deduction>,
}

impl EvaluationHistory {
    // Creates an empty EvaluationHistory
    pub fn new() -> Self {
        Self {
            old_deduction_stacks: Vec::new(),
        }
    }

    // Adds a Deduction to the history
    fn push(&mut self, deduction: Deduction) {
        self.old_deduction_stacks.push(deduction);
    }
}
