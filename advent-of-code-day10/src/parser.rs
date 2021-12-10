use std::io::{BufRead, BufReader, Read};

use eyre::eyre;
use std::convert::TryFrom;

use aoc_core::{Error, Result};

enum OpenChunkSymbol {
  OpenParenthesis,
  OpenSquareBracket,
  OpenBrace,
  LessSign,
}

impl OpenChunkSymbol {
  fn get_associated_close_symbol(&self) -> CloseChunkSymbol {
    match self {
      Self::OpenParenthesis => CloseChunkSymbol::CloseParenthesis,
      Self::OpenSquareBracket => CloseChunkSymbol::CloseSquareBracket,
      Self::OpenBrace => CloseChunkSymbol::CloseBrace,
      Self::LessSign => CloseChunkSymbol::GreaterSign,
    }
  }
}

impl TryFrom<char> for OpenChunkSymbol {
  type Error = Error;

  fn try_from(c: char) -> Result<Self> {
    match c {
      '(' => Ok(Self::OpenParenthesis),
      '[' => Ok(Self::OpenSquareBracket),
      '{' => Ok(Self::OpenBrace),
      '<' => Ok(Self::LessSign),
      _ => Err(eyre!("Invalid '{}' open chunk symbol")),
    }
  }
}

#[derive(PartialEq, Eq)]
enum CloseChunkSymbol {
  CloseParenthesis,
  CloseSquareBracket,
  CloseBrace,
  GreaterSign,
}

impl CloseChunkSymbol {
  fn get_syntax_error_score(&self) -> usize {
    match self {
      Self::CloseParenthesis => 3,
      Self::CloseSquareBracket => 57,
      Self::CloseBrace => 1197,
      Self::GreaterSign => 25137,
    }
  }

  fn get_completion_score(&self) -> usize {
    match self {
      Self::CloseParenthesis => 1,
      Self::CloseSquareBracket => 2,
      Self::CloseBrace => 3,
      Self::GreaterSign => 4,
    }
  }
}

impl TryFrom<char> for CloseChunkSymbol {
  type Error = Error;

  fn try_from(c: char) -> Result<Self> {
    match c {
      ')' => Ok(Self::CloseParenthesis),
      ']' => Ok(Self::CloseSquareBracket),
      '}' => Ok(Self::CloseBrace),
      '>' => Ok(Self::GreaterSign),
      _ => Err(eyre!("Invalid '{}' close chunk symbol")),
    }
  }
}

enum Token {
  OpenChunk(OpenChunkSymbol),
  CloseChunk(CloseChunkSymbol),
  Unclassified(char),
}

impl From<char> for Token {
  fn from(c: char) -> Self {
    OpenChunkSymbol::try_from(c)
      .map(Self::OpenChunk)
      .or_else(|_| CloseChunkSymbol::try_from(c).map(Self::CloseChunk))
      .unwrap_or_else(|_| Self::Unclassified(c))
  }
}

enum ParseResult<'a> {
  Incomplete(Vec<&'a OpenChunkSymbol>),
  Corrupted(&'a CloseChunkSymbol),
  Valid,
}

struct ParsedLine {
  #[allow(dead_code)]
  line_number: usize,
  tokens: Vec<Token>,
}

pub struct Parser {
  lines: Vec<ParsedLine>,
}

impl Parser {
  pub fn from_reader(reader: impl Read) -> Result<Self> {
    let reader = BufReader::new(reader);

    let lines = reader
      .lines()
      .enumerate()
      .map(|(line_number, line)| {
        line
          .map(|s| {
            let tokens = s.trim().chars().map(Token::from).collect();

            ParsedLine {
              line_number,
              tokens,
            }
          })
          .map_err(Into::into)
      })
      .collect::<Result<Vec<_>>>()?;

    Ok(Self { lines })
  }

  fn interpret_lines(&self) -> Vec<ParseResult> {
    self
      .lines
      .iter()
      .map(|line| {
        let mut open_symbols = vec![];

        for token in line.tokens.iter() {
          match token {
            Token::OpenChunk(symbol) => {
              open_symbols.push(symbol);
            }
            Token::CloseChunk(symbol) => {
              if open_symbols
                .pop()
                .map(|open_symbol| open_symbol.get_associated_close_symbol() != *symbol)
                .unwrap_or(true)
              {
                return ParseResult::Corrupted(symbol);
              }
            }
            _ => continue,
          }
        }

        if !open_symbols.is_empty() {
          ParseResult::Incomplete(open_symbols)
        } else {
          ParseResult::Valid
        }
      })
      .collect()
  }

  pub fn compute_completion_score(&self) -> usize {
    let mut completion_scores = self
      .interpret_lines()
      .into_iter()
      .filter_map(|result| match result {
        ParseResult::Incomplete(mut open_symbols) => {
          let mut score = 0;

          while let Some(symbol) = open_symbols.pop() {
            score = score * 5 + symbol.get_associated_close_symbol().get_completion_score();
          }

          Some(score)
        }
        _ => None,
      })
      .collect::<Vec<_>>();

    completion_scores.sort_unstable();

    completion_scores[completion_scores.len() / 2]
  }

  pub fn compute_syntax_error_score(&self) -> usize {
    self
      .interpret_lines()
      .into_iter()
      .filter_map(|result| match result {
        ParseResult::Corrupted(symbol) => Some(symbol.get_syntax_error_score()),
        _ => None,
      })
      .sum::<usize>()
  }
}
