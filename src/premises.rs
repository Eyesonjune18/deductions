use std::fmt::{Display, Formatter, Result};

use crate::ValueMap;

// Represents a propositional logic premise through an abstract syntax tree
#[derive(Debug, Eq, PartialEq)]
pub struct Premise {
    nodes: Vec<PremiseNode>,
}

// Represents nodes in the premise tree
#[derive(Debug, Eq, PartialEq)]
pub enum PremiseNode {
    Proposition(char),
    TruthValue(bool),
    Operator(Operator),
    Negation,
    Subpremise(Premise),
}

// Represents one of 4 required operators for this project
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Operator {
    And,
    Or,
    Implies,
}

impl Display for Premise {
    // Displays the premise as a string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Join all nodes together with a space except for not operators
        for (i, node) in self.nodes.iter().enumerate() {
            // Don't print a space before the first node
            if i == 0 {
                write!(f, "{}", node)?;
            } else {
                match self.nodes[i - 1] {
                    PremiseNode::Negation => write!(f, "{}", node)?,
                    _ => write!(f, " {}", node)?,
                }
            }
        }

        Ok(())
    }
}

impl Display for PremiseNode {
    // Displays the node as a string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PremiseNode::Proposition(proposition) => write!(f, "{}", proposition),
            PremiseNode::TruthValue(value) => write!(f, "{}", value),
            PremiseNode::Operator(operator) => write!(f, "{}", operator),
            PremiseNode::Negation => write!(f, "¬"),
            PremiseNode::Subpremise(subpremise) => write!(f, "({})", subpremise),
        }
    }
}

impl Display for Operator {
    // Displays the operator as a string
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Operator::And => write!(f, "∧"),
            Operator::Or => write!(f, "∨"),
            Operator::Implies => write!(f, "→"),
        }
    }
}

impl Premise {
    // Creates a new Premise from the given fields
    fn new(nodes: Vec<PremiseNode>) -> Self {
        Self { nodes }
    }

    // Creates an Premise from a string
    pub fn parse_str(premise_string: &str) -> Self {
        let mut nodes = Vec::new();

        let mut premise_chars = premise_string.char_indices();

        while let Some((i, c)) = premise_chars.next() {
            match c {
                ' ' => (),
                // If a subpremise is found, parse it recursively
                '(' => {
                    // Collect the subpremise string to be parsed
                    let subpremise_string = get_subpremise_string(&premise_string[i..]);

                    nodes.push(PremiseNode::Subpremise(Self::parse_str(
                        &subpremise_string,
                    )));

                    // Skip the characters in the subpremise
                    premise_chars.nth(subpremise_string.len());
                }
                ')' => (),
                '¬' | '!' => nodes.push(PremiseNode::Negation),
                '∧' | '&' => nodes.push(PremiseNode::Operator(Operator::And)),
                '∨' | '|' => nodes.push(PremiseNode::Operator(Operator::Or)),
                '→' | '>' => nodes.push(PremiseNode::Operator(Operator::Implies)),
                'a'..='z' => nodes.push(PremiseNode::Proposition(c)),
                _ => panic!("Invalid character in premise: '{}'", c),
            }
        }

        Self::new(nodes)
    }

    // Returns the nodes in the Premise
    pub fn get_nodes(&self) -> &Vec<PremiseNode> {
        &self.nodes
    }

    // Checks whether a given Premise is a root proposition such as "p" or "¬p",
    // and if it is, returns the proposition's character and its truth value
    pub fn get_value_if_root_proposition(&self) -> Option<(char, bool)> {
        match self.nodes.len() {
            1 => Some((self.nodes[0].is_proposition()?, true)),
            2 if self.nodes[0].is_negation() => {
                Some((self.nodes[1].is_proposition()?, false))
            }
            _ => None,
        }
    }

    // Substitutes all Proposition nodes with their actual truth values, if known
    pub fn substitute(&mut self, proposition_values: &ValueMap) {
        for node in &mut self.nodes {
            match node {
                PremiseNode::Proposition(proposition) => {
                    if let Some(value) = proposition_values.get_value(*proposition) {
                        *node = PremiseNode::TruthValue(value);
                    }
                }
                PremiseNode::Subpremise(subpremise) => {
                    subpremise.substitute(proposition_values);
                }
                _ => (),
            }
        }
    }

    // Simplifies the premise by removing all unnecessary nodes based on logical rules
    // ? Does this need to be public?
    pub fn simplify(&mut self) {
        // Simplify all subpremises
        for node in &mut self.nodes {
            if let PremiseNode::Subpremise(subpremise) = node {
                subpremise.simplify();
            }
        }

        unimplemented!()
    }
}

// Returns the first subpremise found in the given premise string
fn get_subpremise_string(premise_string: &str) -> String {
    let mut subpremise_string = String::new();
    let mut depth = 0;

    for character in premise_string.chars() {
        match character {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => (),
        }

        // If the depth is 0, the subpremise has been collected
        if depth == 0 {
            break;
        }

        // Do not add the open parenthesis to the subpremise string
        if depth == 1 && character == '(' {
            continue;
        }

        subpremise_string.push(character);
    }

    subpremise_string
}

impl PremiseNode {
    fn is_proposition(&self) -> Option<char> {
        match self {
            PremiseNode::Proposition(p) => Some(*p),
            _ => None,
        }
    }

    fn is_operator(&self) -> Option<Operator> {
        match self {
            PremiseNode::Operator(o) => Some(*o),
            _ => None,
        }
    }

    fn is_negation(&self) -> bool {
        match self {
            PremiseNode::Negation => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let premise = Premise::parse_str("a");

        assert_eq!(premise.get_nodes().len(), 1);
        assert_eq!(premise.get_nodes()[0], PremiseNode::Proposition('a'));
    }

    #[test]
    fn test_parse_complex_1() {
        let premise = Premise::parse_str("a ∧ b ∨ (c → d)");

        assert_eq!(premise.get_nodes().len(), 5);
        assert_eq!(premise.get_nodes()[0], PremiseNode::Proposition('a'));
        assert_eq!(
            premise.get_nodes()[1],
            PremiseNode::Operator(Operator::And)
        );
        assert_eq!(premise.get_nodes()[2], PremiseNode::Proposition('b'));
        assert_eq!(
            premise.get_nodes()[3],
            PremiseNode::Operator(Operator::Or)
        );
        assert!(matches!(
            premise.get_nodes()[4],
            PremiseNode::Subpremise(_)
        ));

        if let PremiseNode::Subpremise(subpremise) = &premise.get_nodes()[4] {
            assert_eq!(subpremise.get_nodes().len(), 3);
            assert_eq!(
                subpremise.get_nodes()[0],
                PremiseNode::Proposition('c')
            );
            assert_eq!(
                subpremise.get_nodes()[1],
                PremiseNode::Operator(Operator::Implies)
            );
            assert_eq!(
                subpremise.get_nodes()[2],
                PremiseNode::Proposition('d')
            );
        }
    }

    #[test]
    fn test_parse_complex_2() {
        let premise = Premise::parse_str("(m & b) > j");

        assert_eq!(premise.get_nodes().len(), 3);

        assert!(matches!(
            premise.get_nodes()[0],
            PremiseNode::Subpremise(_)
        ));

        if let PremiseNode::Subpremise(subpremise) = &premise.get_nodes()[0] {
            assert_eq!(subpremise.get_nodes().len(), 3);
            assert_eq!(
                subpremise.get_nodes()[0],
                PremiseNode::Proposition('m')
            );
            assert_eq!(
                subpremise.get_nodes()[1],
                PremiseNode::Operator(Operator::And)
            );
            assert_eq!(
                subpremise.get_nodes()[2],
                PremiseNode::Proposition('b')
            );
        }

        assert_eq!(
            premise.get_nodes()[1],
            PremiseNode::Operator(Operator::Implies)
        );

        assert_eq!(premise.get_nodes()[2], PremiseNode::Proposition('j'));
    }

    #[test]
    fn test_substitute() {
        let mut premise = Premise::parse_str("a ∧ b ∨ (c → d)");

        let mut proposition_values = ValueMap::default();
        proposition_values.set_value('a', Some(true));
        proposition_values.set_value('b', None);
        proposition_values.set_value('c', Some(false));
        proposition_values.set_value('d', None);

        premise.substitute(&proposition_values);

        assert_eq!(premise.get_nodes().len(), 5);
        assert_eq!(premise.get_nodes()[0], PremiseNode::TruthValue(true));
        assert_eq!(
            premise.get_nodes()[1],
            PremiseNode::Operator(Operator::And)
        );
        assert_eq!(premise.get_nodes()[2], PremiseNode::Proposition('b'));
        assert_eq!(
            premise.get_nodes()[3],
            PremiseNode::Operator(Operator::Or)
        );
        assert!(matches!(
            premise.get_nodes()[4],
            PremiseNode::Subpremise(_)
        ));

        if let PremiseNode::Subpremise(subpremise) = &premise.get_nodes()[4] {
            assert_eq!(subpremise.get_nodes().len(), 3);
            assert_eq!(
                subpremise.get_nodes()[0],
                PremiseNode::TruthValue(false)
            );
            assert_eq!(
                subpremise.get_nodes()[1],
                PremiseNode::Operator(Operator::Implies)
            );
            assert_eq!(
                subpremise.get_nodes()[2],
                PremiseNode::Proposition('d')
            );
        }
    }
}
