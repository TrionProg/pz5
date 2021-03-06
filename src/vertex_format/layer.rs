use std;
use lexer;

use lexer::stream_lexer::{Cursor,Lexeme};

use super::Error;

pub struct VertexFormatSourceLayer<'a>{
    pub name:&'a str,
    pub layer_type:VertexFormatSourceLayerType,
}

#[derive(Copy,Clone)]
pub enum VertexFormatSourceLayerType{
    F32,
    I32,
}

impl<'a> VertexFormatSourceLayer<'a>{
    pub fn new(name:&'a str, layer_type:VertexFormatSourceLayerType) -> Self {
        VertexFormatSourceLayer{
            name:name,
            layer_type:layer_type,
        }
    }

    pub fn parse(cursor:& mut Cursor<'a>, layer_name:&'a str) -> Result<Self, Error<'a>>{
        if cursor.next_lex()? != Lexeme::Colon {
            return Err( Error::UnexpectedLexemeAfter(cursor.get_line(), ":<type>", layer_name, cursor.lex.clone()) );
        }

        let layer_type=match cursor.next_lex()?{
            Lexeme::Ident(layer_type) => {
                match layer_type {
                    "f32" => VertexFormatSourceLayerType::F32,
                    "i32" => VertexFormatSourceLayerType::I32,
                    _ => return Err( Error::UnexpectedLexeme(cursor.get_line(), "f32 or i32", cursor.lex.clone()) ),
                }
            },
            _ => return Err( Error::UnexpectedLexeme(cursor.get_line(), "f32 or i32", cursor.lex.clone()) ),
        };

        Ok( VertexFormatSourceLayer::new(layer_name, layer_type) )
    }

    pub fn print(&self, vertex_format_text:&mut String){
        vertex_format_text.push_str(self.name);
        vertex_format_text.push_str(":");

        vertex_format_text.push_str(self.layer_type.print());
    }
}

impl VertexFormatSourceLayerType {
    pub fn get_size(&self) -> usize {
        match *self{
            VertexFormatSourceLayerType::F32 => 4,
            VertexFormatSourceLayerType::I32 => 4,
        }
    }

    pub fn print(&self) -> &'static str {
        match *self{
            VertexFormatSourceLayerType::F32 => "f32",
            VertexFormatSourceLayerType::I32 => "i32",
        }
    }
}
