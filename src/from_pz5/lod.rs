use std;
use Error;

pub trait FromPz5LOD:Sized{
    type Reader:std::io::Read;

    /*

    fn get_distance(&self) -> f32;
    fn get_data(&self) -> &[u8];
    fn get_all_data(&self) -> &[u8];
    fn get_vertices_count(&self) -> usize;

    fn write(&self,write_to:&mut Pz5Writer) -> Result<(),Error>{

        Ok(())
    }
    */

    //fn write_lod(&self,write_to:&mut Pz5Writer) -> Result<(),Error>;

    fn print(&self);
}
