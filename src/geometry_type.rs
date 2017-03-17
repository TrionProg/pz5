
#[derive(Copy,Clone,PartialEq)]
pub enum GeometryType{
    Points,
    Lines,
    Triangles,
}

impl GeometryType{
    pub fn from_vertices_count(vertices_count:usize) -> Result<GeometryType,String>{
        match vertices_count{
            1 => Ok(GeometryType::Points),
            2 => Ok(GeometryType::Lines),
            3 => Ok(GeometryType::Triangles),
            _ => Err(format!("Incorrect vertices count of geometry type {}",vertices_count))
        }
    }

    pub fn print(&self) -> &'static str{
        match *self{
            GeometryType::Points => "points",
            GeometryType::Lines => "lines",
            GeometryType::Triangles => "triangles",
        }
    }
}

//TODO:add Display
