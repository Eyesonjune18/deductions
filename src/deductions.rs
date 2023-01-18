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

    // Substitutes all root propositions with their actual truth values, if known
    // TODO: This should not be public, but it is for testing
    pub fn substitute_all(&mut self) {
        for proposition in &mut self.proposition_stack {
            proposition.substitute(&self.proposition_values);
        }
    }

    // // Removes all the propositions which have been collapsed to "T" or "F"
    // pub fn trim(&mut self) {
    //     let mut removal_indexes: Vec<usize> = Vec::new();

    //     for (i, proposition) in self.proposition_stack.iter_mut().enumerate() {
    //         if proposition.get_string() == "T" || proposition.get_string() == "F" {
    //             removal_indexes.push(i);
    //         }
    //     }

    //     for i in removal_indexes {
    //         self.proposition_stack.remove(i);
    //     }
    // }

    // Finds and updates all of the actual truth values of the root propositions
    // Only finds values for propositions which have been collapsed to "p" or "!p"
    // Removes the propositions whose values have been determined from the stack
    pub fn update_actual_values(&mut self) {
        let mut removal_indexes: Vec<usize> = Vec::new();

        for (i, p) in self.proposition_stack.iter().enumerate() {
            let proposition_string = p.get_string();
            
            // The proposition can only be a root proposition value if it is one or two characters long
            match proposition_string.len() {
                1 => {
                    // If the proposition is one character long, the first and only character
                    // is the root proposition ("p" etc.) and its value is TRUE
                    let proposition_char = proposition_string.chars().next().unwrap();
                    self.proposition_values.set_value(proposition_char, Some(true));
                },
                2 => {
                    // If the proposition is two characters long, the second character
                    // is the root proposition ("!p" etc.) and its value is FALSE
                    let proposition_char = proposition_string.chars().last().expect("[INTERNAL ERROR] Could not find the last character of the proposition string");
                    self.proposition_values.set_value(proposition_char, Some(false));
                }
                _ => continue,
            };

            removal_indexes.push(i);
        }

        for i in removal_indexes {
            self.proposition_stack.remove(i);
        }
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
                if character.is_lowercase() {
                    values.insert(character, None);
                    // values.insert(character, Some(false));
                }
            }
        }

        Self { values }
    }

    // Returns the value map
    pub fn get_values(&self) -> &HashMap<char, Option<bool>> {
        &self.values
    }

    // Sets the value of a root proposition
    pub fn set_value(&mut self, proposition: char, value: Option<bool>) {
        self.values.insert(proposition, value);
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
