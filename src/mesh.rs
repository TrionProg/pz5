use std;
use Error;
use Pz5LOD;

use GeometryType;

pub trait Pz5Mesh:Sized{
    type LOD:Pz5LOD;

    fn get_name(&self) -> &String;
    fn get_semantics(&self) -> &String;
    fn get_geometry_type(&self) -> GeometryType;
    fn get_lods(&self) -> &Vec<Self::LOD>;

    fn write<WriteTo:std::io::Write>(&self,write_to:&mut WriteTo) -> Result<(),Error>;


    fn read<ReadFrom:std::io::Read>(read_from:&ReadFrom) -> Result<Self,Error>;

    fn print(&self);
}
