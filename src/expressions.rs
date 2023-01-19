use crate::ValueMap;

// Represents a propositional logic expression through an abstract syntax tree
pub struct Expression {
    origin_string: String,
    nodes: Vec<ExpressionNode>,
}

// Represents nodes in the expression tree
pub enum ExpressionNode {
    Proposition(char),
    TruthValue(bool),
    Operator(Operator),
    Subexpression(Expression),
}

// Represents one of 4 required operators for this project
pub enum Operator {
    Not,
    And,
    Or,
    Implies,
}

impl std::fmt::Display for Expression {
    // Displays the expression as a string
    // TODO: Implement Display for each node so the current version
    // can be displayed, rather than just the original string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.origin_string)
    }
}

impl Expression {
    // Creates a new Expression from the given fields
    fn new(origin_string: String, nodes: Vec<ExpressionNode>) -> Self {
        Self {
            origin_string,
            nodes,
        }
    }

    // Creates an Expression from a string
    pub fn parse_str(expression_string: &str) -> Self {
        todo!()
    }

    // Returns the nodes in the Expression
    pub fn get_nodes(&self) -> &Vec<ExpressionNode> {
        &self.nodes
    }

    // Substitutes all Proposition nodes with their actual truth values, if known
    pub fn substitute(&mut self, proposition_values: &ValueMap) {
        for node in &mut self.nodes {
            match node {
                ExpressionNode::Proposition(proposition) => {
                    if let Some(value) = proposition_values.get_value(*proposition) {
                        *node = ExpressionNode::TruthValue(value);
                    }
                }
                ExpressionNode::Subexpression(subexpression) => {
                    subexpression.substitute(proposition_values);
                }
                _ => (),
            }
        }
    }
}
