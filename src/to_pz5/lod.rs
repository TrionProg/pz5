use std;

use Error;
use ToPz5BinaryData;

use config::write::Node;
use config::write::Struct;

pub trait ToPz5LOD:Sized{

    fn get_distance(&self) -> f32;
    fn get_data(&self) -> &[u8];
    fn get_all_data(&self) -> &[u8];
    fn get_vertices_count(&self) -> usize;

    fn write_pz5<'a>(&'a self, lod_struct:&mut Struct<'a>, binary_data:&mut ToPz5BinaryData<'a>) {
        lod_struct.add_field("vertices count", Node::Integer(self.get_vertices_count() as i64) );
        lod_struct.add_field("data index", Node::Integer(binary_data.add_data(self.get_all_data()) as i64) );

        //closure
    }

    fn print(&self);
}
