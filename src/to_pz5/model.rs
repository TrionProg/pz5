use std;
use config;

use Error;
use ToPz5Mesh;
use ToPz5BinaryData;

use config::write::Config;
use config::write::Node;
use config::write::Struct;
use config::write::List;
use config::write::{SplitElementsWith, SplitFieldsWith, AssignmentSymbal};

use std::collections::HashMap;

pub trait ToPz5Model:Sized{
    type Mesh:ToPz5Mesh;

    fn get_name(&self) -> &String;
    fn get_meshes(&self) -> &HashMap<String, Self::Mesh>;

    fn write_pz5<W>(&self,write_to:&mut W) -> Result<(),Error> where W:std::io::Write {
        let mut config=Config::new();
        let mut binary_data=ToPz5BinaryData::new();
        {
            let mut root=config.get_root();

            root.add_field("name", Node::TextRef(self.get_name().as_str()) );

            //before meshes

            self.write_meshes_pz5(&mut root, &mut binary_data);
        }

        let mut writer=config::write::Writer::new( write_to );

        match config.write(&mut writer) {
            Ok ( _ ) => {},
            Err( e ) => return Err(Error::Pz5DocumentWriteError(e)),
        }

        match binary_data.write_pz5(&mut writer) {
            Ok ( _ ) => {},
            Err( e ) => return Err(Error::Pz5BinaryDataWriteError(e)),
        }

        //binary_data.write(&mut writer)?;

        Ok(())
    }

    fn write_meshes_pz5<'a>(&'a self, root:&mut Struct<'a>, binary_data:&mut ToPz5BinaryData<'a>) {
        let mut list=List::new(None, SplitElementsWith::Comma);

        for (_,mesh) in self.get_meshes().iter() {
            let mut mesh_struct=Struct::new(Some("Mesh"), AssignmentSymbal::Eq, SplitFieldsWith::Semicolon);

            mesh.write_pz5(&mut mesh_struct,binary_data);

            list.add_element(Node::Struct(mesh_struct));
        }

        root.add_field("meshes", Node::List(list));
    }

    //fn write_model(&self,write_to:&mut Pz5Writer) -> Result<(),Error>;

    fn print(&self);
    //fn print(&self,tabulation:&String);
    //fn print_tree(&self,tabulation:&String){

}
