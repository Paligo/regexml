use ahash::HashSet;

use crate::re_program::ReFlags;

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

    captures: HashSet<usize>,
    hash_backreferences: bool,

    re_flags: ReFlags,

    warning: Vec<String>,
}

enum Error {
    Internal,
    ExpectedDigit,
    ExpectedValidNumber,
    ExpectedCommaOrRightBracket,
    ExpectedComma,
    BadRange,
    MissingCloseBrace,
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
            return Err(Error::ExpectedDigit);
        }

        // get min ('m' of {m,n}) number
        let mut number = String::new();
        while self.idx < self.len && self.pattern[self.idx].is_ascii_digit() {
            number.push(self.pattern[self.idx]);
            self.idx += 1;
        }

        self.bracket_min = number
            .parse::<usize>()
            .map_err(|_| Error::ExpectedValidNumber)?;

        // if out of input, fail
        if self.idx >= self.len {
            return Err(Error::ExpectedCommaOrRightBracket);
        }

        // if end of expr, optional limit is 0
        if self.pattern[self.idx] == '}' {
            self.idx += 1;
            self.bracket_max = self.bracket_min;
            return Ok(());
        }

        // must have at least {m,} and maybe {m,n}
        if self.idx >= self.len || self.pattern[self.idx] != ',' {
            return Err(Error::ExpectedComma);
        }
        self.idx += 1;

        // if out of input, fail
        if self.idx >= self.len {
            return Err(Error::ExpectedCommaOrRightBracket);
        }

        // if {m,} max is unlimited
        if self.pattern[self.idx] == '}' {
            self.idx += 1;
            self.bracket_max = usize::MAX;
            return Ok(());
        }

        // next char must be a digit
        if self.idx >= self.len || !self.pattern[self.idx].is_ascii_digit() {
            return Err(Error::ExpectedDigit);
        }

        // get max number
        let mut number = String::new();
        while self.idx < self.len && self.pattern[self.idx].is_ascii_digit() {
            number.push(self.pattern[self.idx]);
            self.idx += 1;
        }

        self.bracket_max = number
            .parse::<usize>()
            .map_err(|_| Error::ExpectedValidNumber)?;

        // optional repetitions must be >= 0
        if self.bracket_max < self.bracket_min {
            return Err(Error::BadRange);
        }

        // must have close brace
        if self.idx >= self.len || self.pattern[self.idx] != '}' {
            return Err(Error::MissingCloseBrace);
        }
        Ok(())
    }
}
