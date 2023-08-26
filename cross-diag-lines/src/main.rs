use std::error::Error;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    let mut r = rand_chacha::ChaCha8Rng::seed_from_u64(100);
    let size = 500;
    let block_offset = 5;
    let mut img = image::RgbaImage::new(size, size);
    let max_tile_columns = size / block_offset;
    let max_tile_rows = size / block_offset;
    for row_tile in 0..max_tile_rows {
        for tile_column in 0..max_tile_columns {
            let block = get_block(&mut r);
            for (row_num, row) in block.iter().enumerate() {
                for (column_num, value) in row.iter().enumerate() {
                    let mut pixel_value = image::Pixel::from_slice(&[255, 255, 255, 255]);
                    let x = column_num as u32;
                    let x = x + ((tile_column % max_tile_columns) * block_offset);
                    let y = row_num as u32;
                    let y = y + ((row_tile % max_tile_columns) * block_offset);
                    if *value == 0 {
                        pixel_value = image::Pixel::from_slice(&[0, 0, 0, 255]);
                    }
                    img.put_pixel(x, y, *pixel_value)
                }
            }
        }
    }
    let img_path = cwd.join("example.png");
    img.save(img_path);
    Ok(())
}

fn get_block(random: &mut ChaCha8Rng) -> Vec<Vec<u8>> {
    match random.gen_range(0..4) {
        0 => {
            vec![
                vec![0, 1, 1, 1, 1],
                vec![1, 0, 1, 1, 1],
                vec![1, 1, 0, 1, 1],
                vec![1, 1, 1, 0, 1],
                vec![1, 1, 1, 1, 0],
            ]
        }
        1 | 2 => {
            vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 1, 0, 1],
                vec![1, 1, 0, 1, 1],
                vec![1, 0, 1, 0, 1],
                vec![0, 1, 1, 1, 0],
            ]
        }
        _ => {
            vec![
                vec![1, 1, 1, 1, 0],
                vec![1, 1, 1, 0, 1],
                vec![1, 1, 0, 1, 1],
                vec![1, 0, 1, 1, 1],
                vec![0, 1, 1, 1, 1],
            ]
        }
    }
}
