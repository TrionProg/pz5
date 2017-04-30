use std;

#[derive(Copy,Clone,Debug)]
pub struct Pos2D{
    pub x:f32,
    pub y:f32,
}

impl std::fmt::Display for Pos2D{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Pos2D(x:{}, y:{})", self.x, self.y)
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Pos3D{
    pub x:f32,
    pub y:f32,
    pub z:f32,
}

impl Pos3D {
    pub fn new(x:f32,y:f32,z:f32) -> Self{
        Pos3D{
            x:x,
            y:y,
            z:z,
        }
    }
}

impl std::fmt::Display for Pos3D{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Pos3D(x:{}, y:{}, z:{})", self.x, self.y, self.z)
    }
}

#[derive(Copy,Clone,Debug)]
pub enum Position{
    Pos2D(Pos2D),
    Pos3D(Pos3D),
}

impl std::fmt::Display for Position{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Position::Pos2D(pos2d) => write!(f,"{}",pos2d),
            Position::Pos3D(pos3d) => write!(f,"{}",pos3d),
        }
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Scale2D{
    pub x:f32,
    pub y:f32,
}

impl std::fmt::Display for Scale2D{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Scale2D(x:{}, y:{})", self.x, self.y)
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Scale3D{
    pub x:f32,
    pub y:f32,
    pub z:f32,
}

impl Scale3D {
    pub fn new(x:f32,y:f32,z:f32) -> Self{
        Scale3D{
            x:x,
            y:y,
            z:z,
        }
    }
}

impl std::fmt::Display for Scale3D{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Scale3D(x:{}, y:{}, z:{})", self.x, self.y, self.z)
    }
}

#[derive(Copy,Clone,Debug)]
pub enum Scale{
    Scale2D(Scale2D),
    Scale3D(Scale3D),
}

impl std::fmt::Display for Scale{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Scale::Scale2D(scale2d) => write!(f,"{}",scale2d),
            Scale::Scale3D(scale3d) => write!(f,"{}",scale3d),
        }
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Degrees{
    pub angle:f32,
}

impl std::fmt::Display for Degrees{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Degrees({})", self.angle)
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Radians{
    pub angle:f32,
}

impl std::fmt::Display for Radians{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Radians({})", self.angle)
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Euler{
    pub pitch:f32,
    pub yaw:f32,
    pub roll:f32,
}

impl Euler {
    pub fn new(x:f32,y:f32,z:f32) -> Self{
        Euler{
            pitch:x,
            yaw:y,
            roll:z,
        }
    }
}

impl std::fmt::Display for Euler{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Euler(pitch:{}, yaw:{}, roll:{})", self.pitch, self.yaw, self.roll)
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Quaternion{
    pub x:f32,
    pub y:f32,
    pub z:f32,
    pub w:f32,
}

impl std::fmt::Display for Quaternion{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Quaternion(x:{}, y:{}, z:{}, w:{})", self.x, self.y, self.z, self.w)
    }
}

#[derive(Copy,Clone,Debug)]
pub enum Rotation{
    Degrees(Degrees),
    Radians(Radians),
    Euler(Euler),
    Quaternion(Quaternion),
}

impl std::fmt::Display for Rotation{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Rotation::Degrees(degress) => write!(f,"{}",degress),
            Rotation::Radians(radians) => write!(f,"{}",radians),
            Rotation::Euler(euler) => write!(f,"{}",euler),
            Rotation::Quaternion(quaternion) => write!(f,"{}",quaternion),
        }
    }
}

pub struct Matrix4{
    pub mat:[f32;16],
}

const DEGRESS_IN_RADIAN:f32=180.0 / std::f32::consts::PI;

impl Matrix4 {
    pub fn new(position:&Position,rotation:&Rotation,scale:&Scale) -> Self {
        let mut matrix=match *rotation{
            Rotation::Euler(ref euler) => {
                match *scale {
                    Scale::Scale3D(ref scale3d) => {
                        let rotation_matrix=Self::from_euler(euler);
                        //let scale_matrix=Self::from_scale3d(scale3d);
                        rotation_matrix
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        };

        match *position{
            Position::Pos2D(ref pos2d) => {
                matrix.mat[3]=pos2d.x;
                matrix.mat[7]=pos2d.y;
                matrix.mat[11]=0.0;
            },
            Position::Pos3D(ref pos3d) => {
                matrix.mat[3]=pos3d.x;
                matrix.mat[7]=pos3d.y;
                matrix.mat[11]=pos3d.z;
            },
        }

        matrix
    }

    fn from_raw(mat:[f32;16]) -> Self {
        Matrix4{
            mat:mat,
        }
    }

    /*
    fn from_scale3d(scale3d:&Scale3D) -> Self {
        Matrix4::from_raw([
            scale3d.x,0.0,0.0,0.0,
            0.0,scale3d.y,0.0,0.0,
            0.0,0.0,scale3d.z,0.0,
            0.0,0.0,0.0,0.1
        ])
    }
    */

    fn from_euler(euler:&Euler) -> Self{
        let mut mat=[0.0;16];//TODO:unsafe do not zero

        let pitch=-euler.pitch/DEGRESS_IN_RADIAN;
        let yaw=euler.yaw/DEGRESS_IN_RADIAN;
        let roll=euler.roll/DEGRESS_IN_RADIAN;

        let A      = pitch.cos();
        let B      = pitch.sin();
        let C      = yaw.cos();
        let D      = yaw.sin();
        let E      = roll.cos();
        let F      = roll.sin();

        let AD     =   A * D;
        let BD     =   B * D;

        mat[0] =   C * E;
        mat[1] =  -C * F;
        mat[2] =  -D;
        mat[4] = -BD * E + A * F;
        mat[5] =  BD * F + A * E;
        mat[6] =  -B * C;
        mat[8] =  AD * E + B * F;
        mat[9] = -AD * F + B * E;
        mat[10]=   A * C;

        //mat[3] =  mat[7] = mat[11] = mat[12] = mat[13] = mat[14] = 0;
        //NOTE:3,7,11 are x,y,z
        mat[15]=  1.0;//is scale

        Matrix4::from_raw(mat)
    }
}
