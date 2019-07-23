use self::super::super::super::super::isa::instruction::ParseInstructionError;
use self::super::super::super::super::util::parse_with_prefix;
use self::super::AssemblerDirective;


impl<'s> AssemblerDirective<'s> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub(in super::super) fn from_str_impl(s: &'s str) -> Result<Option<Self>, ParseInstructionError> {
        let mut tokens = s.split_whitespace().peekable();
        let has_colon = tokens.peek() == Some(&":");
        if has_colon {
            tokens.next();
        }

        let operation = parse_directive(&mut tokens, s, has_colon)?;

        if operation.is_some() {
            if let Some(tok) = tokens.next() {
                return Err(ParseInstructionError::TooManyTokens((tok.as_ptr() as usize) - (s.as_ptr() as usize) + 1));
            }
        }

        Ok(operation)
    }
}


fn parse_directive<'i, I: Iterator<Item = &'i str>>(itr: &mut I, orig_str: &'i str, had_colon: bool)
                                                    -> Result<Option<AssemblerDirective<'i>>, ParseInstructionError> {
    static VALID_TOKENS: &[&str] = &["origin", "label"];

    match itr.next() {
        Some(mut tok) => {
            if !had_colon {
                if tok.chars().next().unwrap() != ':' {
                    return Ok(None);
                }

                tok = &tok[1..];
            }

            let start_pos = (tok.as_ptr() as usize) - (orig_str.as_ptr() as usize);

            if tok.eq_ignore_ascii_case("origin") {
                Ok(Some(parse_directive_origin(itr, orig_str, start_pos + 6 + 1)?))
            } else if tok.eq_ignore_ascii_case("label") {
                Ok(Some(parse_directive_label(itr, orig_str, start_pos + 5 + 1)?))
            } else {
                Err(ParseInstructionError::UnrecognisedToken(start_pos + 1, VALID_TOKENS))
            }
        }
        None => Err(ParseInstructionError::EmptyString),
    }
}

fn parse_directive_origin<'i, I: Iterator<Item = &'i str>>(itr: &mut I, orig_str: &'i str, pos: usize)
                                                           -> Result<AssemblerDirective<'i>, ParseInstructionError> {
    static VALID_TOKENS: &[&str] = &["[16-bit origin address]"];

    match itr.next() {
        Some(tok) => {
            let start_pos = (tok.as_ptr() as usize) - (orig_str.as_ptr() as usize);

            if let Some(origin) = parse_with_prefix::<u16>(tok) {
                Ok(AssemblerDirective::SetOrigin(origin))
            } else {
                Err(ParseInstructionError::UnrecognisedToken(start_pos + 1, VALID_TOKENS))
            }
        }
        None => Err(ParseInstructionError::MissingToken(pos, VALID_TOKENS)),
    }
}

fn parse_directive_label<'i, I: Iterator<Item = &'i str>>(itr: &mut I, orig_str: &'i str, pos: usize) -> Result<AssemblerDirective<'i>, ParseInstructionError> {
    static VALID_TOKENS: &[&str] = &["save", "load"];

    match itr.next() {
        Some(tok) => {
            let start_pos = (tok.as_ptr() as usize) - (orig_str.as_ptr() as usize);

            if tok.eq_ignore_ascii_case("save") {
                Ok(AssemblerDirective::SaveLabel(parse_directive_label_name(itr, start_pos + 4 + 1)?))
            } else if tok.eq_ignore_ascii_case("load") {
                Ok(AssemblerDirective::LoadLabel(parse_directive_label_name(itr, start_pos + 4 + 1)?))
            } else {
                Err(ParseInstructionError::UnrecognisedToken(start_pos + 1, VALID_TOKENS))
            }
        }
        None => Err(ParseInstructionError::MissingToken(pos, VALID_TOKENS)),
    }
}

fn parse_directive_label_name<'i, I: Iterator<Item = &'i str>>(itr: &mut I, pos: usize) -> Result<&'i str, ParseInstructionError> {
    static VALID_TOKENS: &[&str] = &["[label name]"];

    match itr.next() {
        Some(tok) => Ok(tok),
        None => Err(ParseInstructionError::MissingToken(pos, VALID_TOKENS)),
    }
}