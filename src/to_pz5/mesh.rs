use std;

use Error;
use ToPz5LOD;
use ToPz5BinaryData;

use config::write::Node;
use config::write::Struct;
use config::write::List;
use config::write::Association;
use config::write::{SplitElementsWith, SplitFieldsWith, AssignmentSymbal};

use GeometryType;

pub trait ToPz5Mesh:Sized{
    type LOD:ToPz5LOD;

    fn get_name(&self) -> &String;
    fn get_semantics(&self) -> &String;
    fn get_geometry_type(&self) -> GeometryType;
    fn get_lods(&self) -> &Vec<Self::LOD>;

    fn write_pz5<'a>(&'a self, mesh_struct:&mut Struct<'a>, binary_data:&mut ToPz5BinaryData<'a>) {
        mesh_struct.add_field("name", Node::TextRef(self.get_name().as_str()) );
        mesh_struct.add_field("semantics", Node::TextRef(self.get_semantics().as_str()) );
        mesh_struct.add_field("geometry type", Node::TextRef(self.get_geometry_type().print()) );

        self.write_lods_pz5(mesh_struct, binary_data);
    }


    fn write_lods_pz5<'a>(&'a self, mesh_struct:&mut Struct<'a>, binary_data:&mut ToPz5BinaryData<'a>) {
        let mut list=List::new(None, SplitElementsWith::Space);

        for lod in self.get_lods().iter() {
            let mut lod_struct=Struct::new(None, AssignmentSymbal::Colon, SplitFieldsWith::Comma);

            lod.write_pz5(&mut lod_struct, binary_data);

            let lod_association=Association::new(
                Node::Float(lod.get_distance() as f64),
                Node::Struct(lod_struct),
            );

            list.add_element(Node::Association(lod_association));
        }

        mesh_struct.add_field("lods", Node::List(list));
    }

    fn print(&self);
}
