use std;
use Error;

pub trait Pz5LOD:Sized{
    fn get_distance(&self) -> f32;
    fn get_data(&self) -> &[u8];
    fn get_all_data(&self) -> &[u8];
    fn get_vertices_count(&self) -> usize;

    fn write<WriteTo:std::io::Write>(&self,write_to:&mut WriteTo) -> Result<(),Error>;


    fn read<ReadFrom:std::io::Read>(read_from:&ReadFrom) -> Result<Self,Error>;

    fn print(&self);
}
