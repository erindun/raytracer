mod color;
use color::Color;

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let color = Color {
                r: (i as f32 / (IMAGE_WIDTH - 1) as f32),
                g: (j as f32 / (IMAGE_HEIGHT - 1) as f32),
                b: 0.25,
            };

            println!("{}", color);
        }
    }
}
