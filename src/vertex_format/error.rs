use std;
use lexer;

use lexer::Line;
use lexer::stream_lexer::Lexeme;

#[derive(Debug)]
pub enum Error<'a>{
    LexerError(lexer::Error),
    UnexpectedLexeme(Line, &'static str, Lexeme<'a>),
    UnexpectedLexemeAfter(Line, &'static str, &'a str, Lexeme<'a>),
    SourceIsEmpty(&'a str),
    VertexFormatIsEmpty,
    Other(String),
}

impl<'a> std::fmt::Display for Error<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Error::LexerError(ref error) => write!(f, "{}", error),
            Error::UnexpectedLexeme(ref line, expected, ref unexpected) => write!(f,"Expected {}, but {} has been found\n{}", expected, unexpected, line),
            Error::UnexpectedLexemeAfter(ref line, expected, ref after, ref unexpected) => write!(f,"Expected {} after {}, but {} has been found\n{}", expected, after, unexpected, line),
            Error::SourceIsEmpty(source_name) => write!(f,"Source \"{}\" of Vertex Format is empty", source_name),
            Error::VertexFormatIsEmpty => write!(f,"Vertex Format is empty"),
            Error::Other(ref message) => write!(f, "{}", message),
        }
    }
}

impl<'a> From<lexer::Error> for Error<'a> {
    fn from(error:lexer::Error) -> Self {
        Error::LexerError(error)
    }
}
