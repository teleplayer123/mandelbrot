use minifb::{Key, Window, WindowOptions};

// Screen dimensions
const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// Initial maximum number of iterations
const INITIAL_MAX_ITER: u32 = 1000;

// Struct to hold coordinates of a zoom target
struct ZoomTarget {
    x: f64,
    y: f64,
    // A description of the target, for optional display
    _description: &'static str,
}

// Predefined path of coordinates for zooming into mini-brots
const ZOOM_PATH: [ZoomTarget; 3] = [
    ZoomTarget { x: -0.743643887037151, y: 0.131825904205330, _description: "The antenna" },
    ZoomTarget { x: -0.1604, y: 1.0336, _description: "Upper spiral" },
    ZoomTarget { x: -0.1554, y: 1.0332, _description: "Another upper spiral" },
];

fn main() {
    // Initial view window coordinates in the complex plane
    let mut x_min = -2.5;
    let mut x_max = 1.0;
    let mut y_min = -1.2;
    let mut y_max = 1.2;

    // Zoom speed and target index
    let zoom_speed = 0.70;
    let mut current_target_index = 0;
    let mut max_iter = INITIAL_MAX_ITER;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = match Window::new(
        "Mandelbrot Zoom - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ) {
        Ok(win) => win,
        Err(e) => {
            eprintln!("Unable to create window: {}", e);
            return;
        }
    };

    window.limit_update_rate(Some(std::time::Duration::from_micros(100)));

    // Main Animation loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_target = &ZOOM_PATH[current_target_index];
        let target_x = current_target.x;
        let target_y = current_target.y;

        // Mandelbrot calculation
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let c_re = x as f64 / WIDTH as f64 * (x_max - x_min) + x_min;
                let c_im = y as f64 / HEIGHT as f64 * (y_max - y_min) + y_min;

                let mut z_re = 0.0;
                let mut z_im = 0.0;

                let mut i = 0;
                while i < max_iter && z_re * z_re + z_im * z_im < 4.0 {
                    let z_re_temp = z_re * z_re - z_im * z_im + c_re;
                    z_im = 2.0 * z_re * z_im + c_im;
                    z_re = z_re_temp;
                    i += 1;
                }

                let color = if i == max_iter {
                    0x000000 // Black for points inside the set
                } else {
                    let hue = (i % 256) as u32;
                    let r = (hue * 3) % 255;
                    let g = (hue * 5) % 255;
                    let b = (hue * 7) % 255;
                    (r << 16) | (g << 8) | b
                };
                buffer[y * WIDTH + x] = color;
            }
        }

        // Update the window with the mandelbrot buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        // Zoom in on the target coordinates
        x_min = target_x - (target_x - x_min) * zoom_speed;
        x_max = target_x + (x_max - target_x) * zoom_speed;
        y_min = target_y - (target_y - y_min) * zoom_speed;
        y_max = target_y + (y_max - target_y) * zoom_speed;

        // Check if we've "arrived" at the current target and switch if needed
        let window_width = x_max - x_min;
        if window_width < 1e-5 {
            println!("Reached target: {}", current_target._description);
            if current_target_index < ZOOM_PATH.len() - 1 {
                current_target_index += 1;
                // Increase max iterations for more detail in the next zoom
                max_iter += 500;
            } else {
                current_target_index = 0; // Loop back to the first target
                max_iter = INITIAL_MAX_ITER; // Reset max iterations
                x_min = -2.5;
                x_max = 1.0;
                y_min = -1.2;
                y_max = 1.2;
            }
        }
    }
}