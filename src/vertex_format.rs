use std;

use lexer;
use lexer::{Cursor,Lexeme};

pub struct VertexFormat<'a>{
    pub sources:Vec<VertexFormatSource<'a>>,
}

pub struct VertexFormatSource<'a>{
    pub name:&'a str,
    pub is_index:bool,
    pub layers:Vec<VertexFormatSourceLayer<'a>>,
}

pub struct VertexFormatSourceLayer<'a>{
    pub name:&'a str,
    pub layer_type:VertexFormatSourceLayerType,
}

#[derive(Copy,Clone)]
pub enum VertexFormatSourceLayerType{//TODO:use f32, f64 etc
    Float,
    Int,
}

impl<'a> VertexFormat<'a>{
    pub fn parse(vertex_format_text:&'a String) -> Result<VertexFormat<'a>, Error>{
        let mut cursor=Cursor::new( vertex_format_text );

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
                                            "float" => VertexFormatSourceLayerType::Float,
                                            "integer" => VertexFormatSourceLayerType::Int,
                                            _ => return Err( Error::UnexpectedLexeme(cursor.printLine(), "float or integer", cursor.lex.print()) ),
                                        }
                                    },
                                    _ => return Err( Error::UnexpectedLexeme(cursor.printLine(), "float or integer", cursor.lex.print()) ),
                                };

                                layers.push(
                                    VertexFormatSourceLayer{
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
                        return Err( Error::Other(format!("Source \"{}\" of vertex_format is empty",source_name)) );
                    }

                    sources.push(
                        VertexFormatSource{
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
            return Err( Error::Other(String::from("VertexFormat is empty")) );
        }

        Ok(
            VertexFormat{
                sources:sources,
            }
        )
    }
}

//TODO:how to use vertex_format error?
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
    fn from(error:lexer::Error) -> Error {
        Error::LexerError(error)
    }
}

//TODO:impl std::error::Error for Error {
