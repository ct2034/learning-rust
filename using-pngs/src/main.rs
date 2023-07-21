use image::Pixel;
use std::env;
use image::ImageBuffer;
use image::Rgb;


fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let fname: &String = &args[1];
    println!("fname: {}", fname);

    // Read png
    let source_image: ImageBuffer<Rgb<u8>, Vec<u8>> = image::open(fname)
        .expect("failed to read image")
        .to_rgb8();
    let width = source_image.width();
    let height = source_image.height();

    // Increase contrast
    let mut contrast_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let pixel = source_image.get_pixel(x, y);
            let mut pixel_out = [0, 0, 0];
            for i in 0..3 {
                let mut val = pixel[i] as f32 / 255.0;
                val = - (val - 1.0).powf(2.0) + 1.0;
                pixel_out[i] = (val * 255.0) as u8;
            }
            contrast_image.put_pixel(x, y, Rgb(pixel_out));
        }
    }

    // Write contrast image
    contrast_image.save("contrast.png").unwrap();

    // Increase contrast differently
    let mut contrast2_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let pixel = source_image.get_pixel(x, y);
            let mut pixel_out = [0, 0, 0];
            for i in 0..3 {
                let mut val = pixel[i] as f32 / 255.0;
                val = val.powf(8.0);
                pixel_out[i] = (val * 255.0) as u8;
            }
            contrast2_image.put_pixel(x, y, Rgb(pixel_out));
        }
    }

    // Write contrast image
    contrast2_image.save("contrast2.png").unwrap();

    // Invert colors
    let mut inverted_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let pixel = source_image.get_pixel(x, y);
            let mut pixel_out = [0, 0, 0];
            for i in 0..3 {
                pixel_out[i] = ((
                    pixel[(1 + i) % 3] as f32 +
                    pixel[(2 + i) % 3] as f32
                ) / 2.0) as u8;
                if pixel_out[i] > 255 {
                    pixel_out[i] = 255;
                }
            }
            inverted_image.put_pixel(x, y, Rgb(pixel_out));
        }
    }

    // Write contrast image
    inverted_image.save("inverted.png").unwrap();


}