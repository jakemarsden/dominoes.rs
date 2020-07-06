use std::char::REPLACEMENT_CHARACTER;

use crate::tokenizer::state::State::*;
use crate::tokenizer::util::Codepoint::*;
use crate::tokenizer::ParseError::*;
use crate::tokenizer::*;

impl Tokenizer {
    pub(in crate::tokenizer) fn handle_data(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('&') => {
                debug_assert_eq!(self.return_state, None);
                self.return_state = Some(Data);
                self.switch_to(CharacterReference);
            }
            Scalar('<') => {
                self.switch_to(TagOpen);
            }
            Scalar('\0') => {
                self.emit_parse_error(UnexpectedNullCharacter);
                self.emit_current_input_character();
            }
            EndOfFile => {
                self.emit_eof();
            }
            Scalar(_) => {
                self.emit_current_input_character();
            }
        }
    }

    pub(in crate::tokenizer) fn handle_rcdata(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_rawtext(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_plaintext(&mut self) {
        unimplemented!();
    }
}

impl Tokenizer {
    pub(in crate::tokenizer) fn handle_tag_open(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('!') => {
                self.switch_to(MarkupDeclarationOpen);
            }
            Scalar('/') => {
                self.switch_to(EndTagOpen);
            }
            Scalar(ch) if ch.is_ascii_alphabetic() => {
                self.create_new_start_tag_token();
                self.reconsume_in(TagName);
            }
            Scalar('?') => {
                self.emit_parse_error(UnexpectedQuestionMarkInsteadOfTagName);
                self.create_new_comment_token();
                self.reconsume_in(BogusComment);
            }
            EndOfFile => {
                self.emit_parse_error(EofBeforeTagName);
                self.emit_character('<');
                self.emit_eof();
            }
            Scalar(_) => {
                self.emit_parse_error(InvalidFirstCharacterOfTagName);
                self.emit_character('<');
                self.reconsume_in(Data);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_end_tag_open(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar(ch) if ch.is_ascii_alphabetic() => {
                self.create_new_end_tag_token();
                self.reconsume_in(TagName);
            }
            Scalar('>') => {
                self.emit_parse_error(MissingEndTagName);
                self.switch_to(Data);
            }
            EndOfFile => {
                self.emit_parse_error(EofBeforeTagName);
                self.emit_character('<');
                self.emit_character('/');
                self.emit_eof();
            }
            Scalar(_) => {
                self.emit_parse_error(InvalidFirstCharacterOfTagName);
                self.create_new_comment_token();
                self.reconsume_in(BogusComment);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_tag_name(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('\t') | Scalar('\n') | Scalar('\u{000C}') | Scalar(' ') => {
                self.switch_to(BeforeAttributeName);
            }
            Scalar('/') => {
                self.switch_to(SelfClosingStartTag);
            }
            Scalar('>') => {
                self.switch_to(Data);
                self.emit_current_tag_token();
            }
            Scalar(ch) if ch.is_ascii_uppercase() => {
                self.current_tag_token()
                    .tag_name
                    .push(ch.to_ascii_lowercase());
            }
            Scalar('\0') => {
                self.emit_parse_error(UnexpectedNullCharacter);
                self.current_tag_token()
                    .tag_name
                    .push(REPLACEMENT_CHARACTER);
            }
            EndOfFile => {
                self.emit_parse_error(EofInTag);
                self.emit_eof();
            }
            Scalar(ch) => {
                self.current_tag_token().tag_name.push(ch);
            }
        }
    }
}

impl Tokenizer {
    pub(in crate::tokenizer) fn handle_rcdata_less_than_sign(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_rcdata_end_tag_open(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_rcdata_end_tag_name(&mut self) {
        unimplemented!();
    }
}

impl Tokenizer {
    pub(in crate::tokenizer) fn handle_rawtext_less_than_sign(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_rawtext_end_tag_open(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_rawtext_end_tag_name(&mut self) {
        unimplemented!();
    }
}

impl Tokenizer {
    pub(in crate::tokenizer) fn handle_script_data_less_than_sign(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_end_tag_open(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_end_tag_name(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_escape_start(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_escape_start_dash(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_escaped(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_escaped_dash(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_escaped_dash_dash(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_escaped_less_than_sign(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_escaped_end_tag_open(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_escaped_end_tag_name(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_double_escape_start(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_double_escaped(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_double_escaped_dash(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_double_escaped_dash_dash(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_double_escaped_less_than_sign(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_script_data_double_escape_end(&mut self) {
        unimplemented!();
    }
}

impl Tokenizer {
    pub(in crate::tokenizer) fn handle_before_attribute_name(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_attribute_name(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_after_attribute_name(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_before_attribute_value(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_attribute_value_double_quoted(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_attribute_value_single_quoted(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_attribute_value_unquoted(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_after_attribute_value_quoted(&mut self) {
        unimplemented!();
    }
}

impl Tokenizer {
    pub(in crate::tokenizer) fn handle_self_closing_start_tag(&mut self) {
        unimplemented!();
    }
}

impl Tokenizer {
    pub(in crate::tokenizer) fn handle_bogus_comment(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_markup_declaration_open(&mut self) {
        if self
            .maybe_consume_next_few_matching_characters("--", true)
            .is_some()
        {
            self.create_new_comment_token();
            self.switch_to(CommentStart);
        } else if self
            .maybe_consume_next_few_matching_characters("DOCTYPE", false)
            .is_some()
        {
            self.switch_to(DOCTYPE);
        } else if self
            .maybe_consume_next_few_matching_characters("[CDATA[", true)
            .is_some()
        {
            unimplemented!();
        } else {
            self.emit_parse_error(IncorrectlyOpenedComment);
            self.create_new_comment_token();
            self.switch_to(BogusComment);
        }
    }

    pub(in crate::tokenizer) fn handle_comment_start(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_comment_start_dash(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_comment(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_comment_less_than_sign(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_comment_less_than_sign_bang(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_comment_less_than_sign_bang_dash(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_comment_less_than_sign_bang_dash_dash(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_comment_end_dash(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_comment_end(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_comment_end_bang(&mut self) {
        unimplemented!();
    }
}

impl Tokenizer {
    pub(in crate::tokenizer) fn handle_doctype(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('\t') | Scalar('\n') | Scalar('\u{000C}') | Scalar(' ') => {
                self.switch_to(BeforeDOCTYPEName);
            }
            Scalar('>') => {
                self.reconsume_in(BeforeDOCTYPEName);
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.create_new_doctype_token();
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
            }
            Scalar(_) => {
                self.emit_parse_error(MissingWhitespaceBeforeDoctypeName);
                self.reconsume_in(BeforeDOCTYPEName);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_beforedoctype_name(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('\t') | Scalar('\n') | Scalar('\u{000C}') | Scalar(' ') => {
                // ignore the character
            }
            Scalar(ch) if ch.is_ascii_uppercase() => {
                self.create_new_doctype_token();
                self.current_doctype_token().name = Some(ch.to_ascii_lowercase().to_string());
                self.switch_to(DOCTYPEName);
            }
            Scalar('\0') => {
                self.emit_parse_error(UnexpectedNullCharacter);
                self.create_new_doctype_token();
                self.current_doctype_token().name = Some(REPLACEMENT_CHARACTER.to_string());
                self.switch_to(DOCTYPEName);
            }
            Scalar('>') => {
                self.emit_parse_error(MissingDoctypeName);
                self.create_new_doctype_token();
                self.current_doctype_token().force_quirks = true;
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.create_new_doctype_token();
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(ch) => {
                self.create_new_doctype_token();
                self.current_doctype_token().name = Some(ch.to_string());
                self.switch_to(DOCTYPEName);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_doctype_name(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('\t') | Scalar('\n') | Scalar('\u{000C}') | Scalar(' ') => {
                self.switch_to(AfterDOCTYPEName);
            }
            Scalar('>') => {
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            Scalar(ch) if ch.is_ascii_uppercase() => {
                self.current_doctype_token()
                    .name
                    .as_mut()
                    .unwrap()
                    .push(ch.to_ascii_lowercase());
            }
            Scalar('\0') => {
                self.emit_parse_error(UnexpectedNullCharacter);
                self.current_doctype_token()
                    .name
                    .as_mut()
                    .unwrap()
                    .push(REPLACEMENT_CHARACTER);
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(ch) => {
                self.current_doctype_token().name.as_mut().unwrap().push(ch);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_afterdoctype_name(&mut self) {
        let codepoint = self.peek_next_input_character();
        match codepoint {
            Scalar('\t') | Scalar('\n') | Scalar('\u{000C}') | Scalar(' ') => {
                self.next_input_character();
                // ignore the character
            }
            Scalar('>') => {
                self.next_input_character();
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            EndOfFile => {
                self.next_input_character();
                self.emit_parse_error(EofInDoctype);
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(_) => {
                if self
                    .maybe_consume_next_few_matching_characters("PUBLIC", false)
                    .is_some()
                {
                    self.switch_to(AfterDOCTYPEPublicKeyword);
                } else if self
                    .maybe_consume_next_few_matching_characters("SYSTEM", false)
                    .is_some()
                {
                    self.switch_to(AfterDOCTYPESystemKeyword);
                } else {
                    self.emit_parse_error(InvalidCharacterSequenceAfterDoctypeName);
                    self.current_doctype_token().force_quirks = true;
                    self.reconsume_in(BogusDOCTYPE);
                }
            }
        }
    }

    pub(in crate::tokenizer) fn handle_afterdoctype_public_keyword(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('\t') | Scalar('\n') | Scalar('\u{000C}') | Scalar(' ') => {
                self.switch_to(BeforeDOCTYPEPublicIdentifier);
            }
            Scalar('"') => {
                self.emit_parse_error(MissingWhitespaceAfterDoctypePublicKeyword);
                self.current_doctype_token().public_identifier = Some(String::new());
                self.switch_to(DOCTYPEPublicIdentifierDoubleQuoted);
            }
            Scalar('\'') => {
                self.emit_parse_error(MissingWhitespaceAfterDoctypePublicKeyword);
                self.current_doctype_token().public_identifier = Some(String::new());
                self.switch_to(DOCTYPEPublicIdentifierSingleQuoted);
            }
            Scalar('>') => {
                self.emit_parse_error(MissingDoctypePublicIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(_) => {
                self.emit_parse_error(MissingQuoteBeforeDoctypePublicIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.reconsume_in(BogusDOCTYPE);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_beforedoctype_public_identifier(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('\t') | Scalar('\n') | Scalar('\u{000C}') | Scalar(' ') => {
                // ignore the character
            }
            Scalar('"') => {
                self.current_doctype_token().public_identifier = Some(String::new());
                self.switch_to(DOCTYPEPublicIdentifierDoubleQuoted);
            }
            Scalar('\'') => {
                self.current_doctype_token().public_identifier = Some(String::new());
                self.switch_to(DOCTYPEPublicIdentifierSingleQuoted);
            }
            Scalar('>') => {
                self.emit_parse_error(MissingDoctypePublicIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(_) => {
                self.emit_parse_error(MissingQuoteBeforeDoctypePublicIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.reconsume_in(BogusDOCTYPE);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_doctype_public_identifier_double_quoted(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('"') => {
                self.switch_to(AfterDOCTYPEPublicIdentifier);
            }
            Scalar('\0') => {
                self.emit_parse_error(UnexpectedNullCharacter);
                self.current_doctype_token()
                    .public_identifier
                    .as_mut()
                    .unwrap()
                    .push(REPLACEMENT_CHARACTER);
            }
            Scalar('>') => {
                self.emit_parse_error(AbruptDoctypePublicIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(ch) => {
                self.current_doctype_token()
                    .public_identifier
                    .as_mut()
                    .unwrap()
                    .push(ch);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_doctype_public_identifier_single_quoted(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('\'') => {
                self.switch_to(AfterDOCTYPEPublicIdentifier);
            }
            Scalar('\0') => {
                self.emit_parse_error(UnexpectedNullCharacter);
                self.current_doctype_token()
                    .public_identifier
                    .as_mut()
                    .unwrap()
                    .push(REPLACEMENT_CHARACTER);
            }
            Scalar('>') => {
                self.emit_parse_error(AbruptDoctypePublicIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(ch) => {
                self.current_doctype_token()
                    .public_identifier
                    .as_mut()
                    .unwrap()
                    .push(ch);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_afterdoctype_public_identifier(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('\t') | Scalar('\n') | Scalar('\u{000C}') | Scalar(' ') => {
                self.switch_to(BetweenDOCTYPEPublicAndSystemIdentifiers);
            }
            Scalar('>') => {
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            Scalar('"') => {
                self.emit_parse_error(MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers);
                self.current_doctype_token().system_identifier = Some(String::new());
                self.switch_to(DOCTYPESystemIdentifierDoubleQuoted);
            }
            Scalar('\'') => {
                self.emit_parse_error(MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers);
                self.current_doctype_token().system_identifier = Some(String::new());
                self.switch_to(DOCTYPESystemIdentifierSingleQuoted);
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(_) => {
                self.emit_parse_error(MissingQuoteBeforeDoctypeSystemIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.reconsume_in(BogusDOCTYPE);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_betweendoctype_public_and_system_identifiers(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('\t') | Scalar('\n') | Scalar('\u{000C}') | Scalar(' ') => {
                // ignore the character
            }
            Scalar('>') => {
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            Scalar('"') => {
                self.current_doctype_token().system_identifier = Some(String::new());
                self.switch_to(DOCTYPESystemIdentifierDoubleQuoted);
            }
            Scalar('\'') => {
                self.current_doctype_token().system_identifier = Some(String::new());
                self.switch_to(DOCTYPESystemIdentifierSingleQuoted);
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(_) => {
                self.emit_parse_error(MissingQuoteBeforeDoctypeSystemIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.reconsume_in(BogusDOCTYPE);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_afterdoctype_system_keyword(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('\t') | Scalar('\n') | Scalar('\u{000C}') | Scalar(' ') => {
                self.switch_to(BeforeDOCTYPESystemIdentifier);
            }
            Scalar('"') => {
                self.emit_parse_error(MissingWhitespaceAfterDoctypeSystemKeyword);
                self.current_doctype_token().system_identifier = Some(String::new());
                self.switch_to(DOCTYPESystemIdentifierDoubleQuoted);
            }
            Scalar('\'') => {
                self.emit_parse_error(MissingWhitespaceAfterDoctypeSystemKeyword);
                self.current_doctype_token().system_identifier = Some(String::new());
                self.switch_to(DOCTYPESystemIdentifierSingleQuoted);
            }
            Scalar('>') => {
                self.emit_parse_error(MissingDoctypeSystemIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(_) => {
                self.emit_parse_error(MissingQuoteBeforeDoctypeSystemIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.reconsume_in(BogusDOCTYPE);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_beforedoctype_system_identifier(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('\t') | Scalar('\n') | Scalar('\u{000C}') | Scalar(' ') => {
                // ignore the character
            }
            Scalar('"') => {
                self.current_doctype_token().system_identifier = Some(String::new());
                self.switch_to(DOCTYPESystemIdentifierDoubleQuoted);
            }
            Scalar('\'') => {
                self.current_doctype_token().system_identifier = Some(String::new());
                self.switch_to(DOCTYPESystemIdentifierSingleQuoted);
            }
            Scalar('>') => {
                self.emit_parse_error(MissingDoctypeSystemIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(_) => {
                self.emit_parse_error(MissingQuoteBeforeDoctypeSystemIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.reconsume_in(BogusDOCTYPE);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_doctype_system_identifier_double_quoted(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('"') => {
                self.switch_to(AfterDOCTYPESystemIdentifier);
            }
            Scalar('\0') => {
                self.emit_parse_error(UnexpectedNullCharacter);
                self.current_doctype_token()
                    .system_identifier
                    .as_mut()
                    .unwrap()
                    .push(REPLACEMENT_CHARACTER);
            }
            Scalar('>') => {
                self.emit_parse_error(AbruptDoctypeSystemIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(ch) => {
                self.current_doctype_token()
                    .system_identifier
                    .as_mut()
                    .unwrap()
                    .push(ch);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_doctype_system_identifier_single_quoted(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('\'') => {
                self.switch_to(AfterDOCTYPESystemIdentifier);
            }
            Scalar('\0') => {
                self.emit_parse_error(UnexpectedNullCharacter);
                self.current_doctype_token()
                    .system_identifier
                    .as_mut()
                    .unwrap()
                    .push(REPLACEMENT_CHARACTER);
            }
            Scalar('>') => {
                self.emit_parse_error(AbruptDoctypeSystemIdentifier);
                self.current_doctype_token().force_quirks = true;
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(ch) => {
                self.current_doctype_token()
                    .system_identifier
                    .as_mut()
                    .unwrap()
                    .push(ch);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_afterdoctype_system_identifier(&mut self) {
        let codepoint = self.next_input_character();
        match codepoint {
            Scalar('\t') | Scalar('\n') | Scalar('\u{000C}') | Scalar(' ') => {
                // ignore the character
            }
            Scalar('>') => {
                self.switch_to(Data);
                self.emit_current_doctype_token();
            }
            EndOfFile => {
                self.emit_parse_error(EofInDoctype);
                self.current_doctype_token().force_quirks = true;
                self.emit_current_doctype_token();
                self.emit_eof();
            }
            Scalar(_) => {
                self.emit_parse_error(UnexpectedCharacterAfterDoctypeSystemIdentifier);
                self.reconsume_in(BogusDOCTYPE);
            }
        }
    }

    pub(in crate::tokenizer) fn handle_bogusdoctype(&mut self) {
        unimplemented!();
    }
}

impl Tokenizer {
    pub(in crate::tokenizer) fn handle_cdata_section(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_cdata_section_bracket(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_cdata_section_end(&mut self) {
        unimplemented!();
    }
}

impl Tokenizer {
    pub(in crate::tokenizer) fn handle_character_reference(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_named_character_reference(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_ambiguous_ampersand(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_numeric_character_reference(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_hexadecimal_character_reference_start(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_decimal_character_reference_start(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_hexadecimal_character_reference(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_decimal_character_reference(&mut self) {
        unimplemented!();
    }

    pub(in crate::tokenizer) fn handle_numeric_character_reference_end(&mut self) {
        unimplemented!();
    }
}
