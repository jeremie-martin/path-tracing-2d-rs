extern crate image;
extern crate line_drawing;

use image::{GenericImage, GenericImageView, GrayImage, ImageBuffer};

use crate::utils::Point;
use line_drawing::*;
use ncollide2d::math::Matrix;

pub struct Screen {
    width: i32,
    height: i32,
    width_f64: f64,
    height_f64: f64,
    pixels: Vec<f64>,
}

impl Screen {
    pub fn new(width: i32, height: i32) -> Screen {
        Screen {
            width,
            height,
            width_f64: width as f64,
            height_f64: height as f64,
            pixels: vec![0.0; (width * height) as usize],
        }
    }

    pub fn draw_line(&mut self, start: &Point, end: &Point) {
        let a = (start.x * self.width_f64, start.y * self.height_f64);
        let b = (end.x * self.width_f64, end.y * self.height_f64);
        let dx = (a.0 - b.0);
        let dy = (a.1 - b.1);
        let bias = (dx * dx + dy * dy).sqrt() / (dx.abs().max(dy.abs()));

        for ((x, y), value) in XiaolinWu::<f64, i32>::new(a, b) {
            if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
                continue;
            }
            self.pixels[(x + self.width * y) as usize] += value * bias;
        }
        //
        // let a = (
        //     (start.x * self.width_f64) as i32,
        //     (start.y * self.height_f64) as i32,
        // );
        // let b = (
        //     (end.x * self.width_f64) as i32,
        //     (end.y * self.height_f64) as i32,
        // );
        //
        // for (x, y) in Bresenham::new(a, b) {
        //     if x < 0 || x >= self.width || y < 0 || y >= self.height {
        //         continue;
        //     }
        //     self.pixels[(x + self.width * y) as usize] += 1.0;
        // }
    }

    pub fn save_img(&self) {
        let contrast = 5.0;
        let brightness = 0.0;
        let gamma = 3.0;

        let pix = self.pixels.clone();

        let (A, B) = pix
            .iter()
            .cloned()
            .fold((f64::INFINITY, 0.0), |(min, max), x| {
                let a = if x < min { x } else { min };
                let b = if x > max { x } else { max };
                return (a, b);
            });

        let pix: Vec<f64> = pix.iter().map(|x| ((x - A) / (B - A))).collect();

        let pix: Vec<f64> = pix.iter().map(|x| x.powf(1.0 / gamma)).collect();
        let pix: Vec<f64> = pix.iter().map(|x| x * contrast).collect();
        let pix: Vec<f64> = pix.iter().map(|x| x + brightness).collect();
        println!("{} {}", A, B);
        let pix: Vec<u8> = pix
            .iter()
            .cloned()
            .map(|x| {
                let color = x * 255.0;
                if color * 255.0 < 0.0 {
                    0
                } else if color > 255.0 {
                    255
                } else {
                    color as u8
                }
            })
            .collect();

        let img: Option<GrayImage> =
            ImageBuffer::from_vec(self.width as u32, self.height as u32, pix);

        if let Some(i) = img {
            i.save("out.png");
        }
    }
}
