use std;
use Error;
use FromPz5Mesh;

use std::collections::HashMap;

//TODO:add print, read

pub trait FromPz5Model:Sized{
    type Mesh:FromPz5Mesh <Reader=Self::Reader>;
    type Reader:std::io::Read;

    /*

    fn get_name(&self) -> &String;
    fn get_meshes(&self) -> &HashMap<String, Self::Mesh>;

    fn write(&self,write_to:&mut Pz5Writer) -> Result<(),Error>{

        Ok(())
    }

    */

    //fn write_model(&self,write_to:&mut Pz5Writer) -> Result<(),Error>;

    fn print(&self);
    //fn print(&self,tabulation:&String);
    //fn print_tree(&self,tabulation:&String){

}
