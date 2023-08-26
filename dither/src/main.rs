use itertools::Itertools;
use std::error::Error;
use std::iter;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    let mut r = rand_chacha::ChaCha8Rng::seed_from_u64(1);
    let size = 1600;
    let block_offset = 16;
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
    match random.gen_range(0..21) {
        0 => [[0u8].into_iter().cycle().take(16).collect()]
            .into_iter()
            .cycle()
            .take(16)
            .collect(),
        1 => [
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8]
                .into_iter()
                .cycle()
                .take(16)
                .collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .take(16)
        .collect(),
        2 => vec![
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]
                .into_iter()
                .cycle()
                .take(16)
                .collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8]
                .into_iter()
                .cycle()
                .take(16)
                .collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]
                .into_iter()
                .cycle()
                .take(16)
                .collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8]
                .into_iter()
                .cycle()
                .take(16)
                .collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
        ],
        3 => vec![
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8, 0u8, 0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8, 0u8, 0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8, 0u8, 0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8, 0u8, 0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
        ],
        4 => [
            [0u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8, 0u8, 0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8, 0u8, 1u8, 0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        5 => [
            [0u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 1u8, 0u8]
                .into_iter()
                .cycle()
                .take(16)
                .collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8, 0u8, 1u8, 0u8].into_iter().cycle().take(16).collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8, 1u8, 0u8, 1u8, 0u8, 0u8, 0u8]
                .into_iter()
                .cycle()
                .take(16)
                .collect(),
            [0u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8, 0u8, 0u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        6 => [
            [0u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        7 => [
            [0u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8, 0u8, 0u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        8 => [
            [0u8, 1u8, 0u8, 0u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8].into_iter().cycle().take(16).collect(),
            [0u8, 0u8, 1u8, 0u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        9 => [
            [0u8, 1u8, 0u8, 0u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        10 => [
            [0u8, 1u8, 0u8, 0u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        11 => [[0u8, 1u8].into_iter().cycle().take(16).collect()]
            .into_iter()
            .cycle()
            .take(16)
            .collect(),
        12 => [
            [1u8, 1u8, 0u8, 1u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        13 => [
            [1u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8, 1u8, 1u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        14 => [
            [1u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        15 => [
            [1u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8, 0u8, 1u8, 1u8, 1u8, 0u8, 1u8]
                .into_iter()
                .cycle()
                .take(16)
                .collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8, 0u8, 0u8].into_iter().cycle().take(16).collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8, 1u8, 0u8, 1u8, 0u8, 1u8, 0u8, 1u8]
                .into_iter()
                .cycle()
                .take(16)
                .collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8, 0u8, 0u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        16 => [
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8, 1u8, 0u8, 1u8].into_iter().cycle().take(16).collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8, 0u8, 0u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        17 => [
            [1u8].into_iter().cycle().take(16).collect(),
            [0u8, 1u8, 1u8, 1u8].into_iter().cycle().take(16).collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        18 => [
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8, 1u8, 1u8, 1u8, 1u8, 0u8, 1u8, 1u8]
                .into_iter()
                .cycle()
                .take(16)
                .collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8]
                .into_iter()
                .cycle()
                .take(16)
                .collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        19 => [
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8]
                .into_iter()
                .cycle()
                .take(16)
                .collect(),
            [1u8].into_iter().cycle().take(16).collect(),
            [1u8].into_iter().cycle().take(16).collect(),
        ]
        .into_iter()
        .cycle()
        .take(16)
        .collect(),
        _ => [[1u8].into_iter().cycle().take(16).collect()]
            .into_iter()
            .cycle()
            .take(16)
            .collect(),
    }
}
