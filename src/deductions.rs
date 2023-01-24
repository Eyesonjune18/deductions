use std::collections::HashMap;

use crate::Premise;
use crate::PremiseNode;

// Stores all the given or working premises on a stack
pub struct Deduction {
    premise_stack: Vec<Premise>,
    proposition_values: ValueMap,
}

// Stores all known root proposition values in the Deduction
#[derive(Debug)]
pub struct ValueMap {
    values: HashMap<char, Option<bool>>,
}

impl std::fmt::Display for Deduction {
    // Displays all the propositions in the Deduction
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for premise in &self.premise_stack {
            writeln!(f, "{}", premise)?;
        }

        Ok(())
    }
}

impl Default for Deduction {
    // Creates an empty Deduction
    fn default() -> Self {
        Self {
            premise_stack: Vec::new(),
            proposition_values: ValueMap::default(),
        }
    }
}

impl Deduction {
    // Creates a new Deduction from the given fields
    fn new(premise_stack: Vec<Premise>, proposition_values: ValueMap) -> Self {
        Self {
            premise_stack,
            proposition_values,
        }
    }

    // Creates a Deduction from a vector of premises
    pub fn from_strs(premises: Vec<&str>) -> Self {
        let premise_stack: Vec<Premise> = premises.iter().map(|x| Premise::parse_str(x)).collect();
        let proposition_values = ValueMap::from_premise_stack(&premise_stack);

        Self::new(premise_stack, proposition_values)
    }

    // Checks if the Deduction is empty
    fn is_empty(&self) -> bool {
        self.premise_stack.is_empty()
    }

    // Returns the proposition values
    pub fn get_values(&self) -> &ValueMap {
        &self.proposition_values
    }

    // Substitutes all root propositions with their actual truth values, if known
    // * This should not be public but it is for testing purposes *
    pub fn substitute_all(&mut self) {
        for premise in &mut self.premise_stack {
            premise.substitute(&self.proposition_values);
        }
    }

    // Finds and updates all of the actual truth values of the root propositions
    // Only finds values for propositions which have been collapsed to "p" or "!p"
    // Removes the propositions whose values have been determined from the stack
    // * This should not be public but it is for testing purposes *
    // TODO: Write test cases for this!
    pub fn update_actual_values(&mut self) {
        for premise in &mut self.premise_stack {
            if let Some((proposition_char, proposition_value)) =
                premise.get_value_if_root_proposition()
            {
                self.proposition_values
                    .set_value(proposition_char, Some(proposition_value));
            }
        }
    }
}

impl Default for ValueMap {
    // Creates an empty ValueMap
    fn default() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}

impl ValueMap {
    // Creates a new ValueMap from the given fields
    fn new(values: HashMap<char, Option<bool>>) -> Self {
        Self { values }
    }

    // Finds all the root propositions in the given stack and initializes them to None
    // This is used to create a Deduction from a vector of propositions
    fn from_premise_stack(premise_stack: &Vec<Premise>) -> Self {
        let mut values = HashMap::new();

        fn inner<'a>(
            values: &mut HashMap<char, Option<bool>>,
            premise: impl Iterator<Item = &'a PremiseNode>,
        ) {
            for node in premise {
                match node {
                    PremiseNode::Proposition(proposition_char) => {
                        values.insert(*proposition_char, None);
                    }
                    PremiseNode::Subpremise(subpremise) => {
                        inner(values, subpremise.get_nodes().iter());
                    }
                    _ => (),
                }
            }
        }

        for premise in premise_stack {
            inner(&mut values, premise.get_nodes().iter());
        }

        Self { values }
    }

    // Gets the value of a root proposition, if known
    pub fn get_value(&self, proposition: char) -> Option<bool> {
        *self.values.get(&proposition)
        .expect(format!("[INTERNAL ERROR] Attempted to find the value of a proposition '{}' which does not exist in the ValueMap. Was it initialized correctly?", proposition).as_str())
    }

    // Sets the value of a root proposition
    pub fn set_value(&mut self, proposition: char, value: Option<bool>) {
        self.values.insert(proposition, value);
    }
}
