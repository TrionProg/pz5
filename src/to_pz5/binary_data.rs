use std;
use config;
use rustc_serialize;


pub struct ToPz5BinaryData<'a>{
    list:Vec<&'a[u8]>,
}

impl<'a> ToPz5BinaryData<'a>{
    pub fn new() -> Self{
        ToPz5BinaryData{
            list:Vec::with_capacity(16),
        }
    }

    pub fn add_data(&mut self,data:&'a[u8]) -> usize{
        let index=self.list.len();
        self.list.push(data);

        index
    }

    pub fn write_pz5<W>(&self,writer:&mut config::write::Writer<W>) -> Result<(),config::write::Error> where W:std::io::Write{
        writer.write_str("\"binary data\" : [\n");
        writer.inc_tab();

        for (i,data) in self.list.iter().enumerate() {
            writer.write_tab()?;
            writer.write_str(i.to_string().as_ref())?;
            writer.write_str(" => \"")?;

            use rustc_serialize::base64::{ToBase64, FromBase64, STANDARD};

            let data_base64=data.to_base64(STANDARD);

            writer.write_str(data_base64.as_str())?;
            writer.write_str("\"\n")?;
        }

        writer.dec_tab();
        writer.write_str("]")?;

        Ok(())
    }
}
