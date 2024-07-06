use image::{ImageBuffer, RgbImage};
use std::{env::args, fs::File, io::Read};
fn main()
{
    let args = args().collect::<Vec<String>>();
    if args.len() != 1
    {
        let input_file = args[1].clone();
        let output_file = args[2].clone();
        let mut file = File::open(input_file).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let w = ((buffer.len() / 4) as f64).sqrt() as u32;
        let mut img: RgbImage = ImageBuffer::new(w, w);
        for y in 0..w
        {
            for x in 0..w
            {
                let i = (y * w + x) as usize * 4;
                img.put_pixel(
                    x,
                    w - y - 1,
                    image::Rgb([buffer[i + 2], buffer[i + 1], buffer[i]]),
                );
            }
        }
        img.save(output_file).unwrap();
    }
}
