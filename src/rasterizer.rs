use std::collections::HashMap;

use glam::{Mat4, Vec3, Vec4};

use crate::triangle::Triangle;

pub enum Buffer {
    Color,
    Depth,
    All,
}

#[derive(Debug)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub coord: Vec3,
    pub color: Vec3,
}

#[derive(Debug, Clone)]
/// 用于指示哪三个顶点构成了三角形
pub struct Indices(usize, usize, usize);

impl Indices {
    pub fn new(x: usize, y: usize, z: usize) -> Indices {
        Self(x, y, z)
    }
}

#[derive(Debug)]
pub struct Rasterizer {
    width: usize,
    height: usize,

    next_id: usize,

    model: Mat4,      // 模型矩阵
    view: Mat4,       // 视口矩阵
    projection: Mat4, // 透视矩阵

    pub frame_buf: Vec<Vec3>, // 缓冲区
    depth_buf: Vec<f32>,      // 深度缓存区

    vertex_buf: HashMap<usize, Vertex>,
    indice_buf: Vec<Indices>, // 指示哪些顶点构成了三角形
}

// public method
impl Rasterizer {
    pub fn new(w: usize, h: usize) -> Rasterizer {
        Self {
            width: w,
            height: h,
            next_id: 0,
            model: Mat4::ZERO,
            view: Mat4::ZERO,
            projection: Mat4::ZERO,
            frame_buf: vec![Vec3::ZERO; w * h],
            depth_buf: vec![0.0; w * h],
            vertex_buf: HashMap::new(),
            indice_buf: Vec::new(),
        }
    }

    pub fn set_mode(&mut self, m: Mat4) {
        self.model = m;
    }

    pub fn set_view(&mut self, v: Mat4) {
        self.view = v;
    }

    pub fn set_projection(&mut self, p: Mat4) {
        self.projection = p;
    }

    pub fn set_pixel2(&mut self, x: i32, y: i32, color: &Vec3) {
        let ind = self.get_index(x as usize, y as usize);
        // self.frame_buf[ind] = color.clone();
        self.frame_buf[ind] = Vec3::new(238.0, 217.0, 185.0);
        // println!("x, y: ({}, {}), index: {}", x, y, ind);
    }

    pub fn set_pixel(&mut self, point: &Vec3, color: &Vec3) {
        let w = self.width as f32;
        let h = self.height as f32;
        if point.x < 0.0 || point.x >= w || point.y < 0.0 || point.y >= h {
            // invalid pixel
            return;
        }
        // let ind = ((h - point.y) * w + point.x) as usize;
        let ind = self.get_index(point.x as usize, point.y as usize);
        self.frame_buf[ind] = color.clone();
    }

    pub fn clear(&mut self, buffer: Buffer) {
        match buffer {
            Buffer::Color => self.frame_buf.fill(Vec3::ZERO),
            Buffer::Depth => self.depth_buf.fill(0.0),
            Buffer::All => {
                self.frame_buf.fill(Vec3::ZERO);
                self.depth_buf.fill(0.0);
            }
        }
    }

    pub fn draw(&mut self, inds: &Vec<Indices>, ttype: Primitive) {
        match ttype {
            Primitive::Triangle => (),
            _ => panic!("Drawing primitives other than triangle is not implemented yet!"),
        }

        let triangles = self.calc_triangles(inds);

        for t in triangles.iter() {
            self.rasterize_triangle(t);
        }
    }

    pub fn frame_buf_data(&self) -> Vec<f32> {
        self.frame_buf
            .iter()
            .flat_map(|item| item.to_array())
            .collect::<Vec<f32>>()
    }

    pub fn insert_vertexs(&mut self, vertexs: &Vec<Vertex>) {
        for vex in vertexs.iter() {
            self.vertex_buf.insert(self.next_id, vex.clone());
            self.next_id += 1;
        }
    }

    pub fn insert_indices(&mut self, inds: &Vec<Indices>) {
        for ind in inds.iter() {
            self.indice_buf.push(ind.clone());
        }
    }
}

fn to_vec4(v: &Vec3, w: f32) -> Vec4 {
    v.extend(w)
}

// private method
impl Rasterizer {
    /// Bresenham 画线算法
    pub fn draw_line(&mut self, begin: Vec3, end: Vec3) {
        let x1 = begin.x as i32; // 509
        let y1 = begin.y as i32; // 515
        let x2 = end.x as i32; // 350
        let y2 = end.y as i32; // 350

        let color = Vec3::new(255.0, 255.0, 255.0);
        let mut point = Vec3::new(0.0, 0.0, 1.0);

        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut dx: i32 = 0;
        let mut dy: i32 = 0;
        let mut dx1: i32 = 0;
        let mut dy1: i32 = 0;
        let mut px: i32 = 0;
        let mut py: i32 = 0;
        let mut xe: i32 = 0;
        let mut ye: i32 = 0;

        dx = x2 - x1;
        dy = y2 - y1;
        dx1 = dx.abs();
        dy1 = dy.abs();
        px = 2 * dy1 - dx1;
        py = 2 * dx1 - dy1;

        if dy1 <= dx1 {
            if dx >= 0 {
                x = x1;
                y = y1;
                xe = x2;
            } else {
                x = x2;
                y = y2;
                xe = x1;
            }
            self.set_pixel2(x, y, &color);
            while x < xe {
                x += 1;
                if px < 0 {
                    px += 2 * dy1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        y += 1;
                    } else {
                        y -= 1;
                    }
                    px += 2 * (dy1 - dx1);
                }
                self.set_pixel2(x, y, &color);
            }
        } else {
            if dy >= 0 {
                x = x1;
                y = y1;
                ye = y2;
            } else {
                x = x2;
                y = y2;
                ye = y1;
            }
            self.set_pixel2(x, y, &color);
            while y < ye {
                y += 1;
                if py <= 0 {
                    py += 2 * dx1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        x += 1;
                    } else {
                        x -= 1;
                    }
                    py += 2 * (dx1 - dy1);
                }
                self.set_pixel2(x, y, &color);
            }
        }
    }

    fn rasterize_triangle(&mut self, t: &Triangle) {
        let v = t.to_vec4();
        // 1. 创建包围盒
        let min_x = v[0].x.min(v[1].x).min(v[2].x) as usize;
        let min_y = v[0].y.min(v[1].y).min(v[2].y) as usize;
        let max_x = v[0].x.max(v[1].x).max(v[2].x) as usize;
        let max_y = v[0].y.max(v[1].y).max(v[2].y) as usize;

        for x in min_x..max_x {
            for y in min_y..max_y {
                // 2. 遍历bounding box, 计算像素中心的屏幕空间坐标是否在三角形内
                if !t.is_inside(x, y) {
                    continue;
                }

                // 3. 如果在三角形内，对比其插值深度与深度缓存区的值
                let (alpha, beta, gamma) = t.compute_Barycentric_2d(x, y);
                let w_reciprocal = 1.0 / (alpha / v[0].w + beta / v[1].w + gamma / v[2].w);
                let mut z_interpolated =
                    alpha * v[0].z / v[0].w + beta * v[1].z / v[1].w + gamma * v[2].z / v[2].w;
                z_interpolated *= w_reciprocal;

                let idx = self.get_index(x, y);
                // 4. 如果更靠近相机，则更新缓存区颜色
                if z_interpolated < self.depth_buf[idx] {
                    self.set_pixel2(x as i32, y as i32, t.get_color());
                    self.depth_buf[idx] = z_interpolated;
                }
            }
        }
        println!("{:?}", t.get_color());
    }

    fn calc_triangles(&self, inds: &Vec<Indices>) -> Vec<Triangle> {
        let f1: f32 = (100.0 - 0.1) / 2.0;
        let f2: f32 = (100.0 + 0.1) / 2.0;
        let width = self.width as f32;
        let height = self.height as f32;

        let mut triangles = Vec::new();

        // mvp 矩阵
        let mvp: Mat4 = self.projection * self.view * self.model;
        for ind in inds.iter() {
            let vexs = self.get_vertexs(ind);
            // 转化为齐次坐标，添加一个维度
            let mut vertexs = [
                mvp * to_vec4(&vexs[0].coord, 1.0),
                mvp * to_vec4(&vexs[1].coord, 1.0),
                mvp * to_vec4(&vexs[2].coord, 1.0),
            ];

            for vex in vertexs.iter_mut() {
                *vex /= vex.w;
            }

            for vex in vertexs.iter_mut() {
                vex.x = 0.5 * width as f32 * (vex.x + 1.0);
                vex.y = 0.5 * height as f32 * (vex.y + 1.0);
                vex.z = vex.z * f1 * f2;
            }

            let mut t = Triangle::new();
            for (idx, vex) in vertexs.iter().enumerate() {
                t.set_vertex(idx, vex.truncate());
            }

            for (idx, vex) in vexs.iter().enumerate() {
                t.set_color(idx, vex.color.x, vex.color.y, vex.color.z);
            }

            triangles.push(t);
        }
        triangles
    }

    fn rasterize_wireframe(&mut self, t: &Triangle) {
        self.draw_line(t.c(), t.a());
        self.draw_line(t.c(), t.b());
        self.draw_line(t.b(), t.a());
    }

    fn get_vertexs(&self, ind: &Indices) -> [&Vertex; 3] {
        [
            self.vertex_buf.get(&ind.0).unwrap(),
            self.vertex_buf.get(&ind.1).unwrap(),
            self.vertex_buf.get(&ind.2).unwrap(),
        ]
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (self.height - 1 - (y as f32).round() as usize) * self.width + x
    }
}
