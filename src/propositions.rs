use crate::ValueMap;

// Encodes a proposition string
pub struct Proposition {
    proposition_string: String,
}

impl std::fmt::Display for Proposition {
    // Displays the proposition string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.proposition_string)
    }
}

impl Proposition {
    // Creates an empty Proposition
    fn new() -> Self {
        Self {
            proposition_string: String::new(),
        }
    }

    // Creates a Proposition from a string
    pub fn from_str(proposition_string: &str) -> Self {
        Self {
            proposition_string: proposition_string.to_string(),
        }
    }

    // Returns the proposition string
    pub fn get_string(&self) -> &str {
        &self.proposition_string
    }

    // Substitutes all the root propositions with their actual
    // truth values (encoded in "TRUE" and "FALSE") if known
    // TODO: This might be better represented by a data structure rather than Strings
    pub fn substitute(&mut self, values: &ValueMap) {
        for (proposition_char, value) in values.get_values() {
            if let Some(value) = value {
                let value = if *value { "TRUE" } else { "FALSE" };

                self.proposition_string = self.proposition_string.replace(*proposition_char, value);
            }
        }
    }

    // Collapses the proposition string based on actual truth values
    pub fn collapse(&mut self) {
        todo!()
    }
}
