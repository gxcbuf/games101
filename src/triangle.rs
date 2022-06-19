use glam::{Vec2, Vec3, Vec4};

#[derive(Debug)]
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
        self.color[idx] = Vec3::new(r, g, b)
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

    /// 目前只返回一种颜色
    pub fn get_color(&self) -> &Vec3 {
        &self.color[0]
    }

    /// 计算屏幕空间坐标(x, y)是否在三角形内部
    pub fn is_inside(&self, x: usize, y: usize) -> bool {
        let v = &self.v;
        let p = Vec3::new(x as f32, y as f32, 0.0);
        // 三角形三个顶点，忽略z坐标
        let p0 = Vec3::new(v[0].x, v[0].y, 0.0);
        let p1 = Vec3::new(v[1].x, v[1].y, 0.0);
        let p2 = Vec3::new(v[2].x, v[2].y, 0.0);

        let i = (p1 - p0).cross(p - p0).z;
        let j = (p2 - p1).cross(p - p1).z;
        let k = (p0 - p2).cross(p - p2).z;

        if i > 0.0 {
            j > 0.0 && k > 0.0
        } else {
            j < 0.0 && k < 0.0
        }
    }

    /// 计算重心坐标
    pub fn compute_barycentric_2d(&self, x: usize, y: usize) -> (f32, f32, f32) {
        let x = x as f32;
        let y = y as f32;

        let v = &self.v;

        let alpha = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y
            - v[2].x * v[1].y)
            / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y
                - v[2].x * v[1].y);
        let beta = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y
            - v[0].x * v[2].y)
            / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y
                - v[0].x * v[2].y);

        let gamma = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y
            - v[1].x * v[0].y)
            / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y
                - v[1].x * v[0].y);

        (alpha, beta, gamma)
    }
}
