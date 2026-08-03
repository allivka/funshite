
#[derive(Copy, Clone)]
pub struct Vec2i(pub isize, pub isize);

#[derive(Copy, Clone)]
pub struct Vec2d(pub f64, pub f64);


#[derive(Copy, Clone)]
pub struct Vec3d(pub f64, pub f64, pub f64);

pub const I3X3: [[f64; 3]; 3] = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
];

pub const ZERO3X3: [[f64; 3]; 3] = [
    [0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0],
];
pub enum RotationMatrixKind {
    X(f64),
    Y(f64),
    Z(f64)
}

pub fn new_vec3d_rotation_matrix(kind: RotationMatrixKind) -> [[f64; 3]; 3] {
    match kind {
        RotationMatrixKind::X(a) => {

            let s = a.sin();
            let c = a.cos();

            [
                [1.0, 0.0, 0.0],
                [0.0, c, -s],
                [0.0, s, c],
            ]
        },

        RotationMatrixKind::Y(a) => {

            let s = a.sin();
            let c = a.cos();

            [
                [c, 0.0, s],
                [0.0, 1.0, 0.0],
                [-s, 0.0, c],
            ]
        },

        RotationMatrixKind::Z(a) => {

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

pub fn multiply_matrix_3x3(base: &[[f64; 3]; 3], factor: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
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
pub struct RGBA(pub u8, pub u8, pub u8, pub u8);

impl RGBA {

    pub fn from_argb_u32(n: u32) -> RGBA {
        RGBA(
            (0b1111_1111 & (n >> 16)) as u8,
            (0b1111_1111 & (n >> 8)) as u8,
            (0b1111_1111 & (n >> 0)) as u8,
            (0b1111_1111 & (n >> 24)) as u8
        )
    }

    pub fn to_argb_u32(&self) -> u32 {
        (self.3 as u32) << 24 | (self.0 as u32) << 16 | (self.1 as u32) << 8 | (self.2 as u32)
    }

}