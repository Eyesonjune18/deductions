use std::collections::HashMap;

// Stores all the given or working propositions on a stack
pub struct Deduction {
    proposition_stack: Vec<String>,
    proposition_values: ValueMap,
}

// Stores the values of the propositions used in the given deduction
struct ValueMap {
    values: HashMap<char, bool>
}

impl Deduction {
    // Creates an empty Deduction
    fn new() -> Self {
        Self {
            proposition_stack: Vec::new(),
            proposition_values: ValueMap::new(),
        }
    }

    // Creates a Deduction from a vector of propositions
    pub fn from_strs(propositions: Vec<&str>) -> Self {
        Self {
            proposition_stack: propositions.iter().map(|x| x.to_string()).collect(),
            proposition_values: ValueMap::new(),
        }
    }

    // Checks if the Deduction is empty
    fn is_empty(&self) -> bool {
        self.proposition_stack.is_empty()
    }
}

impl ValueMap {
    // Creates an empty ValueMap
    fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}