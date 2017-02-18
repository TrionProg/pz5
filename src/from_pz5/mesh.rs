use std;
use Error;
use FromPz5LOD;

use GeometryType;

pub trait FromPz5Mesh:Sized{
    type LOD:FromPz5LOD <Reader=Self::Reader>;
    type Reader:std::io::Read;

    /*

    fn get_name(&self) -> &String;
    fn get_semantics(&self) -> &String;
    fn get_geometry_type(&self) -> GeometryType;
    fn get_lods(&self) -> &Vec<Self::LOD>;

    fn write(&self,write_to:&mut Pz5Writer) -> Result<(),Error>{

        Ok(())
    }

    */

    //fn write_mesh(&self,write_to:&mut Pz5Writer) -> Result<(),Error>;

    fn print(&self);
}
