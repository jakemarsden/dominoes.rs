use super::ParseError::*;
use super::Token::*;
use super::*;

#[test]
fn empty_document() {
    let input = "";
    let (tokenizer, actual) = create_and_exec_tokenizer(input);

    let actual = actual.borrow();
    assert_eq!(actual.tokens, &[EndOfFile]);
    assert_eq!(actual.parse_errors, &[]);
    assert_eq!(tokenizer.state, State::Data);
}

#[test]
fn empty_html_tags() {
    let input = "<html></html>";
    let (tokenizer, actual) = create_and_exec_tokenizer(input);

    let actual = actual.borrow();
    let expected_tokens = [
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
    ];
    assert_eq!(actual.tokens, expected_tokens);
    assert_eq!(actual.parse_errors, &[]);
    assert_eq!(tokenizer.state, State::Data);
}

#[test]
fn dtd_less_doctype_decl() {
    let input = "<!DOCTYPE html>";
    let (tokenizer, actual) = create_and_exec_tokenizer(input);

    let actual = actual.borrow();
    let expected_tokens = [
        Doctype {
            name: Some(String::from("html")),
            public_identifier: None,
            system_identifier: None,
            force_quirks: false,
        },
        EndOfFile,
    ];
    assert_eq!(actual.tokens, expected_tokens);
    assert_eq!(actual.parse_errors, &[]);
    assert_eq!(tokenizer.state, State::Data);
}

#[test]
fn doctype_decl_with_legacy_public_identifier() {
    let input = "<!DOCTYPE html PUBLIC \"my 'public' identifier\">";
    let (tokenizer, actual) = create_and_exec_tokenizer(input);

    let actual = actual.borrow();
    let expected_tokens = [
        Doctype {
            name: Some(String::from("html")),
            public_identifier: Some(String::from("my 'public' identifier")),
            system_identifier: None,
            force_quirks: false,
        },
        EndOfFile,
    ];
    assert_eq!(actual.tokens, expected_tokens);
    assert_eq!(actual.parse_errors, &[]);
    assert_eq!(tokenizer.state, State::Data);
}

#[test]
fn doctype_decl_with_legacy_system_identifier() {
    let input = "<!DOCTYPE html SYSTEM \"my 'system' identifier\">";
    let (tokenizer, actual) = create_and_exec_tokenizer(input);

    let actual = actual.borrow();
    let expected_tokens = [
        Doctype {
            name: Some(String::from("html")),
            public_identifier: None,
            system_identifier: Some(String::from("my 'system' identifier")),
            force_quirks: false,
        },
        EndOfFile,
    ];
    assert_eq!(actual.tokens, expected_tokens);
    assert_eq!(actual.parse_errors, &[]);
    assert_eq!(tokenizer.state, State::Data);
}

#[test]
fn doctype_decl_with_legacy_public_and_system_identifiers() {
    let input = "<!DOCTYPE html PUBLIC \"my 'public' identifier\" \"my 'system' identifier\">";
    let (tokenizer, actual) = create_and_exec_tokenizer(input);

    let actual = actual.borrow();
    let expected_tokens = [
        Doctype {
            name: Some(String::from("html")),
            public_identifier: Some(String::from("my 'public' identifier")),
            system_identifier: Some(String::from("my 'system' identifier")),
            force_quirks: false,
        },
        EndOfFile,
    ];
    assert_eq!(actual.tokens, expected_tokens);
    assert_eq!(actual.parse_errors, &[]);
    assert_eq!(tokenizer.state, State::Data);
}

#[test]
fn comment() {
    let input = "<!-- This - is -- a -> comment! -->";
    let (tokenizer, actual) = create_and_exec_tokenizer(input);

    let actual = actual.borrow();
    let expected_tokens = [
        Comment {
            data: String::from(" This - is -- a -> comment! "),
        },
        EndOfFile,
    ];
    assert_eq!(actual.tokens, expected_tokens);
    assert_eq!(actual.parse_errors, &[]);
    assert_eq!(tokenizer.state, State::Data);
}

#[test]
fn nested_comment_parse_error() {
    let input = "<!-- This is a <!-- nested comment -->";
    let (tokenizer, actual) = create_and_exec_tokenizer(input);

    let actual = actual.borrow();
    let expected_tokens = [
        Comment {
            data: String::from(" This is a <!-- nested comment "),
        },
        EndOfFile,
    ];
    assert_eq!(actual.tokens, expected_tokens);
    assert_eq!(actual.parse_errors, &[NestedComment]);
    assert_eq!(tokenizer.state, State::Data);
}

fn create_and_exec_tokenizer(input: &str) -> (Tokenizer, Rc<RefCell<MockTokenConsumer>>) {
    let mock_output = MockTokenConsumer::create_ref_counted();
    let mut tokenizer = Tokenizer::new(input.into(), mock_output.clone());
    tokenizer.exec();

    (tokenizer, mock_output)
}

struct MockTokenConsumer {
    tokens: Vec<Token>,
    parse_errors: Vec<ParseError>,
}

impl MockTokenConsumer {
    fn create_ref_counted() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new()))
    }

    fn new() -> Self {
        Self {
            tokens: Vec::new(),
            parse_errors: Vec::new(),
        }
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
