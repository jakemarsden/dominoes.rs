use crate::tokenizer::Token::{self, *};
use crate::tokenizer::Tokenizer;

use super::state::InsertionMode::*;
use super::Parser;

impl<T: Tokenizer> Parser<T> {
    pub(in crate::parser) fn handle_initial(&mut self, token: Token) {
        match token {
            Character('\t')
            | Character('\n')
            | Character('\u{000C}')
            | Character('\r')
            | Character(' ') => {
                // ignore the token
            }
            Comment { data: _ } => {
                unimplemented!();
            }
            Doctype {
                name,
                public_identifier,
                system_identifier,
                force_quirks: _,
            } => {
                if name != Some("html".into())
                    || public_identifier.is_some()
                    || system_identifier
                        .filter(|id| id != "about:legacy-compat")
                        .is_some()
                {
                    self.emit_anonymous_parse_error();
                }
                unimplemented!();
            }
            _ => {
                // TODO: If the document is not an iframe srcdoc document, then this is a parse
                //       error; set the Document to quirks mode.
                self.reprocess_in(BeforeHtml);
            }
        }
    }

    pub(in crate::parser) fn handle_before_html(&mut self, _: Token) {
        unimplemented!();
    }

    pub(in crate::parser) fn handle_before_head(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_head(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_head_noscript(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_after_head(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_body(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_text(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_table(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_table_text(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_caption(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_column_group(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_table_body(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_row(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_cell(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_select(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_select_in_table(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_template(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_after_body(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_in_frameset(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_after_frameset(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_after_after_body(&mut self, _: Token) {
        unimplemented!()
    }

    pub(in crate::parser) fn handle_after_after_frameset(&mut self, _: Token) {
        unimplemented!()
    }
}
