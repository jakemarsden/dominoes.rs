use super::Token::*;
use super::*;

#[test]
fn empty_document() {
    let input = "";

    let mock = Rc::new(RefCell::new(MockTokenConsumer::new()));
    let mut tokenizer = Tokenizer::new(String::from(input), mock.clone());
    tokenizer.exec();

    let mock = mock.borrow_mut();
    mock.assert_tokens_eq(&[EndOfFile]);
    mock.assert_no_errors();
}

#[test]
fn empty_html_tags() {
    let input = "<html></html>";

    let mock = Rc::new(RefCell::new(MockTokenConsumer::new()));
    let mut tokenizer = Tokenizer::new(String::from(input), mock.clone());
    tokenizer.exec();

    let mock = mock.borrow_mut();
    mock.assert_tokens_eq(&[
        Tag {
            kind: TagKind::Start,
            tag_name: String::from("html"),
            self_closing: false,
            attributes: Attributes::new(),
        },
        Tag {
            kind: TagKind::End,
            tag_name: String::from("html"),
            self_closing: false,
            attributes: Attributes::new(),
        },
        EndOfFile,
    ]);
    mock.assert_no_errors();
    assert_eq!(tokenizer.state, State::Data);
}

#[test]
fn dtd_less_doctype_decl() {
    let input = "<!DOCTYPE html>";

    let mock = Rc::new(RefCell::new(MockTokenConsumer::new()));
    let mut tokenizer = Tokenizer::new(String::from(input), mock.clone());
    tokenizer.exec();

    let mock = mock.borrow_mut();
    mock.assert_tokens_eq(&[
        Doctype {
            name: Some(String::from("html")),
            public_identifier: None,
            system_identifier: None,
            force_quirks: false,
        },
        EndOfFile,
    ]);
    mock.assert_no_errors();
    assert_eq!(tokenizer.state, State::Data);
}

#[test]
fn doctype_decl_with_legacy_public_identifier() {
    let input = "<!DOCTYPE html PUBLIC \"my 'public' identifier\">";

    let mock = Rc::new(RefCell::new(MockTokenConsumer::new()));
    let mut tokenizer = Tokenizer::new(String::from(input), mock.clone());
    tokenizer.exec();

    let mock = mock.borrow_mut();
    mock.assert_tokens_eq(&[
        Doctype {
            name: Some(String::from("html")),
            public_identifier: Some(String::from("my 'public' identifier")),
            system_identifier: None,
            force_quirks: false,
        },
        EndOfFile,
    ]);
    mock.assert_no_errors();
    assert_eq!(tokenizer.state, State::Data);
}

#[test]
fn doctype_decl_with_legacy_system_identifier() {
    let input = "<!DOCTYPE html SYSTEM \"my 'system' identifier\">";

    let mock = Rc::new(RefCell::new(MockTokenConsumer::new()));
    let mut tokenizer = Tokenizer::new(String::from(input), mock.clone());
    tokenizer.exec();

    let mock = mock.borrow_mut();
    mock.assert_tokens_eq(&[
        Doctype {
            name: Some(String::from("html")),
            public_identifier: None,
            system_identifier: Some(String::from("my 'system' identifier")),
            force_quirks: false,
        },
        EndOfFile,
    ]);
    mock.assert_no_errors();
    assert_eq!(tokenizer.state, State::Data);
}

#[test]
fn doctype_decl_with_legacy_public_and_system_identifiers() {
    let input = "<!DOCTYPE html PUBLIC \"my 'public' identifier\" \"my 'system' identifier\">";

    let mock = Rc::new(RefCell::new(MockTokenConsumer::new()));
    let mut tokenizer = Tokenizer::new(String::from(input), mock.clone());
    tokenizer.exec();

    let mock = mock.borrow_mut();
    mock.assert_tokens_eq(&[
        Doctype {
            name: Some(String::from("html")),
            public_identifier: Some(String::from("my 'public' identifier")),
            system_identifier: Some(String::from("my 'system' identifier")),
            force_quirks: false,
        },
        EndOfFile,
    ]);
    mock.assert_no_errors();
    assert_eq!(tokenizer.state, State::Data);
}

struct MockTokenConsumer {
    tokens: Vec<Token>,
    parse_errors: Vec<ParseError>,
}

impl MockTokenConsumer {
    fn new() -> Self {
        Self {
            tokens: Vec::new(),
            parse_errors: Vec::new(),
        }
    }

    fn assert_tokens_eq(&self, expected: &[Token]) {
        assert_eq!(self.tokens, expected);
    }

    fn assert_no_errors(&self) {
        assert!(self.parse_errors.is_empty());
    }
}

impl TokenConsumer for MockTokenConsumer {
    fn accept_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn accept_parse_error(&mut self, error: ParseError) {
        self.parse_errors.push(error);
    }
}
