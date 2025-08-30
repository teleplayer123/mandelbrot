use image::{ImageBuffer, RgbImage};
use num_complex::Complex;
use std::time::{Duration, Instant};

fn mandelbrot(c: Complex<f64>, max_iter: u32) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..max_iter {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iter
}

fn render_mandelbrot(
    width: u32,
    height: u32,
    center: Complex<f64>,
    scale: f64,
    max_iter: u32,
) -> RgbImage {
    let mut img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let cx = center.re + (x as f64 - width as f64 / 2.0) * scale;
        let cy = center.im + (y as f64 - height as f64 / 2.0) * scale;
        let c = Complex::new(cx, cy);

        let iter = mandelbrot(c, max_iter);
        let color = if iter == max_iter {
            [0, 0, 0] // Black for points inside the set
        } else {
            let hue = (iter as f64 / max_iter as f64) * 360.0;
            let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);
            [r, g, b]
        };
        *pixel = image::Rgb(color);
    }
    img
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    if s == 0.0 {
        let val = (v * 255.0) as u8;
        return (val, val, val);
    }

    let h_i = (h / 60.0).floor() as i32;
    let f = h / 60.0 - h_i as f64;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    let (r, g, b) = match h_i {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => (0.0, 0.0, 0.0), // Should not happen
    };

    (
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
    )
}

fn main() {
    let width = 800;
    let height = 600;
    let max_iter = 256;
    let mut scale = 4.0;
    let mut center = Complex::new(-0.5, 0.0);
    let zoom_speed = 0.98;
    let move_speed = 0.001; // Adjust for movement speed

    let start_time = Instant::now();
    let frame_duration = Duration::from_millis(33); // Approximately 30 FPS
    for frame in 0..300 { // Render 300 frames
        println!("Processing frame: {}", frame);
        let img = render_mandelbrot(width, height, center, scale, max_iter);
        img.save(format!("mandelbrot_{}.png", frame)).unwrap();

        scale *= zoom_speed;
        center += Complex::new(move_speed*frame as f64 * (frame as f64).cos(), move_speed*frame as f64 * (frame as f64).sin()); //Spiral move.
        let elapsed = start_time.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    println!("Animation frames rendered.");
}
