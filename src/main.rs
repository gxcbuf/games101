#![allow(unused)]

use anyhow::Result;
use glam::{Mat4, Vec3};
use opencv::{highgui, prelude::*};

use rasterizer::{Buffer, Indices, Primitive, Rasterizer, Vertex};

mod rasterizer;
mod triangle;

pub const PI: f32 = 3.1415926;

fn get_model_matrix(angle: f32) -> Mat4 {
    let mut model = Mat4::IDENTITY;

    // 计算旋转角
    let angle = angle * PI / 180.0;
    let cosa = angle.cos();
    let sina = angle.sin();

    // 旋转矩阵, 由于是列式，所以需要转置
    let rotation = Mat4::from_cols_array(&[
        cosa, -sina, 0.0, 0.0, //
        sina, cosa, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0, //
    ])
    .transpose();

    model = rotation * model;
    model
}

fn get_view_matrix(eye_pos: Vec3) -> Mat4 {
    let mut view = Mat4::IDENTITY;

    // 平移到原点
    let translate = Mat4::from_cols_array(&[
        1.0, 0.0, 0.0, -eye_pos.x, //
        0.0, 1.0, 0.0, -eye_pos.y, //
        0.0, 0.0, 1.0, -eye_pos.z, //
        0.0, 0.0, 0.0, 1.0,
    ])
    .transpose();

    view = translate * view;
    view
}

/// 透视变换
/// eye_fov 视场角
/// aspec_ratio 长宽比
/// z_near 视锥的近平面
/// z_far 视锥的远平面
fn get_projection_matrix(eye_fov: f32, aspec_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
    let mut projection = Mat4::IDENTITY;

    // 根据 n, f, fov, aspect 求出 l, r, b, t;
    // 由于我们是朝着-Z方向看，所以需要n,f转化成负的
    let n = -z_near;
    let f = -z_far;
    let t = (eye_fov * PI / 180.0 / 2.0).tan() * n.abs();
    let r = t * aspec_ratio;
    let l = -r;
    let b = -t;

    let presp2ortho = Mat4::from_cols_array(&[
        n,
        0.0,
        0.0,
        0.0,
        0.0,
        n,
        0.0,
        0.0,
        0.0,
        0.0,
        n + f,
        -n * f,
        0.0,
        0.0,
        1.0,
        0.0,
    ])
    .transpose();

    let translate = Mat4::from_cols_array(&[
        1.0,
        0.0,
        0.0,
        -(r + l) / 2.0,
        0.0,
        1.0,
        0.0,
        -(t + b) / 2.0,
        0.0,
        0.0,
        1.0,
        -(n + f) / 2.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ])
    .transpose();

    let scale = Mat4::from_cols_array(&[
        2.0 / (r - l),
        0.0,
        0.0,
        0.0,
        0.0,
        2.0 / (t - b),
        0.0,
        0.0,
        0.0,
        0.0,
        2.0 / (n - f),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ])
    .transpose();

    projection = scale * translate * presp2ortho;
    projection
}

fn get_rotation(axis: Vec3, angle: f32) -> Mat4 {
    let angle = angle * PI / 180.0;
    // Mat4::from_axis_angle(axis, angle);
    let cosa = angle.cos();
    let sina = angle.sin();
    let [x, y, z] = axis.to_array();

    let matrix_i = Mat4::IDENTITY;

    let axis_n = Mat4::from_cols_array(&[
        x * x,
        x * y,
        x * z,
        0.0,
        y * x,
        y * y,
        y * z,
        0.0,
        z * x,
        z * y,
        z * z,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
    ])
    .transpose();

    let matrix_n = Mat4::from_cols_array(&[
        0.0, -z, y, 0.0, //
        z, 0.0, -x, 0.0, //
        -y, x, 0.0, 0.0, //
        0.0, 0.0, 0.0, 1.0, //
    ])
    .transpose();

    let mut rotation = cosa * matrix_i + (1.0 - cosa) * axis_n + sina * matrix_n;
    rotation.w_axis.w = 1.0;
    rotation
}

fn main() -> Result<()> {
    highgui::named_window("window", highgui::WINDOW_NORMAL)?;
    highgui::resize_window("window", 700, 700)?;

    // 定义700*700的光栅化
    let mut r = Rasterizer::new(700, 700);
    // 视口
    let eye_pos = Vec3::new(0.0, 0.0, 5.0);

    // 定义三角形
    let vertexs = vec![
        Vertex {
            coord: Vec3::new(2.0, 0.0, -2.0),
            color: Vec3::new(217.0, 238.0, 185.0),
        },
        Vertex {
            coord: Vec3::new(0.0, 2.0, -2.0),
            color: Vec3::new(217.0, 238.0, 185.0),
        },
        Vertex {
            coord: Vec3::new(-2.0, 0.0, -2.0),
            color: Vec3::new(217.0, 238.0, 185.0),
        },
        //
        Vertex {
            coord: Vec3::new(3.5, -1.0, -5.0),
            color: Vec3::new(185.0, 217.0, 238.0),
        },
        Vertex {
            coord: Vec3::new(2.5, 1.5, -5.0),
            color: Vec3::new(185.0, 217.0, 238.0),
        },
        Vertex {
            coord: Vec3::new(-1.0, 0.5, -5.0),
            color: Vec3::new(185.0, 217.0, 238.0),
        },
    ];
    let inds = vec![Indices::new(0, 1, 2), Indices::new(3, 4, 5)];

    r.insert_vertexs(&vertexs);
    r.insert_indices(&inds);

    let mut angle: f32 = 0.0;

    let mut key: i32 = 0;
    let axis = Vec3::new(0.0, 0.0, 1.0);
    // while key != 27 {
    r.clear(Buffer::All);

    // mvp变换
    r.set_mode(get_model_matrix(angle));
    r.set_view(get_view_matrix(eye_pos));
    r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));

    // 光栅化
    r.draw(&inds, Primitive::Triangle);

    // 获得光栅化后的数据
    let buf = r.frame_buf_data();

    // 设置opencv的数据
    let mut data = Mat::default();
    unsafe {
        data = Mat::new_rows_cols_with_data(
            700,
            700,
            opencv::core::CV_32FC3,
            buf.as_ptr() as *mut core::ffi::c_void,
            0,
        )?;
    };
    let mut frame = Mat::default();
    data.convert_to(&mut frame, opencv::core::CV_8UC3, 1.0, 1.0)?;

    // 展示数据
    highgui::imshow("window", &frame)?;

    key = highgui::wait_key(10)?;
    if key == b'a' as i32 {
        angle += 10.0;
    } else if key == b'd' as i32 {
        angle -= 10.0;
    }
    // }
    loop {}
    Ok(())
}
