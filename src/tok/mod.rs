// derived from both the LALRPOP whitespace tokenizer, and LALRPOP's
// internal tokenizer

use std::str::CharIndices;
use std::str::FromStr;
use unicode_xid::UnicodeXID;

use self::ErrorCode::*;
use self::Tok::*;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub location: usize,
    pub code: ErrorCode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    If,
    Then,
    Else,
    Begin,
    End,
    Let,
    In,
    Eq,
    RArrow,
    LArrow,
    Lt,
    Gt,
    Plus,
    Minus,
    Mul,
    Cons,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Fun,
    Fix,
    Head,
    Tail,
    Emptyq,
    Array,
    True,
    False,
    Empty,
    Comma,
    Set,
    Ident(&'input str),
    Num(i64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    UnrecognizedToken,
    ExpectedNumber,
}

fn error<T>(c: ErrorCode, l: usize) -> Result<T, Error> {
    Err(Error {
        location: l,
        code: c,
    })
}

pub type Spanned<T> = (usize, T, usize);

pub struct Tokenizer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
}

const KEYWORDS: &'static [(&'static str, Tok<'static>)] = &[
    ("if", If),
    ("then", Then),
    ("else", Else),
    ("begin", Begin),
    ("end", End),
    ("let", Let),
    ("in", In),
    ("fun", Fun),
    ("fix", Fix),
    ("head", Head),
    ("tail", Tail),
    ("true", True),
    ("false", False),
    ("array", Array),
    ("set", Set),
    ];

impl<'input> Tokenizer<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut t = Tokenizer {
            text: input,
            chars: input.char_indices(),
            lookahead: None,
        };
        t.bump();
        t
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        self.lookahead = self.chars.next();
        self.lookahead
    }

    fn word(&mut self, idx0: usize) -> Spanned<&'input str> {
        match self.take_while(is_identifier_continue) {
            Some(end) => (idx0, &self.text[idx0..end], end),
            None => (idx0, &self.text[idx0..], self.text.len()),
        }
    }

    fn take_while<F>(&mut self, mut keep_going: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        self.take_until(|c| !keep_going(c))
    }

    fn take_until<F>(&mut self, mut terminate: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        loop {
            match self.lookahead {
                None => {
                    return None;
                }
                Some((idx1, c)) => {
                    if terminate(c) {
                        return Some(idx1);
                    } else {
                        self.bump();
                    }
                }
            }
        }
    }

    fn identifierish(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        let (start, word, end) = self.word(idx0);

        if word == "empty" {
            if let Some((_, '?')) = self.lookahead {
                self.bump();
                return Ok((start, Emptyq, end + 1));
            }
            return Ok((start, Empty, end));
        }

        // search for a keyword first; if none are found, this is
        // either a MacroId or an Id, depending on whether there
        // is a `<` immediately afterwards
        let tok = KEYWORDS.iter()
            .filter(|&&(w, _)| w == word)
            .map(|&(_, ref t)| t.clone())
            .next()
            .unwrap_or_else(|| Ident(word));

        Ok((start, tok, end))
    }

    fn number(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        let (start, word, end) = match self.take_while(is_digit) {
            Some(end) => (idx0, &self.text[idx0..end], end),
            None => (idx0, &self.text[idx0..], self.text.len()),
        };

        Ok((start, Num(i64::from_str(word).unwrap()), end))
    }
}

macro_rules! consume {
    ($s: expr, $i:expr, $tok:expr, $len:expr) => { {
        $s.bump();
        Some(Ok(($i, $tok, $i+$len)))
    } }
}


impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Spanned<Tok<'input>>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.lookahead {
                Some((i, '=')) => consume!(self, i, Eq, 1),
                Some((i, '<')) => {
                    match self.bump() {
                        Some((_, '-')) => consume!(self, i, LArrow, 2),
                        _ => {
                            // we've already bumped, don't consume
                            Some(Ok((i, Lt, 1)))
                        }
                    }
                }
                Some((i, '>')) => consume!(self, i, Gt, 1),
                Some((i, '-')) => {
                    match self.bump() {
                        Some((_, '>')) => consume!(self, i, RArrow, 2),
                        Some((_, c)) if is_digit(c) => {
                            if let Ok((_, Num(n), end)) = self.number(i + 1) {
                                Some(Ok((i, Num(-n), end)))
                            } else {
                                Some(error(ExpectedNumber, i + 1))
                            }
                        }
                        _ => {
                            // we've already bumped, don't consume
                            Some(Ok((i, Minus, 1)))
                        }
                    }
                }
                Some((i, '+')) => consume!(self, i, Plus, 1),
                Some((i, '*')) => consume!(self, i, Mul, 1),
                Some((i, ':')) => {
                    match self.bump() {
                        Some((_, ':')) => consume!(self, i, Cons, 2),
                        _ => Some(error(UnrecognizedToken, i)),
                    }
                }
                Some((i, '(')) => consume!(self, i, LParen, 1),
                Some((i, ')')) => consume!(self, i, RParen, 1),
                Some((i, '[')) => consume!(self, i, LBracket, 1),
                Some((i, ']')) => consume!(self, i, RBracket, 1),
                Some((i, ',')) => consume!(self, i, Comma, 1),
                Some((i, c)) if is_digit(c) => Some(self.number(i)),
                Some((i, c)) if is_identifier_start(c) => Some(self.identifierish(i)),
                Some((_, c)) if c.is_whitespace() => {
                    self.bump();
                    continue;
                }
                Some((i, _)) => Some(error(UnrecognizedToken, i)),
                None => None,
            };
        }
    }
}

fn is_digit(c: char) -> bool {
    '9' >= c && c >= '0'
}

fn is_identifier_start(c: char) -> bool {
    UnicodeXID::is_xid_start(c) || c == '_'
}

fn is_identifier_continue(c: char) -> bool {
    UnicodeXID::is_xid_continue(c)
}
