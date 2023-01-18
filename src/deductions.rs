use crate::Proposition;

use std::collections::HashMap;

// Stores all the given or working propositions on a stack
pub struct Deduction {
    proposition_stack: Vec<Proposition>,
    proposition_values: ValueMap,
}

// Stores the values of the root propositions used in the given deduction
pub struct ValueMap {
    values: HashMap<char, Option<bool>>,
}

impl std::fmt::Display for Deduction {
    // Displays all the propositions in the Deduction
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for proposition in &self.proposition_stack {
            writeln!(f, "{}", proposition)?;
        }

        Ok(())
    }
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
        let proposition_stack: Vec<Proposition> = propositions
            .iter()
            .map(|x| Proposition::from_str(x))
            .collect();
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

    // Finds all the root propositions in the given stack and initializes them to None
    // This is used to create a Deduction from a vector of propositions
    fn from_proposition_stack(proposition_stack: &Vec<Proposition>) -> Self {
        let mut values = HashMap::new();

        for proposition in proposition_stack {
            for character in proposition.get_string().chars() {
                if character.is_alphabetic() {
                    values.insert(character, None);
                }
            }
        }

        Self { values }
    }

    // Returns the value map
    pub fn get_values(&self) -> &HashMap<char, Option<bool>> {
        &self.values
    }
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
(TRUE ∨ s) → m
b → t
TRUE → ¬t

=>
(m ∧ ¬b) → j
TRUE → m
b → t
t = FALSE


=>
(m ∧ ¬b) → j
m = TRUE
b → FALSE

=>
(TRUE ∧ ¬b) → j
b = FALSE

=>
(TRUE ∧ ¬FALSE) → j

=>
(TRUE ∧ TRUE) → j

=>
TRUE → j

=>
j = TRUE
*/
