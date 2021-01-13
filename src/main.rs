use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};

fn encrypt() {
    let secret_message = "xD\0";
    let mut input = image::open("res/input.png").unwrap().into_rgb8();
    let mut output: RgbImage = ImageBuffer::new(input.dimensions().0, input.dimensions().1);
    let secret_vector: Vec<u8> = secret_message.chars().map(|x| x as u8).collect();
    let number_of_pixels = input.dimensions().0 * input.dimensions().1;
    if secret_vector.len() as u32 * 3 > number_of_pixels {
        panic!("Image to small to fit message.");
    }
    let mut pxi: i32 = 0; // Index of pixel
    for c in secret_vector {
        let p11;
        let p22;
        let p33;
        {
            let mut p1 = input
                .get_pixel_mut(
                    pxi as u32 % input.dimensions().0,
                    pxi as u32 / input.dimensions().0,
                )
                .clone();
            if c & (1 << 0) == 1 {
                p1.0[0] |= 0b00000001;
            } else {
                p1.0[0] &= 0b11111110;
            }
            if c & (1 << 1) == 1 {
                p1.0[1] |= 0b00000001;
            } else {
                p1.0[1] &= 0b11111110;
            }
            if c & (1 << 2) == 1 {
                p1.0[2] |= 0b00000001;
            } else {
                p1.0[2] &= 0b11111110;
            }
            // p1.0 = [255u8, 254u8, 254u8, 254u8];
            p11 = p1;
        }
        {
            let mut p2 = input
                .get_pixel_mut(
                    (pxi + 1) as u32 % input.dimensions().0,
                    (pxi + 1) as u32 / input.dimensions().0,
                )
                .clone();
            if c & (1 << 3) == 1 {
                p2.0[0] |= 0b00000001;
            } else {
                p2.0[0] &= 0b11111110;
            }
            if c & (1 << 4) == 1 {
                p2.0[1] |= 0b00000001;
            } else {
                p2.0[1] &= 0b11111110;
            }
            if c & (1 << 5) == 1 {
                p2.0[2] |= 0b00000001;
            } else {
                p2.0[2] &= 0b11111110;
            }
            // p2.0 = [254u8, 254u8, 254u8, 254u8];
            p22 = p2;
        }
        {
            let mut p3 = input
                .get_pixel_mut(
                    (pxi + 2) as u32 % input.dimensions().0,
                    (pxi + 2) as u32 / input.dimensions().0,
                )
                .clone();
            if c & (1 << 6) == 1 {
                p3.0[0] |= 0b00000001;
            } else {
                p3.0[0] &= 0b11111110;
            }
            if c & (1 << 7) == 1 {
                p3.0[1] |= 0b00000001;
            } else {
                p3.0[1] &= 0b11111110;
            }
            p33 = p3;
        }
        output.put_pixel(
            pxi as u32 % input.dimensions().0,
            pxi as u32 / input.dimensions().0,
            p11,
        );
        output.put_pixel(
            (pxi + 1) as u32 % input.dimensions().0,
            (pxi + 1) as u32 / input.dimensions().0,
            p22,
        );
        output.put_pixel(
            (pxi + 2) as u32 % input.dimensions().0,
            (pxi + 2) as u32 / input.dimensions().0,
            p33,
        );
        pxi += 3;
    }
    loop {
        if (pxi as u32) < number_of_pixels {
            let px = input
                .get_pixel_mut(
                    pxi as u32 % input.dimensions().0,
                    pxi as u32 / input.dimensions().0,
                )
                .clone();
            output.put_pixel(
                pxi as u32 % input.dimensions().0,
                pxi as u32 / input.dimensions().0,
                px,
            );
            pxi += 1;
        } else {
            break;
        }
    }
    output
        .save_with_format("res/output.png", ImageFormat::Png)
        .unwrap();
}

fn decrypt() {
    let mut secret_message = "".to_owned();
    let input = image::open("res/output.png").unwrap().into_rgb8();
    let mut pxi = 0; // Index of pixel
    loop {
        let mut c: u8 = 0b00000000;
        let p1 = input.get_pixel(
            pxi as u32 % input.dimensions().0,
            pxi as u32 / input.dimensions().0,
        );
        let p2 = input.get_pixel(
            pxi + 1 as u32 % input.dimensions().0,
            pxi + 1 as u32 / input.dimensions().0,
        );
        let p3 = input.get_pixel(
            pxi + 2 as u32 % input.dimensions().0,
            pxi + 2 as u32 / input.dimensions().0,
        );
        if p1.0[0] & (1 << 0) == 1 {
            c |= 0b00000001;
        }
        if p1.0[1] & (1 << 0) == 1 {
            c |= 0b00000010;
        }
        if p1.0[2] & (1 << 0) == 1 {
            c |= 0b00000100;
        }
        if p2.0[0] & (1 << 0) == 1 {
            c |= 0b00001000;
        }
        if p2.0[1] & (1 << 0) == 1 {
            c |= 0b00010000;
        }
        if p2.0[2] & (1 << 0) == 1 {
            c |= 0b00100000;
        }
        if p3.0[0] & (1 << 0) == 1 {
            c |= 0b01000000;
        }
        if p3.0[1] & (1 << 0) == 1 {
            c |= 0b10000000;
        }
        println!("{:?}", c as char);
        if c as char == '\0' {
            break;
        } else {
            secret_message = secret_message + &(c as char).to_string();
            pxi += 3;
        }
        println!("{:?}", secret_message);
    }
}

fn main() {
    encrypt();
    decrypt();
}
