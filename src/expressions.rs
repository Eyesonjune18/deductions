use crate::ValueMap;

// Represents a propositional logic expression through an abstract syntax tree
#[derive(Debug, Eq, PartialEq)]
pub struct Expression {
    origin_string: String,
    nodes: Vec<ExpressionNode>,
}

// Represents nodes in the expression tree
#[derive(Debug, Eq, PartialEq)]
pub enum ExpressionNode {
    Proposition(char),
    TruthValue(bool),
    Operator(Operator),
    Subexpression(Expression),
}

// Represents one of 4 required operators for this project
#[derive(Debug, Eq, PartialEq)]
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
        let mut nodes = Vec::new();

        let mut expression_chars = expression_string.char_indices();

        while let Some((i, c)) = expression_chars.next() {
            match c {
                ' ' => (),
                // If a subexpression is found, parse it recursively
                '(' => {
                    // Collect the subexpression string to be parsed
                    let subexpression_string = get_subexpression_string(&expression_string[i..]);

                    nodes.push(ExpressionNode::Subexpression(Self::parse_str(&subexpression_string)));

                    // Skip the characters in the subexpression
                    expression_chars.nth(subexpression_string.len());
                }
                ')' => (),
                '¬' | '!' => nodes.push(ExpressionNode::Operator(Operator::Not)),
                '∧' | '&' => nodes.push(ExpressionNode::Operator(Operator::And)),
                '∨' | '|' => nodes.push(ExpressionNode::Operator(Operator::Or)),
                '→' | '>' => nodes.push(ExpressionNode::Operator(Operator::Implies)),
                'a'..='z' => nodes.push(ExpressionNode::Proposition(c)),
                _ => panic!("Invalid character in expression: '{}'", c),
            }
        }

        Self::new(expression_string.to_string(), nodes)
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

// Returns the first subexpression found in the given expression string
fn get_subexpression_string(expression_string: &str) -> String {
    let mut subexpression_string = String::new();
    let mut depth = 1;

    for character in expression_string.chars() {
        match character {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => (),
        }
        
        // If the depth is 0, the subexpression has been collected
        if depth == 0 {
            break;
        }

        subexpression_string.push(character);
    }

    // Remove the parentheses from the subexpression
    subexpression_string.remove(0);
    subexpression_string.pop();

    subexpression_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let expression = Expression::parse_str("a");

        assert_eq!(expression.get_nodes().len(), 1);
        assert_eq!(expression.get_nodes()[0], ExpressionNode::Proposition('a'));
    }

    #[test]
    fn test_parse_complex() {
        let expression = Expression::parse_str("a ∧ b ∨ (c → d)");

        assert_eq!(expression.get_nodes().len(), 5);
        assert_eq!(expression.get_nodes()[0], ExpressionNode::Proposition('a'));
        assert_eq!(expression.get_nodes()[1], ExpressionNode::Operator(Operator::And));
        assert_eq!(expression.get_nodes()[2], ExpressionNode::Proposition('b'));
        assert_eq!(expression.get_nodes()[3], ExpressionNode::Operator(Operator::Or));
        assert!(matches!(expression.get_nodes()[4], ExpressionNode::Subexpression(_)));

        if let ExpressionNode::Subexpression(subexpression) = &expression.get_nodes()[4] {
            assert_eq!(subexpression.get_nodes().len(), 3);
            assert_eq!(subexpression.get_nodes()[0], ExpressionNode::Proposition('c'));
            assert_eq!(subexpression.get_nodes()[1], ExpressionNode::Operator(Operator::Implies));
            assert_eq!(subexpression.get_nodes()[2], ExpressionNode::Proposition('d'));
        }
    }
}
