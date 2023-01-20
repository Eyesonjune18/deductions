use std::fmt::{Display, Formatter, Result};

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

impl Display for Expression {
    // Displays the expression as a string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Join all nodes together with a space except for not operators
        for (i, node) in self.nodes.iter().enumerate() {
            // Don't print a space before the first node
            if i == 0 {
                write!(f, "{}", node)?;
            } else {
                match self.nodes[i - 1] {
                    ExpressionNode::Operator(Operator::Not) => write!(f, "{}", node)?,
                    _ => write!(f, " {}", node)?,
                }
            }
        }

        Ok(())
    }
}

impl Display for ExpressionNode {
    // Displays the node as a string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionNode::Proposition(proposition) => write!(f, "{}", proposition),
            ExpressionNode::TruthValue(value) => write!(f, "{}", value),
            ExpressionNode::Operator(operator) => write!(f, "{}", operator),
            ExpressionNode::Subexpression(subexpression) => write!(f, "({})", subexpression),
        }
    }
}

impl Display for Operator {
    // Displays the operator as a string
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Operator::Not => write!(f, "¬"),
            Operator::And => write!(f, "∧"),
            Operator::Or => write!(f, "∨"),
            Operator::Implies => write!(f, "→"),
        }
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

                    nodes.push(ExpressionNode::Subexpression(Self::parse_str(
                        &subexpression_string,
                    )));

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
    let mut depth = 0;

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

        // Do not add the open parenthesis to the subexpression string
        if depth == 1 && character == '(' {
            continue;
        }

        subexpression_string.push(character);
    }

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
    fn test_parse_complex_1() {
        let expression = Expression::parse_str("a ∧ b ∨ (c → d)");

        assert_eq!(expression.get_nodes().len(), 5);
        assert_eq!(expression.get_nodes()[0], ExpressionNode::Proposition('a'));
        assert_eq!(
            expression.get_nodes()[1],
            ExpressionNode::Operator(Operator::And)
        );
        assert_eq!(expression.get_nodes()[2], ExpressionNode::Proposition('b'));
        assert_eq!(
            expression.get_nodes()[3],
            ExpressionNode::Operator(Operator::Or)
        );
        assert!(matches!(
            expression.get_nodes()[4],
            ExpressionNode::Subexpression(_)
        ));

        if let ExpressionNode::Subexpression(subexpression) = &expression.get_nodes()[4] {
            assert_eq!(subexpression.get_nodes().len(), 3);
            assert_eq!(
                subexpression.get_nodes()[0],
                ExpressionNode::Proposition('c')
            );
            assert_eq!(
                subexpression.get_nodes()[1],
                ExpressionNode::Operator(Operator::Implies)
            );
            assert_eq!(
                subexpression.get_nodes()[2],
                ExpressionNode::Proposition('d')
            );
        }
    }

    #[test]
    fn test_parse_complex_2() {
        let expression = Expression::parse_str("(m & b) > j");

        assert_eq!(expression.get_nodes().len(), 3);

        assert!(matches!(
            expression.get_nodes()[0],
            ExpressionNode::Subexpression(_)
        ));

        if let ExpressionNode::Subexpression(subexpression) = &expression.get_nodes()[0] {
            assert_eq!(subexpression.get_nodes().len(), 3);
            assert_eq!(
                subexpression.get_nodes()[0],
                ExpressionNode::Proposition('m')
            );
            assert_eq!(
                subexpression.get_nodes()[1],
                ExpressionNode::Operator(Operator::And)
            );
            assert_eq!(
                subexpression.get_nodes()[2],
                ExpressionNode::Proposition('b')
            );
        }

        assert_eq!(
            expression.get_nodes()[1],
            ExpressionNode::Operator(Operator::Implies)
        );
        
        assert_eq!(expression.get_nodes()[2], ExpressionNode::Proposition('j'));
    }

    #[test]
    fn test_substitute() {
        let mut expression = Expression::parse_str("a ∧ b ∨ (c → d)");

        let mut proposition_values = ValueMap::default();
        proposition_values.set_value('a', Some(true));
        proposition_values.set_value('b', None);
        proposition_values.set_value('c', Some(false));
        proposition_values.set_value('d', None);

        expression.substitute(&proposition_values);

        assert_eq!(expression.get_nodes().len(), 5);
        assert_eq!(expression.get_nodes()[0], ExpressionNode::TruthValue(true));
        assert_eq!(
            expression.get_nodes()[1],
            ExpressionNode::Operator(Operator::And)
        );
        assert_eq!(expression.get_nodes()[2], ExpressionNode::Proposition('b'));
        assert_eq!(
            expression.get_nodes()[3],
            ExpressionNode::Operator(Operator::Or)
        );
        assert!(matches!(
            expression.get_nodes()[4],
            ExpressionNode::Subexpression(_)
        ));

        if let ExpressionNode::Subexpression(subexpression) = &expression.get_nodes()[4] {
            assert_eq!(subexpression.get_nodes().len(), 3);
            assert_eq!(
                subexpression.get_nodes()[0],
                ExpressionNode::TruthValue(false)
            );
            assert_eq!(
                subexpression.get_nodes()[1],
                ExpressionNode::Operator(Operator::Implies)
            );
            assert_eq!(
                subexpression.get_nodes()[2],
                ExpressionNode::Proposition('d')
            );
        }
    }
}
