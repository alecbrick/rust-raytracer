use log::info;

mod color;
mod vec3;

fn main() {
    env_logger::init();
    info!("Starting up");

    let image_width: i32 = 256;
    let image_height: i32 = 256;

    println!("P3\n{image_width} {image_height}\n255");
    for j in 0..image_height {
        let lines_remaining = image_height - j;
        info!("\nScanlines remaining: {lines_remaining}");
        for i in 0..image_width {
            let r = (i as f32) / ((image_width as f32) - 1.0);
            let g = (j as f32) / ((image_height as f32) - 1.0);
            let b: f32 = 0.0;

            let pixel = color::Color::new(r, g, b);
            color::write_color(&pixel);
        }
    }
}
