use std::fmt::Debug;

#[derive(Debug)]
pub enum Token {
    Doctype {
        name: Option<String>,
        public_identifier: Option<String>,
        system_identifier: Option<String>,
        force_quirks: bool,
    },
    Tag {
        kind: TagKind,
        tag_name: String,
        self_closing: bool,
        attributes: Attributes,
    },
    Comment {
        data: String,
    },
    Character {
        data: char,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TagKind {
    Start,
    End,
}

#[derive(Debug)]
pub struct Attributes {
    attrs: Vec<Attribute>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Attribute(String, String);

pub(in crate::tokenizer) trait IncompleteToken:
    Debug + Default + Into<Token>
{
}

#[derive(Debug)]
pub(in crate::tokenizer) struct IncompleteDoctype {
    name: Option<String>,
    public_identifier: Option<String>,
    system_identifier: Option<String>,
    force_quirks: bool,
}

#[derive(Debug)]
pub(in crate::tokenizer) struct IncompleteTag {
    kind: TagKind,
    tag_name: String,
    self_closing: bool,
    attributes: Attributes,
}

#[derive(Debug)]
pub(in crate::tokenizer) struct IncompleteComment {
    data: String,
}

impl Attributes {
    pub(in crate::tokenizer) fn new() -> Self {
        Self { attrs: Vec::new() }
    }

    pub(in crate::tokenizer) fn push(&mut self, attr: Attribute) {
        self.attrs.push(attr);
    }
}

impl Attribute {
    pub(in crate::tokenizer) fn new(name: &str, value: &str) -> Self {
        Self(String::from(name), String::from(value))
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn value(&self) -> &str {
        &self.1
    }
}

impl Default for IncompleteDoctype {
    fn default() -> Self {
        Self {
            name: None,
            public_identifier: None,
            system_identifier: None,
            force_quirks: false,
        }
    }
}

impl Into<Token> for IncompleteDoctype {
    fn into(self) -> Token {
        debug_assert!(!is_empty_some(&self.name));
        debug_assert!(!is_empty_some(&self.public_identifier));
        debug_assert!(!is_empty_some(&self.system_identifier));
        Token::Doctype {
            name: self.name,
            public_identifier: self.public_identifier,
            system_identifier: self.system_identifier,
            force_quirks: self.force_quirks,
        }
    }
}

impl Default for IncompleteTag {
    fn default() -> Self {
        Self {
            kind: TagKind::Start,
            tag_name: String::new(),
            self_closing: false,
            attributes: Attributes::new(),
        }
    }
}

impl Into<Token> for IncompleteTag {
    fn into(self) -> Token {
        debug_assert!(!self.tag_name.is_empty());
        Token::Tag {
            kind: self.kind,
            tag_name: self.tag_name,
            self_closing: self.self_closing,
            attributes: self.attributes,
        }
    }
}

impl Default for IncompleteComment {
    fn default() -> Self {
        Self {
            data: String::new(),
        }
    }
}

impl Into<Token> for IncompleteComment {
    fn into(self) -> Token {
        debug_assert!(!self.data.is_empty());
        Token::Comment { data: self.data }
    }
}

impl IncompleteToken for IncompleteDoctype {}

impl IncompleteToken for IncompleteTag {}

impl IncompleteToken for IncompleteComment {}

fn is_empty_some(value: &Option<String>) -> bool {
    if let Some(it) = value {
        it.is_empty()
    } else {
        false
    }
}
