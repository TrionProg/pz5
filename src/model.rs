use std;
use Error;
use Pz5Mesh;

use std::collections::HashMap;

pub trait Pz5Model:Sized{
    type Mesh:Pz5Mesh;

    fn get_name(&self) -> &String;
    fn get_meshes(&self) -> &HashMap<String, Self::Mesh>;

    fn write<WriteTo:std::io::Write>(&self,write_to:&mut WriteTo) -> Result<(),Error>;


    fn read<ReadFrom:std::io::Read>(read_from:&ReadFrom) -> Result<Self,Error>;

    fn print(&self);
    //fn print(&self,tabulation:&String);
    //fn print_tree(&self,tabulation:&String){

}
