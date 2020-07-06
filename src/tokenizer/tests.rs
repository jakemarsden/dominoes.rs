use super::error::ParseError::*;
use super::token::Token::*;
use super::token::{Attributes, TagKind};
use super::Tokenizer;

#[test]
fn empty_document() {
    let input = "";
    let tokenizer = Tokenizer::new(input.into());

    let actual: Vec<_> = tokenizer.collect();

    let expected = [Ok(EndOfFile)];
    assert_eq!(actual, &expected);
}

#[test]
fn empty_html_tags() {
    let input = "<html></html>";
    let tokenizer = Tokenizer::new(input.into());

    let actual: Vec<_> = tokenizer.collect();

    let expected = [
        Ok(Tag {
            kind: TagKind::Start,
            tag_name: String::from("html"),
            self_closing: false,
            attributes: Attributes::new(),
        }),
        Ok(Tag {
            kind: TagKind::End,
            tag_name: String::from("html"),
            self_closing: false,
            attributes: Attributes::new(),
        }),
        Ok(EndOfFile),
    ];
    assert_eq!(actual, &expected);
}

#[test]
fn dtd_less_doctype_decl() {
    let input = "<!DOCTYPE html>";
    let tokenizer = Tokenizer::new(input.into());

    let actual: Vec<_> = tokenizer.collect();

    let expected = [
        Ok(Doctype {
            name: Some(String::from("html")),
            public_identifier: None,
            system_identifier: None,
            force_quirks: false,
        }),
        Ok(EndOfFile),
    ];
    assert_eq!(actual, &expected);
}

#[test]
fn doctype_decl_with_legacy_public_identifier() {
    let input = "<!DOCTYPE html PUBLIC \"my 'public' identifier\">";
    let tokenizer = Tokenizer::new(input.into());

    let actual: Vec<_> = tokenizer.collect();

    let expected = [
        Ok(Doctype {
            name: Some(String::from("html")),
            public_identifier: Some(String::from("my 'public' identifier")),
            system_identifier: None,
            force_quirks: false,
        }),
        Ok(EndOfFile),
    ];
    assert_eq!(actual, &expected);
}

#[test]
fn doctype_decl_with_legacy_system_identifier() {
    let input = "<!DOCTYPE html SYSTEM \"my 'system' identifier\">";
    let tokenizer = Tokenizer::new(input.into());

    let actual: Vec<_> = tokenizer.collect();

    let expected = [
        Ok(Doctype {
            name: Some(String::from("html")),
            public_identifier: None,
            system_identifier: Some(String::from("my 'system' identifier")),
            force_quirks: false,
        }),
        Ok(EndOfFile),
    ];
    assert_eq!(actual, &expected);
}

#[test]
fn doctype_decl_with_legacy_public_and_system_identifiers() {
    let input = "<!DOCTYPE html PUBLIC \"my 'public' identifier\" \"my 'system' identifier\">";
    let tokenizer = Tokenizer::new(input.into());

    let actual: Vec<_> = tokenizer.collect();

    let expected = [
        Ok(Doctype {
            name: Some(String::from("html")),
            public_identifier: Some(String::from("my 'public' identifier")),
            system_identifier: Some(String::from("my 'system' identifier")),
            force_quirks: false,
        }),
        Ok(EndOfFile),
    ];
    assert_eq!(actual, &expected);
}

#[test]
fn comment() {
    let input = "<!-- This - is -- a -> comment! -->";
    let tokenizer = Tokenizer::new(input.into());

    let actual: Vec<_> = tokenizer.collect();

    let expected = [
        Ok(Comment {
            data: String::from(" This - is -- a -> comment! "),
        }),
        Ok(EndOfFile),
    ];
    assert_eq!(actual, &expected);
}

#[test]
fn nested_comment_parse_error() {
    let input = "<!-- This is a <!-- nested comment -->";
    let tokenizer = Tokenizer::new(input.into());

    let actual: Vec<_> = tokenizer.collect();

    let expected = [
        Err(NestedComment),
        Ok(Comment {
            data: String::from(" This is a <!-- nested comment "),
        }),
        Ok(EndOfFile),
    ];
    assert_eq!(actual, &expected);
}
