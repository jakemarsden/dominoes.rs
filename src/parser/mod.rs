use std::cell::RefCell;
use std::rc::Rc;

use state::InsertionMode;

use crate::dom::Node;
use crate::tokenizer::error::ParseError;
use crate::tokenizer::{Token, Tokenizer};

mod impl_;
mod state;

#[cfg(test)]
mod tests;

pub struct Parser<T: Tokenizer> {
    tokenizer: T,
    root_node: Rc<RefCell<Node>>,
    mode: InsertionMode,
    current_token: Token,
    reprocess_current_token: bool,
}

impl<T: Tokenizer> Parser<T> {
    pub fn new(tokenizer: T) -> Self {
        Self {
            tokenizer,
            root_node: Node::create_document(),
            mode: InsertionMode::Initial,
            current_token: Token::EndOfFile,
            reprocess_current_token: false,
        }
    }

    pub fn parse(&mut self) -> Rc<RefCell<Node>> {
        while self.do_some_work() {}
        self.root_node.clone()
    }

    pub(crate) fn switch_to(&mut self, next_mode: InsertionMode) {
        if self.mode == next_mode {
            return;
        }
        println!("Parser::switch_to: {:?} -> {:?}", self.mode, next_mode);
        self.mode = next_mode;
    }

    pub(crate) fn reprocess_in(&mut self, next_mode: InsertionMode) {
        println!("Parser::reprocess_in: {:?} -> {:?}", self.mode, next_mode);
        debug_assert!(!self.reprocess_current_token);
        self.reprocess_current_token = true;
        self.mode = next_mode;
    }

    /// Returns `true` if there is more work to do
    fn do_some_work(&mut self) -> bool {
        if let Some(token) = self.next_non_error_token() {
            self.handle(token);
            true
        } else {
            false
        }
    }

    fn next_non_error_token(&mut self) -> Option<Token> {
        if !self.reprocess_current_token {
            loop {
                match self.tokenizer.next() {
                    Some(Ok(token)) => {
                        self.current_token = token;
                        break;
                    }
                    Some(Err(error)) => {
                        self.emit_parse_error(error);
                        continue;
                    }
                    None => {
                        return None;
                    }
                }
            }
        }
        Some(self.current_token.clone())
    }

    fn handle(&mut self, token: Token) {
        match self.mode {
            InsertionMode::Initial => self.handle_initial(token),
            InsertionMode::BeforeHtml => self.handle_before_html(token),
            InsertionMode::BeforeHead => self.handle_before_head(token),
            InsertionMode::InHead => self.handle_in_head(token),
            InsertionMode::InHeadNoscript => self.handle_in_head_noscript(token),
            InsertionMode::AfterHead => self.handle_after_head(token),
            InsertionMode::InBody => self.handle_in_body(token),
            InsertionMode::Text => self.handle_text(token),
            InsertionMode::InTable => self.handle_in_table(token),
            InsertionMode::InTableText => self.handle_in_table_text(token),
            InsertionMode::InCaption => self.handle_in_caption(token),
            InsertionMode::InColumnGroup => self.handle_in_column_group(token),
            InsertionMode::InTableBody => self.handle_in_table_body(token),
            InsertionMode::InRow => self.handle_in_row(token),
            InsertionMode::InCell => self.handle_in_cell(token),
            InsertionMode::InSelect => self.handle_in_select(token),
            InsertionMode::InSelectInTable => self.handle_in_select_in_table(token),
            InsertionMode::InTemplate => self.handle_in_template(token),
            InsertionMode::AfterBody => self.handle_after_body(token),
            InsertionMode::InFrameset => self.handle_in_frameset(token),
            InsertionMode::AfterFrameset => self.handle_after_frameset(token),
            InsertionMode::AfterAfterBody => self.handle_after_after_body(token),
            InsertionMode::AfterAfterFrameset => self.handle_after_after_frameset(token),
        }
    }

    fn emit_parse_error(&self, error: ParseError) {
        // TODO: emit the parse error
        println!("Parser::emit_parse_error: {:?}", error);
    }

    fn emit_anonymous_parse_error(&self) {
        // TODO: work out what kind of parse error it should be
        println!("Parser::emit_anonymous_parse_error");
    }
}
