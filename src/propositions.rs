// Encodes a proposition string
pub struct Proposition {
    proposition_string: String,
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
}
