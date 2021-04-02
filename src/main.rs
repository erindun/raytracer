fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r: f32 = (i as f32 / (IMAGE_WIDTH - 1) as f32) * 255.999;
            let g: f32 = (j as f32 / (IMAGE_HEIGHT - 1) as f32) * 255.999;
            let b: f32 = 0.25 * 255.999;

            println!("{} {} {}", r as i32, g as i32, b as i32);
        }
    }
}
