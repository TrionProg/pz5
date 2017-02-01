use std;
use std::str::CharIndices;

#[derive(PartialEq, PartialOrd, Clone)]
pub enum Lexeme<'a>{
    EOF,
    String(&'a str),
    Eq,
    Colon,
    Comma,
    NewLine,
    Bracket(char),
    Ampersand,
}

impl <'a>Lexeme<'a>{
    pub fn print(&self) -> String{
        match *self {
            Lexeme::EOF => String::from("EOF"),
            Lexeme::String( s ) => format!("\"{}\"", s),
            Lexeme::Eq => String::from("'='"),
            Lexeme::Colon => String::from("':'"),
            Lexeme::Comma => String::from("','"),
            Lexeme::NewLine => String::from("'new line'"),
            Lexeme::Bracket( b ) => format!("{}",b),
            Lexeme::Ampersand => String::from("&"),
        }
    }
}

pub struct Cursor<'a>{
    text:&'a str,
    it:CharIndices<'a>,

    curPos:usize,
    curChar:char,

    pub lineNumber:usize,
    pub lineBegin:usize,

    pub line:usize,
    pub pos:usize,
    pub lex:Lexeme<'a>,
}

impl<'a>Cursor<'a>{
    pub fn new( text:&'a str ) -> Cursor<'a>{
        let mut cursor=Cursor {
            text:text,
            it:text.char_indices(),

            curPos:0,
            curChar:'\0',

            lineNumber:1,
            lineBegin:0,

            line:0,
            pos:0,
            lex:Lexeme::EOF,
        };

        cursor.nextChar();

        cursor
    }

    pub fn printLine(&self) -> String{
        let (a,bc)=self.text.split_at(self.lineBegin);

        let line=match bc.find( '\n' ){
            Some( lineEnd ) => {
                let (b,c) = bc.split_at( lineEnd );
                b
            },
            None =>
                bc,
        };

        format!("Line: {} ; {} \n", self.lineNumber, line)
    }

    fn nextChar(&mut self){
        match self.it.next(){
            None => {
                self.curPos+=1;
                self.curChar='\0';
            },
            Some( (pos, ch) ) => {
                self.curPos=pos;
                self.curChar=ch;
            },
        }
    }

    pub fn next(&mut self) -> Result<Lexeme<'a>, Error>{
        loop{
            self.pos=self.curPos;
            self.line=self.lineNumber;

            if self.curChar.is_alphabetic() || self.curChar.is_digit(10) || self.curChar=='.' {
                while self.curChar.is_alphabetic() || self.curChar.is_digit(10) || self.curChar=='.' {
                    self.nextChar();
                }

                let (ab,c)=self.text.split_at(self.curPos);
                let (a,b)=ab.split_at(self.pos);

                self.lex=Lexeme::String(b);
            }else{
                match self.curChar {
                    '\0' => {
                        self.lex=Lexeme::EOF;
                    },
                    '\n' => {
                        self.lineBegin=self.curPos+1;
                        self.lineNumber+=1;

                        self.lex=Lexeme::NewLine;
                        self.nextChar();
                    },
                    '{' | '}' | '[' | ']' | '(' | ')' => {
                        self.lex=Lexeme::Bracket(self.curChar);
                        self.nextChar();
                    },
                    '=' => {
                        self.lex=Lexeme::Eq;
                        self.nextChar();
                    },
                    ':' => {
                        self.lex=Lexeme::Colon;
                        self.nextChar();
                    },
                    ',' => {
                        self.lex=Lexeme::Comma;
                        self.nextChar();
                    },
                    '&' => {
                        self.lex=Lexeme::Ampersand;
                        self.nextChar();
                    },
                    '"' | '\'' => {
                        let quotesChar=self.curChar;

                        self.nextChar();

                        self.pos=self.curPos;

                        while self.curChar!=quotesChar {
                            if self.curChar=='\n'{
                                self.lineBegin=self.curPos+1;
                                self.lineNumber+=1;
                            }else if self.curChar=='\0' {
                                return Err( Error::UnexpectedEOF(self.printLine()) );
                            }

                            self.nextChar();
                        }

                        let (ab,c)=self.text.split_at(self.curPos);
                        let (a,b)=ab.split_at(self.pos);

                        self.lex=Lexeme::String(b);

                        self.nextChar();
                    },
                    '/' => {
                        self.nextChar();

                        if self.curChar=='/' {
                            while self.curChar!='\n' || self.curChar!='\0' {
                                self.nextChar();
                            }

                            continue;
                        }else if self.curChar=='*' {
                            self.nextChar();

                            while self.curChar!='*' {
                                if self.curChar=='\n'{
                                    self.lineBegin=self.curPos+1;
                                    self.lineNumber+=1;
                                }else if self.curChar=='\0' {
                                    return Err( Error::UnexpectedEOF(self.printLine()) );
                                }

                                self.nextChar();
                            }

                            self.nextChar();

                            if self.curChar!='/' {
                                return Err( Error::Other(self.printLine(), String::from("expected '/' at end of multiline comment")) );
                            }

                            self.nextChar();

                            continue;
                        }else{
                            return Err( Error::UnexpectedSymbal(self.printLine(), '/') );
                        }
                    },
                    '\r' | '\t' | ' ' => {
                        loop{
                            match self.curChar{
                                '\r' | '\t' | ' ' => self.nextChar(),
                                _ => break,
                            }
                        }

                        continue;
                    },
                    _=>{
                        return Err( Error::UnexpectedSymbal(self.printLine(), self.curChar) );
                    },
                }
            }

            return Ok(self.lex.clone());
        }
    }
}

#[derive(Debug)]
pub enum Error{
    UnexpectedEOF(String),
    UnexpectedSymbal(String, char),
    Other(String,String),
}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Error::UnexpectedEOF(ref line) => write!(f, "Unexpected EOF\n{}", line),
            Error::UnexpectedSymbal(ref line, symbal) => write!(f, "Unexpected symbal {}\n{}", symbal, line),
            Error::Other(ref line, ref message) => write!(f, "{}\n{}", message, line),
        }
    }
}

//TODO:impl std::error::Error for Error {
