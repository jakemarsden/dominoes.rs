pub use error::*;
use state::*;
pub use token::*;
use util::*;

mod error;
mod impl_;
mod state;
mod token;
mod util;

pub struct Tokenizer {
    input: CodepointStream,
    state: State,
    current_input_character: Codepoint,
    reconsume_depth: u8,
}

impl From<String> for Tokenizer {
    fn from(input: String) -> Self {
        Self {
            input: CodepointStream::from(input),
            state: State::Data,
            current_input_character: Codepoint::NULL,
            reconsume_depth: 0,
        }
    }
}

impl Tokenizer {
    pub fn exec(&mut self) {
        while self.input.peek(0) != Codepoint::EndOfFile {
            self.handle(self.state);
        }
        self.handle(self.state);
    }

    pub(in crate::tokenizer) fn consume_next_input_character(&mut self) -> Codepoint {
        if self.reconsume_depth == 0 {
            self.current_input_character = self.input.consume_next();
        }
        self.current_input_character
    }

    pub(in crate::tokenizer) fn reconsume_in(&mut self, state: State) {
        // TODO: if we never hit this assert, `self.reconsume_depth` can be a `bool`
        debug_assert_eq!(self.reconsume_depth, 0);
        self.reconsume_depth += 1;
        self.handle(state);
        self.reconsume_depth -= 1;
    }

    pub(in crate::tokenizer) fn emit_token(&self, token: Token) {
        // TODO: emit the token
        println!("Tokenizer::emit_token: {:?}", token);
    }

    pub(in crate::tokenizer) fn emit_parse_error(&self, error: ParseError) {
        // TODO: emit the error
        println!("Tokenizer::emit_parse_error: {:?}", error);
    }

    fn handle(&mut self, state: State) {
        match state {
            State::Data => self.handle_data(),
            State::RCDATA => self.handle_rcdata(),
            State::RAWTEXT => self.handle_rawtext(),
            State::ScriptData => self.handle_script_data(),
            State::PLAINTEXT => self.handle_plaintext(),
            State::TagOpen => self.handle_tag_open(),
            State::EndTagOpen => self.handle_end_tag_open(),
            State::TagName => self.handle_tag_name(),
            State::RCDATALessThanSign => self.handle_rcdata_less_than_sign(),
            State::RCDATAEndTagOpen => self.handle_rcdata_end_tag_open(),
            State::RCDATAEndTagName => self.handle_rcdata_end_tag_name(),
            State::RAWTEXTLessThanSign => self.handle_rawtext_less_than_sign(),
            State::RAWTEXTEndTagOpen => self.handle_rawtext_end_tag_open(),
            State::RAWTEXTEndTagName => self.handle_rawtext_end_tag_name(),
            State::ScriptDataLessThanSign => self.handle_script_data_less_than_sign(),
            State::ScriptDataEndTagOpen => self.handle_script_data_end_tag_open(),
            State::ScriptDataEndTagName => self.handle_script_data_end_tag_name(),
            State::ScriptDataEscapeStart => self.handle_script_data_escape_start(),
            State::ScriptDataEscapeStartDash => self.handle_script_data_escape_start_dash(),
            State::ScriptDataEscaped => self.handle_script_data_escaped(),
            State::ScriptDataEscapedDash => self.handle_script_data_escaped_dash(),
            State::ScriptDataEscapedDashDash => self.handle_script_data_escaped_dash_dash(),
            State::ScriptDataEscapedLessThanSign => {
                self.handle_script_data_escaped_less_than_sign()
            }
            State::ScriptDataEscapedEndTagOpen => self.handle_script_data_escaped_end_tag_open(),
            State::ScriptDataEscapedEndTagName => self.handle_script_data_escaped_end_tag_name(),
            State::ScriptDataDoubleEscapeStart => self.handle_script_data_double_escape_start(),
            State::ScriptDataDoubleEscaped => self.handle_script_data_double_escaped(),
            State::ScriptDataDoubleEscapedDash => self.handle_script_data_double_escaped_dash(),
            State::ScriptDataDoubleEscapedDashDash => {
                self.handle_script_data_double_escaped_dash_dash()
            }
            State::ScriptDataDoubleEscapedLessThanSign => {
                self.handle_script_data_double_escaped_less_than_sign()
            }
            State::ScriptDataDoubleEscapeEnd => self.handle_script_data_double_escape_end(),
            State::BeforeAttributeName => self.handle_before_attribute_name(),
            State::AttributeName => self.handle_attribute_name(),
            State::AfterAttributeName => self.handle_after_attribute_name(),
            State::BeforeAttributeValue => self.handle_before_attribute_value(),
            State::AttributeValueDoubleQuoted => self.handle_attribute_value_double_quoted(),
            State::AttributeValueSingleQuoted => self.handle_attribute_value_single_quoted(),
            State::AttributeValueUnquoted => self.handle_attribute_value_unquoted(),
            State::AfterAttributeValueQuoted => self.handle_after_attribute_value_quoted(),
            State::SelfClosingStartTag => self.handle_self_closing_start_tag(),
            State::BogusComment => self.handle_bogus_comment(),
            State::MarkupDeclarationOpen => self.handle_markup_declaration_open(),
            State::CommentStart => self.handle_comment_start(),
            State::CommentStartDash => self.handle_comment_start_dash(),
            State::Comment => self.handle_comment(),
            State::CommentLessThanSign => self.handle_comment_less_than_sign(),
            State::CommentLessThanSignBang => self.handle_comment_less_than_sign_bang(),
            State::CommentLessThanSignBangDash => self.handle_comment_less_than_sign_bang_dash(),
            State::CommentLessThanSignBangDashDash => {
                self.handle_comment_less_than_sign_bang_dash_dash()
            }
            State::CommentEndDash => self.handle_comment_end_dash(),
            State::CommentEnd => self.handle_comment_end(),
            State::CommentEndBang => self.handle_comment_end_bang(),
            State::DOCTYPE => self.handle_doctype(),
            State::BeforeDOCTYPEName => self.handle_beforedoctype_name(),
            State::DOCTYPEName => self.handle_doctype_name(),
            State::AfterDOCTYPEName => self.handle_afterdoctype_name(),
            State::AfterDOCTYPEPublicKeyword => self.handle_afterdoctype_public_keyword(),
            State::BeforeDOCTYPEPublicIdentifier => self.handle_beforedoctype_public_identifier(),
            State::DOCTYPEPublicIdentifierDoubleQuoted => {
                self.handle_doctype_public_identifier_double_quoted()
            }
            State::DOCTYPEPublicIdentifierSingleQuoted => {
                self.handle_doctype_public_identifier_single_quoted()
            }
            State::AfterDOCTYPEPublicIdentifier => self.handle_afterdoctype_public_identifier(),
            State::BetweenDOCTYPEPublicAndSystemIdentifiers => {
                self.handle_betweendoctype_public_and_system_identifiers()
            }
            State::AfterDOCTYPESystemKeyword => self.handle_afterdoctype_system_keyword(),
            State::BeforeDOCTYPESystemIdentifier => self.handle_beforedoctype_system_identifier(),
            State::DOCTYPESystemIdentifierDoubleQuoted => {
                self.handle_doctype_system_identifier_double_quoted()
            }
            State::DOCTYPESystemIdentifierSingleQuoted => {
                self.handle_doctype_system_identifier_single_quoted()
            }
            State::AfterDOCTYPESystemIdentifier => self.handle_afterdoctype_system_identifier(),
            State::BogusDOCTYPE => self.handle_bogusdoctype(),
            State::CDATASection => self.handle_cdata_section(),
            State::CDATASectionBracket => self.handle_cdata_section_bracket(),
            State::CDATASectionEnd => self.handle_cdata_section_end(),
            State::CharacterReference => self.handle_character_reference(),
            State::NamedCharacterReference => self.handle_named_character_reference(),
            State::AmbiguousAmpersand => self.handle_ambiguous_ampersand(),
            State::NumericCharacterReference => self.handle_numeric_character_reference(),
            State::HexadecimalCharacterReferenceStart => {
                self.handle_hexadecimal_character_reference_start()
            }
            State::DecimalCharacterReferenceStart => {
                self.handle_decimal_character_reference_start()
            }
            State::HexadecimalCharacterReference => self.handle_hexadecimal_character_reference(),
            State::DecimalCharacterReference => self.handle_decimal_character_reference(),
            State::NumericCharacterReferenceEnd => self.handle_numeric_character_reference_end(),
        }
    }
}
