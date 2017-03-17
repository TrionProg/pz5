extern crate rustc_serialize;
extern crate config;
extern crate lexer;

pub mod vertex_format;

mod error;
pub use error::Error;

mod math_types;
pub use math_types::{Pos2D,Pos3D,Position};
pub use math_types::{Scale2D,Scale3D,Scale};
pub use math_types::{Degrees,Radians,Euler,Quaternion,Rotation};
pub use math_types::Matrix4;

mod geometry_type;
pub use geometry_type::GeometryType;

mod geometry;
pub use geometry::Pz5Geometry;

mod to_pz5;
pub use to_pz5::{ToPz5BinaryData, ToPz5LOD, ToPz5Mesh, ToPz5Model};

mod from_pz5;
pub use from_pz5::{FromPz5LOD, FromPz5Mesh, FromPz5Model};

//TODO:use same id for same data(and how to remember it?)
//TODO:and use same data as Rc and just one buffer in GPU
