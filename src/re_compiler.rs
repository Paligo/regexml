use ahash::{HashSet, HashSetExt};

use crate::{charclass::CharacterClass, re_program::ReFlags};

// No flags (nothing special)
pub(crate) const NODE_NORMAL: u32 = 0;
// True if top level expr
pub(crate) const NODE_TOPLEVEL: u32 = 2;

struct ReCompiler<'a> {
    // Input state for compiling regular expression

    // input string
    pattern: &'a [char],
    // length of the pattern string
    len: usize,
    // current input index into ac
    idx: usize,
    // total number of paren pairs
    capturing_open_paren_count: usize,

    // {m, n} stacks
    // minimum number of matches
    bracket_min: usize,
    // maximum number of matches
    bracket_max: usize,

    is_xpath: bool,
    is_xpath_30: bool,
    is_xsd_11: bool,

    captures: HashSet<char>,
    has_back_references: bool,

    re_flags: ReFlags,

    warning: Vec<String>,
}

enum Error {
    Internal,
    Syntax(String),
}

impl Error {
    fn syntax(s: impl Into<String>) -> Error {
        Error::Syntax(s.into())
    }
}

impl<'a> ReCompiler<'a> {
    fn bracket(&mut self) -> Result<(), Error> {
        if self.idx >= self.len {
            return Err(Error::Internal);
        }
        if self.pattern[self.idx] != '{' {
            return Err(Error::Internal);
        }
        self.idx += 1;

        // next char must be a digit
        if self.idx >= self.len || !self.pattern[self.idx].is_ascii_digit() {
            return Err(Error::syntax("Expected digit"));
        }

        // get min ('m' of {m,n}) number
        let mut number = String::new();
        while self.idx < self.len && self.pattern[self.idx].is_ascii_digit() {
            number.push(self.pattern[self.idx]);
            self.idx += 1;
        }

        self.bracket_min = number
            .parse::<usize>()
            .map_err(|_| Error::syntax("Expected valid number"))?;

        // if out of input, fail
        if self.idx >= self.len {
            return Err(Error::syntax("Expected comma or right bracket"));
        }

        // if end of expr, optional limit is 0
        if self.pattern[self.idx] == '}' {
            self.idx += 1;
            self.bracket_max = self.bracket_min;
            return Ok(());
        }

        // must have at least {m,} and maybe {m,n}
        if self.idx >= self.len || self.pattern[self.idx] != ',' {
            return Err(Error::syntax("Expected comma"));
        }
        self.idx += 1;

        // if out of input, fail
        if self.idx >= self.len {
            return Err(Error::syntax("Expected comma or right bracket"));
        }

        // if {m,} max is unlimited
        if self.pattern[self.idx] == '}' {
            self.idx += 1;
            self.bracket_max = usize::MAX;
            return Ok(());
        }

        // next char must be a digit
        if self.idx >= self.len || !self.pattern[self.idx].is_ascii_digit() {
            return Err(Error::syntax("Unexpected digit"));
        }

        // get max number
        let mut number = String::new();
        while self.idx < self.len && self.pattern[self.idx].is_ascii_digit() {
            number.push(self.pattern[self.idx]);
            self.idx += 1;
        }

        self.bracket_max = number
            .parse::<usize>()
            .map_err(|_| Error::syntax("Expected valid number"))?;

        // optional repetitions must be >= 0
        if self.bracket_max < self.bracket_min {
            return Err(Error::syntax("Bad range"));
        }

        // must have close brace
        if self.idx >= self.len || self.pattern[self.idx] != '}' {
            return Err(Error::syntax("Missing closing brace"));
        }
        Ok(())
    }

    fn escape(&mut self, in_square_brackets: bool) -> Result<CharacterClass, Error> {
        // "Shouldn't" happen
        if self.pattern[self.idx] != '\\' {
            return Err(Error::Internal);
        }

        // escape shouldn't occur as last character in string!
        if self.idx + 1 >= self.len {
            return Err(Error::syntax("Escape terminates string"));
        }

        // switch on character after backslash
        self.idx += 2;
        let escape_char = self.pattern[self.idx - 1];
        match escape_char {
            'n' => Ok(CharacterClass::Char('\n')),
            'r' => Ok(CharacterClass::Char('\r')),
            't' => Ok(CharacterClass::Char('\t')),
            '\\' | '|' | '.' | '-' | '^' | '?' | '*' | '+' | '{' | '}' | '(' | ')' | '[' | ']' => {
                Ok(CharacterClass::Char(escape_char))
            }
            '$' => {
                if self.is_xpath {
                    Ok(CharacterClass::Char('$'))
                } else {
                    Err(Error::syntax("In XSD, '$' must not be escaped"))
                }
            }
            's' => Ok(CharacterClass::escape_s_lower()),
            'S' => Ok(CharacterClass::escape_s_upper()),
            // TODO: i, I, c, C, d, D, w, W
            'p' | 'P' => {
                if self.idx == self.len {
                    return Err(Error::syntax(format!(
                        "Expected '{{' after \\{}",
                        escape_char
                    )));
                }
                if self.pattern[self.idx] != '{' {
                    return Err(Error::syntax(format!(
                        "Expected '{{' after \\{}",
                        escape_char
                    )));
                }
                let from = self.idx + 1;
                let close = self
                    .pattern
                    .iter()
                    .skip(from)
                    .position(|c| *c == '}')
                    .ok_or(Error::syntax(format!(
                        "No closing '}}' after \\{}",
                        escape_char
                    )))?;
                let block = &self.pattern[self.idx..close];
                todo!()
            }
            '0' => Err(Error::syntax("Octal escapes are not allowed")),
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if in_square_brackets {
                    return Err(Error::syntax(
                        "Backreferences not allowed within character classes",
                    ));
                }
                if !self.is_xpath {
                    return Err(Error::syntax("digit not allowed after \\"));
                }
                let mut back_ref = (escape_char as u32) - ('0' as u32);
                while self.idx < self.len {
                    let c1 = self.pattern[self.idx].to_digit(10);
                    if let Some(c1) = c1 {
                        let back_ref2 = back_ref * 10 + c1;
                        if back_ref2 > (self.capturing_open_paren_count as u32 - 1) {
                            break;
                        }
                        back_ref = back_ref2;
                        self.idx += 1;
                    } else {
                        break;
                    }
                }
                if !self.captures.contains(&char::from_u32(back_ref).unwrap()) {
                    let explanation = if back_ref > ((self.capturing_open_paren_count - 1) as u32) {
                        "(no such group)"
                    } else {
                        "(group not yet closed)"
                    };
                    return Err(Error::syntax(format!(
                        "invalid backreference \\{} {}",
                        back_ref, explanation
                    )));
                }
                self.has_back_references = true;
                // for convenience a back-reference is treated as a character
                // class, though this is a fiction
                // TODO: need to probably introduce CharacterClass::BackReference
                // so we can detect these. Alternatively return an enum that's
                // either a back reference or a character class here.
                Ok(CharacterClass::Char(char::from_u32(back_ref).unwrap()))
            }
            escape_char => Err(Error::syntax(format!(
                "Escape character '{}' not allowed",
                escape_char
            ))),
        }
    }
}
