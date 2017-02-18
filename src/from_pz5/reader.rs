use std;

use rustc_serialize::base64::{ToBase64, STANDARD};

struct Pz5Writer<'a,'b,Writer:std::io::Write>{
    writer:&'a Writer,
    binary_data:Vec<&'b Vec<u8> >,
}

impl<'a,'b,Writer:std::io::Write> for Pz5Writer<'a,'b,Writer>{
    pub fn new(writer:&mut Writer) -> Pz5Writer{
        Pz5Writer{
            writer:writer,
            binary_data:Vec::with_capacity(16),
        }
    }

    /*
    //pub fn print(&mut self,data:

    pub fn begin_struct(&mut self,struct_name:Option<& str>) -> Result<(),std::io::Error>{
        self.writer.write_all(self.tabulation.as_bytes())?;

        match struct_name{
            Some( struct_name ) =>
                self.writer.write_all(format!(" \"{}\" {\n").as_bytes())?,
            None => self.writer.write_all("{\n".as_bytes())?,
        }

        self.tabulation.push_str("    ");

        Ok(())
    }

    pub fn end_struct(&mut self) -> Result<(),std::io::Error>{
        self.tabulation.truncate(self.tabulation.len()-4);

        self.writer.write_all(self.tabulation.as_bytes())?;
        self.writer.write_all("{\n".as_bytes())?;

        Ok(())
    }

    pub fn write_field(name:&str, value:&str, write_comma:bool) -> Result<(),std::io::Error>{
        self.writer.write_all(self.tabulation.as_bytes())?;
        if write_comma {
            self.writer.write_all(format!("\"{}\" = \"{}\",\n", name, value).as_bytes())?;
        }else{
            self.writer.write_all(format!("\"{}\" = \"{}\"\n", name, value).as_bytes())?;
        }

        Ok(())
    }

    pub fn write_association(name:&str, value:&str, write_comma:bool) -> Result<(),std::io::Error>{
        self.writer.write_all(self.tabulation.as_bytes())?;
        if write_comma {
            self.writer.write_all(format!("\"{}\" => \"{}\",\n", name, value).as_bytes())?;
        }else{
            self.writer.write_all(format!("\"{}\" => \"{}\"\n", name, value).as_bytes())?;
        }

        Ok(())
    }

    */

    pub fn add_data(&mut self,data:&'b Vec<u8>) -> usize{
        let index=self.binary_data.len();
        self.binary_data.push(data);

        index
    }

    pub fn finish(&mut self) -> Result<(),std::io::Error>{
        //TODO:write json
        self.writer.write("\"binary data\" = [\n".as_bytes())?;

        let data_count=self.binary_data.len();

        for (index, data) in self.binary_data.iter().enumerate(){
            let base64=data[..].to_base64(STANDARD);

            self.writer.write(format!("    {} => \"", index).as_bytes())?;
            self.writer.write(data.as_bytes()),

            if index==data_count-1 {
                self.writer.write("\"\n".as_bytes())?;
            }else{
                self.writer.write("\",\n".as_bytes())?;
            }
        }

        self.writer.write("]\n".as_bytes())?;

        Ok(())
    }
}
