use glam::{Vec2, Vec3, Vec4};

pub struct Triangle {
    // 三角形的三个点, v0, v1, v2
    pub v: [Vec3; 3],

    // 每个顶点的数据
    pub color: [Vec3; 3],      // 每个顶点的颜色
    pub tex_coords: [Vec2; 3], // texture u, v
    pub normal: [Vec3; 3],     // 每个顶点法向量
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            v: [Vec3::ZERO; 3],
            color: [Vec3::ZERO; 3],
            tex_coords: [Vec2::ZERO; 3],
            normal: [Vec3::ZERO; 3],
        }
    }

    pub fn a(&self) -> Vec3 {
        self.v[0]
    }
    pub fn b(&self) -> Vec3 {
        self.v[1]
    }
    pub fn c(&self) -> Vec3 {
        self.v[2]
    }

    pub fn set_vertex(&mut self, idx: usize, ver: Vec3) {
        self.v[idx] = ver;
    }

    pub fn set_normal(&mut self, idx: usize, n: Vec3) {
        self.normal[idx] = n;
    }

    pub fn set_color(&mut self, idx: usize, r: f32, g: f32, b: f32) {
        if r < 0.0 || r > 255.0 || g < 0.0 || g > 255.0 || b < 0.0 || b > 255.0 {
            panic!("Invalid color values");
        }
        self.color[idx] = Vec3::new(r / 255.0, g / 255.0, b / 255.0)
    }

    pub fn set_texcoord(&mut self, idx: usize, s: f32, t: f32) {
        self.tex_coords[idx] = Vec2::new(s, t);
    }

    pub fn to_vec4(&self) -> [Vec4; 3] {
        let mut res: [Vec4; 3] = [Vec4::ZERO; 3];
        for (idx, vex) in self.v.iter().enumerate() {
            res[idx] = Vec4::new(vex.x, vex.y, vex.z, 1.0);
        }
        res
    }
}
