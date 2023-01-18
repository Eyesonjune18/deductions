use std::collections::HashMap;

// Stores all the given or working propositions on a stack
pub struct Deduction {
    proposition_stack: Vec<String>,
    proposition_values: ValueMap,
}

// Stores the values of the propositions used in the given deduction
struct ValueMap {
    values: HashMap<char, Option<bool>>
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
        let proposition_stack: Vec<String> = propositions.iter().map(|x| x.to_string()).collect();
        let proposition_values = ValueMap::from_proposition_stack(&proposition_stack);

        Self {
            proposition_stack,
            proposition_values,
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

    // Finds all the propositions in the given stack and initializes them to None
    // This is used to create a Deduction from a vector of propositions
    fn from_proposition_stack(proposition_stack: &Vec<String>) -> Self {
        let mut values = HashMap::new();

        for proposition in proposition_stack {
            for character in proposition.chars() {
                if character.is_alphabetic() {
                    values.insert(character, None);
                }
            }
        }

        Self {
            values,
        }
    }
}