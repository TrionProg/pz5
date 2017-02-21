use std;
use VertexFormat;

pub struct Pz5Geometry{
    data:Vec<u8>,
}

impl Pz5Geometry{
    pub fn from_raw(data:Vec<u8>) -> Self{
        Pz5Geometry{
            data:data,
        }
    }

    pub fn as_buf<V>(&self) -> &[V] {
        use std::mem::size_of;
        use std::mem::transmute;
        use std::slice;

        unsafe{
            let vertex_data=transmute::<&[u8], &[V]>(&self.data[..]);
            slice::from_raw_parts::<V>( &vertex_data[0] as *const V , self.data.len()/size_of::<V>() )
        }
    }

    //pub fn build_by_vertex_format(&self, self_vf:&VertexFormat, output_vf:&VertexFormat) -> Pz5Geometry{
    //}
}
