use std::convert::TryInto;

pub(in crate::tokenizer) struct CodepointStream {
    source: String,
    cursor: usize,
}

#[derive(Clone, Copy, Debug)]
pub(in crate::tokenizer) enum Codepoint {
    Scalar(char),
    EndOfFile,
}

impl CodepointStream {
    pub(in crate::tokenizer) fn peek(&self, offset: usize) -> Codepoint {
        self.source
            .chars()
            .nth(self.cursor + offset)
            .map(Codepoint::from)
            .unwrap_or(Codepoint::EndOfFile)
    }

    pub(in crate::tokenizer) fn advance(&mut self, count: usize) -> bool {
        if self.cursor + count < self.source.len() {
            self.cursor += count;
            true
        } else {
            self.cursor = self.source.len();
            false
        }
    }

    pub(in crate::tokenizer) fn consume_next(&mut self) -> Codepoint {
        let codepoint = self.peek(0);
        self.advance(1);
        codepoint
    }

    pub(in crate::tokenizer) fn next_few_characters_match(
        &self,
        expected: &str,
        case_sensitive: bool,
    ) -> bool {
        expected
            .chars()
            .enumerate()
            .all(|(idx, expected_ch)| self.peek(idx).eq_char(&expected_ch, case_sensitive))
    }

    pub(in crate::tokenizer) fn maybe_consume_next_few_matching_characters(
        &mut self,
        expected: &str,
        case_sensitive: bool,
    ) -> Option<String> {
        let mut buf = String::with_capacity(expected.len());
        for (idx, expected_ch) in expected.chars().enumerate() {
            let codepoint = self.peek(idx);
            if codepoint.eq_char(&expected_ch, case_sensitive) {
                let ch = codepoint.try_into().unwrap();
                buf.push(ch);
            } else {
                return None;
            }
        }
        debug_assert_eq!(buf.len(), expected.len());
        self.advance(buf.len());
        Some(buf)
    }
}

impl From<String> for CodepointStream {
    fn from(source: String) -> Self {
        Self { source, cursor: 0 }
    }
}

impl Codepoint {
    pub(in crate::tokenizer) const NULL: Self = Self::Scalar('\0');

    pub(in crate::tokenizer) fn eq(&self, other: &Self, case_sensitive: bool) -> bool {
        match other {
            Self::Scalar(other_ch) => self.eq_char(other_ch, case_sensitive),
            Self::EndOfFile => match self {
                Codepoint::Scalar(_) => false,
                Codepoint::EndOfFile => true,
            },
        }
    }

    pub(in crate::tokenizer) fn eq_char(&self, other: &char, case_sensitive: bool) -> bool {
        match self {
            Self::Scalar(ch) => {
                if case_sensitive {
                    ch.eq(other)
                } else {
                    ch.eq_ignore_ascii_case(other)
                }
            }
            Self::EndOfFile => false,
        }
    }
}

impl PartialEq<Self> for Codepoint {
    fn eq(&self, other: &Codepoint) -> bool {
        self.eq(other, true)
    }
}

impl From<char> for Codepoint {
    fn from(ch: char) -> Self {
        Self::Scalar(ch)
    }
}

impl Into<u32> for Codepoint {
    fn into(self) -> u32 {
        match self {
            Codepoint::Scalar(ch) => ch.into(),
            Codepoint::EndOfFile => 0xffffffff,
        }
    }
}

impl TryInto<char> for Codepoint {
    type Error = ();

    fn try_into(self) -> Result<char, Self::Error> {
        match self {
            Codepoint::Scalar(ch) => Ok(ch),
            Codepoint::EndOfFile => Err(()),
        }
    }
}
