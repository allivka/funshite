
use std::fmt;

#[derive(Copy, Clone)]
pub struct Vec2i(pub isize, pub isize);

impl fmt::Debug for Vec2i {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec2i({}, {})", self.0, self.1)
    }
}

impl fmt::Display for Vec2i {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Copy, Clone)]
pub struct Vec2d(pub f64, pub f64);

impl fmt::Debug for Vec2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec2d({:.2}, {:.2})", self.0, self.1)
    }
}

impl fmt::Display for Vec2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.0, self.1)
    }
}

#[derive(Copy, Clone)]
pub struct Vec3d(pub f64, pub f64, pub f64);

impl fmt::Debug for Vec3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3d({:.2}, {:.2}, {:.2})", self.0, self.1, self.2)
    }
}

impl fmt::Display for Vec3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.0, self.1, self.2)
    }
}

pub const I3X3: [[f64; 3]; 3] = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
];

pub const ZERO3X3: [[f64; 3]; 3] = [
    [0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0],
];
pub fn new_vec3d_rotation_matrix(axis: Axis) -> [[f64; 3]; 3] {
    match axis {
        Axis::X(a) => {

            let s = a.sin();
            let c = a.cos();

            [
                [1.0, 0.0, 0.0],
                [0.0, c, -s],
                [0.0, s, c],
            ]
        },

        Axis::Y(a) => {

            let s = a.sin();
            let c = a.cos();

            [
                [c, 0.0, s],
                [0.0, 1.0, 0.0],
                [-s, 0.0, c],
            ]
        },

        Axis::Z(a) => {

            let s = a.sin();
            let c = a.cos();

            [
                [c, -s, 0.0],
                [s, c, 0.0],
                [0.0, 0.0, 1.0],
            ]
        },
    }
}

pub fn multiply_matrix3x3(base: &[[f64; 3]; 3], factor: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut t: [[f64; 3]; 3] = ZERO3X3;

    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                t[i][j] += base[i][k] * factor[k][j];
            }
        }
    }

    t
}

pub fn multiply_vector3d_by_matrix3x3(v: Vec3d, r: &[[f64; 3]; 3]) -> Vec3d {
    Vec3d(
        r[0][0] * v.0 + r[0][1] * v.1 + r[0][2] * v.2,
        r[1][0] * v.0 + r[1][1] * v.1 + r[1][2] * v.2,
        r[2][0] * v.0 + r[2][1] * v.1 + r[2][2] * v.2,
    )
}

pub fn transpose_matrix3x3(m: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    [
        [m[0][0], m[1][0], m[2][0]],
        [m[0][1], m[1][1], m[2][1]],
        [m[0][2], m[1][2], m[2][2]],
    ]
}

#[derive(Copy, Clone)]
pub struct LineCoefficients {
    pub k: f32,
    pub b: f32
}

impl LineCoefficients {
    pub fn new_from_line(start: Vec2i, end: Vec2i) -> Self {
        let dx = (end.0 - start.0) as f32;
        let dy = (end.1 - start.1) as f32;

        let k = if dx == 0.0 {
            f32::INFINITY
        } else {
            dy / dx
        };

        let b = if dx == 0.0 {
            start.0 as f32
        } else {
            start.1 as f32 - k * start.0 as f32  // y = kx + b → b = y - kx
        };

        LineCoefficients { k, b }
    }
}

#[derive(Copy, Clone)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {

    pub fn from_argb_u32(n: u32) -> RGBA {
        RGBA {
            r: (0b1111_1111 & (n >> 16)) as u8,
            g: (0b1111_1111 & (n >> 8)) as u8,
            b: (0b1111_1111 & (n >> 0)) as u8,
            a: (0b1111_1111 & (n >> 24)) as u8,
        }
    }
    
    pub fn r(n: u8) -> RGBA {
        RGBA {
            r: n,
            g: 0,
            b: 0,
            a: 0,
        }
    }

    pub fn g(n: u8) -> RGBA {
        RGBA {
            r: 0,
            g: n,
            b: 0,
            a: 0,
        }
    }

    pub fn b(n: u8) -> RGBA {
        RGBA {
            r: 0,
            g: 0,
            b: n,
            a: 0,
        }
    }

    pub fn a(n: u8) -> RGBA {
        RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: n,
        }
    }

    pub fn to_argb_u32(&self) -> u32 {
        (self.a as u32) << 24 | (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }

}

pub enum MotionKind {
    Rotation,
    Linear,
}

pub enum DirectionKind {
    Straight,
    Vertical,
    Horizontal,
}

pub enum Axis {
    X(f64),
    Y(f64),
    Z(f64),
}

pub struct Motion {
    pub kind: MotionKind,
    pub direction: DirectionKind,
    pub axis: Axis,
}
