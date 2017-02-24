use std;
use lexer;

use lexer::stream_lexer::{Cursor,Lexeme};

use super::Error;
use super::VertexFormatSource;

pub struct VertexFormat<'a>{
    pub sources:Vec<VertexFormatSource<'a>>,
}

impl<'a> VertexFormat<'a>{
    pub fn parse(vertex_format_text:&'a String) -> Result<Self, Error<'a>>{
        let mut sources=Vec::new();

        let mut cursor=Cursor::new( vertex_format_text );

        loop{
            match cursor.next_lex()? {
                Lexeme::EOF => break,
                Lexeme::Ident(source_name) => {
                    let source=VertexFormatSource::parse(&mut cursor, source_name)?;

                    sources.push( source );
                },
                _ => return Err( Error::UnexpectedLexeme(cursor.get_line(), "name of source", cursor.lex.clone()) ),
            }
        }

        if sources.len()==0 {
            return Err( Error::VertexFormatIsEmpty );
        }

        Ok(
            VertexFormat{
                sources:sources,
            }
        )
    }

    pub fn remove_indexes(&mut self) {
        for source in self.sources.iter_mut(){
            source.is_index=false;
        }
    }

    pub fn print(&self) -> String{
        let mut vertex_format_text=String::with_capacity(16);

        for source in self.sources.iter().take(self.sources.len()-1) {
            source.print(&mut vertex_format_text);
            vertex_format_text.push_str(" ");
        }

        match self.sources.iter().last(){
            Some( source ) => {source.print(&mut vertex_format_text);},
            None => {},
        }

        vertex_format_text
    }
}
