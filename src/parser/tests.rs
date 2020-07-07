use crate::dom::Node;
use crate::parser::Parser;
use crate::tokenizer::result::Result;
use crate::tokenizer::Token::*;
use crate::tokenizer::{Token, Tokenizer};

// TODO: #[test]
fn empty_document() {
    let mock_tokenizer = MockTokenizer::new(vec![Ok(EndOfFile)]);
    let mut parser = Parser::new(mock_tokenizer);

    let actual = parser.parse();

    let expected = Node::create_document();
    assert_eq!(&actual, &expected);
}

struct MockTokenizer {
    tokens: Vec<Result<Token>>,
}

impl MockTokenizer {
    pub fn new(tokens: Vec<Result<Token>>) -> Self {
        Self { tokens }
    }
}

impl Iterator for MockTokenizer {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.pop()
    }
}

impl Tokenizer for MockTokenizer {}
