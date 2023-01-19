// Represents a propositional logic expression through an abstract syntax tree
pub struct Expression {
    origin_string: String,
    nodes: Vec<ExpressionNode>,
}

// Represents nodes in the expression tree
enum ExpressionNode {
    Proposition(char),
    TruthValue(bool),
    Operator(Operator),
    Subexpression(Expression),
}

// Represents one of 4 required operators for this project
enum Operator {
    Not,
    And,
    Or,
    Implies,
}

impl std::fmt::Display for Expression {
    // Displays the expression as a string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.origin_string)
    }
}

impl Expression {
    fn new() -> Self {
        Self {
            origin_string: String::new(),
            nodes: Vec::new(),
        }
    }

    fn from_str(expression_string: &str) -> Self {
        Self {
            origin_string: expression_string.to_string(),
            nodes: Vec::new(),
        }
    }
}
