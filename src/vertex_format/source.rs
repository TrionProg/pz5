use std;
use lexer;

use lexer::stream_lexer::{Cursor,Lexeme};

use super::Error;
use super::VertexFormatSourceLayer;

pub struct VertexFormatSource<'a>{
    pub name:&'a str,
    pub is_index:bool,
    pub layers:Vec<VertexFormatSourceLayer<'a>>,
}

impl<'a> VertexFormatSource<'a>{
    pub fn parse(cursor:& mut Cursor<'a>, source_name:&'a str) -> Result<Self, Error<'a>>{
        if cursor.next_lex()? != Lexeme::Colon {
            return Err( Error::UnexpectedLexeme(cursor.get_line(), "':'", cursor.lex.clone()) );
        }

        let is_index=match cursor.next_lex()? {
            Lexeme::Operator('&') => { cursor.next_lex()?; true},
            _ => false,
        };

        if cursor.lex != Lexeme::Bracket('(') {
            return Err( Error::UnexpectedLexeme(cursor.get_line(), "'('", cursor.lex.clone()) );
        }

        let mut layers=Vec::new();

        loop{
            match cursor.next_lex()? {
                Lexeme::Ident(layer_name) => {
                    let layer=VertexFormatSourceLayer::parse(cursor, layer_name)?;

                    layers.push( layer );

                    match cursor.next_lex()? {
                        Lexeme::Comma => {},
                        Lexeme::Bracket(')') => break,
                        _ => return Err( Error::UnexpectedLexeme(cursor.get_line(), "',' or ')'", cursor.lex.clone()) ),
                    }
                },
                _ => return Err( Error::UnexpectedLexeme(cursor.get_line(), "name of layer", cursor.lex.clone()) ),
            }
        }

        if layers.len()==0 {
            return Err( Error::SourceIsEmpty(source_name) );
        }

        Ok(
            VertexFormatSource{
                name:source_name,
                is_index:is_index,
                layers:layers,
            }
        )
    }

    pub fn print(&self, vertex_format_text:&mut String){
        vertex_format_text.push_str(self.name);
        vertex_format_text.push_str(":");

        if self.is_index {
            vertex_format_text.push_str("&");
        }

        vertex_format_text.push_str("(");

        for layer in self.layers.iter().take(self.layers.len()-1) {
            layer.print(vertex_format_text);
            vertex_format_text.push_str(",");
        }

        match self.layers.iter().last(){
            Some( layer ) => {layer.print(vertex_format_text);},
            None => {},
        }

        vertex_format_text.push_str(")");
    }
}
