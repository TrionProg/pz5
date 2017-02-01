use std;

use lexer;
use lexer::{Cursor,Lexeme};

pub struct Semantics<'a>{
    pub sources:Vec<SemanticsSource<'a>>,
}

pub struct SemanticsSource<'a>{
    pub name:&'a str,
    pub is_index:bool,
    pub layers:Vec<SemanticsSourceLayer<'a>>,
}

pub struct SemanticsSourceLayer<'a>{
    pub name:&'a str,
    pub layer_type:SemanticsSourceLayerType,
}

#[derive(Copy,Clone)]
pub enum SemanticsSourceLayerType{
    Float,
    Int,
}

impl<'a> Semantics<'a>{
    pub fn parse(semantics_text:&'a String) -> Result<Semantics<'a>, Error>{
        let mut cursor=Cursor::new( semantics_text );

        let mut sources=Vec::new();

        loop{
            match cursor.next()?{
                Lexeme::EOF => break,
                Lexeme::String(source_name) => {
                    if cursor.next()?!=Lexeme::Colon {
                        return Err( Error::UnexpectedLexeme(cursor.printLine(), "':'", cursor.lex.print()) );
                    }

                    let is_index=match cursor.next()?{
                        Lexeme::Ampersand => { cursor.next()?; true},
                        _ => false,
                    };

                    if cursor.lex!=Lexeme::Bracket('(') {
                        return Err( Error::UnexpectedLexeme(cursor.printLine(), "'('", cursor.lex.print()) );
                    }

                    let mut layers=Vec::new();

                    loop{
                        match cursor.next()?{
                            Lexeme::String(layer_name) => {
                                if cursor.next()?!=Lexeme::Colon {
                                    return Err( Error::UnexpectedLexemeAfter(cursor.printLine(), ":<type>", String::from(layer_name), cursor.lex.print()) );
                                }

                                let layer_type=match cursor.next()?{
                                    Lexeme::String(type_str) => {
                                        match type_str {
                                            "float" => SemanticsSourceLayerType::Float,
                                            "integer" => SemanticsSourceLayerType::Int,
                                            _ => return Err( Error::UnexpectedLexeme(cursor.printLine(), "float or integer", cursor.lex.print()) ),
                                        }
                                    },
                                    _ => return Err( Error::UnexpectedLexeme(cursor.printLine(), "float or integer", cursor.lex.print()) ),
                                };

                                layers.push(
                                    SemanticsSourceLayer{
                                        name:layer_name,
                                        layer_type:layer_type,
                                    }
                                );

                                match cursor.next()?{
                                    Lexeme::Comma => {},
                                    Lexeme::Bracket(')') => break,
                                    _ => return Err( Error::UnexpectedLexeme(cursor.printLine(), "',' or ')'", cursor.lex.print()) ),
                                }
                            },
                            _ => return Err( Error::UnexpectedLexeme(cursor.printLine(), "name of layer", cursor.lex.print()) ),
                        }
                    }

                    if layers.len()==0 {
                        return Err( Error::Other(format!("Source \"{}\" of semantics is empty",source_name)) );
                    }

                    sources.push(
                        SemanticsSource{
                            name:source_name,
                            is_index:is_index,
                            layers:layers,
                        }
                    );
                },
                _ => return Err( Error::UnexpectedLexeme(cursor.printLine(), "name of source", cursor.lex.print()) ),
            }
        }

        if sources.len()==0 {
            return Err( Error::Other(String::from("Semantics is empty")) );
        }

        Ok(
            Semantics{
                sources:sources,
            }
        )
    }
}

#[derive(Debug)]
pub enum Error{
    LexerError(lexer::Error),
    UnexpectedLexeme(String, &'static str, String),
    UnexpectedLexemeAfter(String, &'static str, String, String),
    Other(String),
}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Error::LexerError(ref error) => write!(f, "{}", error),
            Error::UnexpectedLexeme(ref line, expected, ref unexpected) => write!(f,"Expected {}, but {} has been found\n{}", expected, unexpected, line),
            Error::UnexpectedLexemeAfter(ref line, expected, ref after, ref unexpected) => write!(f,"Expected {} after {}, but {} has been found\n{}", expected, after, unexpected, line),
            Error::Other(ref message) => write!(f, "{}", message),
        }
    }
}

impl From<lexer::Error> for Error {
    fn from(e:lexer::Error) -> Error {
        Error::LexerError(e)
    }
}

//TODO:impl std::error::Error for Error {
