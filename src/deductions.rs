use std::collections::HashMap;

use crate::Expression;
use crate::ExpressionNode;

// Stores all the given or working expressions on a stack
pub struct Deduction {
    expression_stack: Vec<Expression>,
    proposition_values: ValueMap,
}

// Stores all known root proposition values in the Deduction
pub struct ValueMap {
    values: HashMap<char, Option<bool>>,
}

impl std::fmt::Display for Deduction {
    // Displays all the propositions in the Deduction
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for expression in &self.expression_stack {
            writeln!(f, "{}", expression)?;
        }

        Ok(())
    }
}

impl Default for Deduction {
    // Creates an empty Deduction
    fn default() -> Self {
        Self {
            expression_stack: Vec::new(),
            proposition_values: ValueMap::default(),
        }
    }
}

impl Deduction {
    // Creates a new Deduction from the given fields
    fn new(expression_stack: Vec<Expression>, proposition_values: ValueMap) -> Self {
        Self {
            expression_stack,
            proposition_values,
        }
    }

    // Creates a Deduction from a vector of expressions
    pub fn from_strs(expressions: Vec<&str>) -> Self {
        let expression_stack: Vec<Expression> = expressions
            .iter()
            .map(|x| Expression::parse_str(x))
            .collect();
        let proposition_values = ValueMap::from_expression_stack(&expression_stack);

        Self::new(expression_stack, proposition_values)
    }

    // Checks if the Deduction is empty
    fn is_empty(&self) -> bool {
        self.expression_stack.is_empty()
    }

    // Substitutes all root propositions with their actual truth values, if known
    fn substitute_all(&mut self) {
        for expression in &mut self.expression_stack {
            expression.substitute(&self.proposition_values);
        }
    }

    // Finds and updates all of the actual truth values of the root propositions
    // Only finds values for propositions which have been collapsed to "p" or "!p"
    // Removes the propositions whose values have been determined from the stack
    fn update_actual_values(&mut self) {
        // TODO: Rewrite this

        // let mut removal_indexes: Vec<usize> = Vec::new();

        // for (i, p) in self.expression_stack.iter().enumerate() {
        //     let expression_string = p.get_string();

        //     // The proposition can only be a root proposition value if it is one or two characters long
        //     match expression_string.len() {
        //         1 => {
        //             // If the proposition is one character long, the first and only character
        //             // is the root proposition ("p" etc.) and its value is TRUE
        //             let proposition_char = expression_string.chars().next().unwrap();
        //             self.proposition_values
        //                 .set_value(proposition_char, Some(true));
        //         }
        //         2 => {
        //             // If the proposition is two characters long, the second character
        //             // is the root proposition ("!p" etc.) and its value is FALSE
        //             let proposition_char = expression_string.chars().last().expect("[INTERNAL ERROR] Could not find the last character of the proposition string");
        //             self.proposition_values
        //                 .set_value(proposition_char, Some(false));
        //         }
        //         _ => continue,
        //     };

        //     removal_indexes.push(i);
        // }

        // for i in removal_indexes {
        //     self.expression_stack.remove(i);
        // }
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
    fn from_expression_stack(expression_stack: &Vec<Expression>) -> Self {
        let mut values: HashMap<char, Option<bool>> = HashMap::new();

        for expression in expression_stack {
            for node in expression.get_nodes() {
                if let ExpressionNode::Proposition(proposition_char) = node {
                    values.insert(*proposition_char, None);
                }
            }
        }

        Self { values }
    }

    // Gets the value of a root proposition, if known
    pub fn get_value(&self, proposition: char) -> Option<bool> {
        *self.values.get(&proposition).expect("[INTERNAL ERROR] Attempted to find the value of a proposition which does not exist in the ValueMap")
    }

    // Sets the value of a root proposition
    fn set_value(&mut self, proposition: char, value: Option<bool>) {
        self.values.insert(proposition, value);
    }
}
