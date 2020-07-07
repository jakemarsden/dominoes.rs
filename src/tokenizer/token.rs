use std::fmt::Debug;

#[derive(Clone, PartialEq, Debug)]
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
    Character(char),
    EndOfFile,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TagKind {
    Start,
    End,
}

#[derive(Clone, Debug)]
pub struct Attributes {
    attrs: Vec<Attribute>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Attribute(String, String);

pub(in crate::tokenizer) trait IncompleteToken: Debug + Into<Token> {}

#[derive(Debug)]
pub(in crate::tokenizer) struct IncompleteDoctype {
    pub(in crate::tokenizer) name: Option<String>,
    pub(in crate::tokenizer) public_identifier: Option<String>,
    pub(in crate::tokenizer) system_identifier: Option<String>,
    pub(in crate::tokenizer) force_quirks: bool,
}

#[derive(Debug)]
pub(in crate::tokenizer) struct IncompleteTag {
    pub(in crate::tokenizer) kind: TagKind,
    pub(in crate::tokenizer) tag_name: String,
    pub(in crate::tokenizer) self_closing: bool,
    pub(in crate::tokenizer) attributes: Attributes,
}

#[derive(Debug)]
pub(in crate::tokenizer) struct IncompleteComment {
    pub(in crate::tokenizer) data: String,
}

impl From<char> for Token {
    fn from(data: char) -> Self {
        Self::Character(data)
    }
}

impl Attributes {
    pub(in crate::tokenizer) fn new() -> Self {
        Self { attrs: Vec::new() }
    }
}

impl PartialEq<Self> for Attributes {
    //! Does not care about order
    fn eq(&self, other: &Attributes) -> bool {
        // TODO: Could be more efficient
        let mut attrs = self.attrs.clone();
        let mut other_attrs = other.attrs.clone();

        // Need to sort by value as well as by name. Sorting by name only could be unstable if a
        // collection has multiple attributes with the same name but different values.
        let comparator =
            |a: &Attribute, b: &Attribute| a.name().cmp(b.name()).then(a.value().cmp(b.value()));
        attrs.sort_by(comparator);
        other_attrs.sort_by(comparator);
        attrs == other_attrs
    }
}

impl Attribute {
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

impl IncompleteTag {
    pub(in crate::tokenizer) fn default(kind: TagKind) -> Self {
        Self {
            kind,
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
