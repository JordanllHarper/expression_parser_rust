use crate::lexer::*;

#[derive(PartialEq, Eq, Debug)]
enum Node {
    Value(i32),
    Expression {
        operator: Operator,
        lhs: Option<Box<Node>>,
        rhs: Option<Box<Node>>,
    },
}

fn map_option_value_in_box<T>(source: Option<T>) -> Option<Box<T>> {
    source.map(|t| Box::new(t))
}

impl Node {
    pub fn new_expression(operator: Operator, lhs: Option<Node>, rhs: Option<Node>) -> Node {
        Self::Expression {
            operator,
            lhs: map_option_value_in_box(lhs),
            rhs: map_option_value_in_box(rhs),
        }
    }
    pub fn new_some_value_boxed(i: i32) -> Option<Box<Node>> {
        Some(Box::new(Self::Value(i)))
    }

    pub fn new_some_value(i: i32) -> Option<Node> {
        Some(Self::Value(i))
    }
}

// 2 + 2
//  ==
// expression:
// lhs: value 2
// rhs: value 2
// operator: +

// 2 -> value node: 2
// + -> expression:
//   lhs node 2
//   rhs ???
//   operator: +
//

fn recursive_parse(
    tokens: &[Token],
    index: usize,
    previous_node: Option<Node>,
) -> (Option<Node>, usize) {
    let token_at_index = tokens.get(index);
    if let Some(t) = token_at_index {
        //  + 2 + 2
        let n = match t {
            Token::IntegerNum(i) => todo!(),
            Token::Operator(o) => todo!(),
            Token::Paren(p) => todo!(), // (
        };
        (Some(n), index)
    } else {
        (None, index)
    }
}
pub fn parse(tokens: Vec<Token>) -> Option<Node> {
    let nodes = recursive_parse(&tokens, 0, None);
    nodes.0
}

#[cfg(test)]
mod test {
    use super::{parse, Operator};
    use crate::{lexer::Token, parser::Node};

    #[test]
    fn single_value_node() {
        let data = vec![Token::IntegerNum(2)];
        let expected = Node::Value(2);
        let actual = parse(data).unwrap();
        assert_eq!(expected, actual);
    }
    #[test]
    fn operator_and_single_value() {
        let data = vec![Token::Operator(super::Operator::Add), Token::IntegerNum(2)];
        let expected = Node::Expression {
            operator: Operator::Add,
            lhs: Node::new_some_value_boxed(2),
            rhs: None,
        };
        let actual = parse(data).unwrap();
        assert_eq!(expected, actual);
    }
}
