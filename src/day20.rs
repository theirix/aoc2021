use std::collections::HashSet;
use std::fmt;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(35, 3351);

/* Impl */

struct Image {
    // top-level are rows so it can be indexed as [row][col] (aka [y][x])
    points: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
struct Pattern {
    // Positions (0..511) where pixels are lit
    lit: HashSet<usize>,
}

impl fmt::Debug for Image {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Image {}x{}: {{\n", self.rows, self.cols)?;
        for r in 0..self.rows {
            write!(f, "   ")?;
            for c in 0..self.cols {
                write!(f, "{}", if self.points[r][c] { "#" } else { "." }).unwrap();
            }
            writeln!(f)?;
        }
        write!(f, "}}")
    }
}

fn read_pattern(line: &str) -> Pattern {
    assert!(line.len() == 512);
    Pattern {
        lit: line
            .chars()
            .enumerate()
            .filter(|(_idx, c)| *c == '#')
            .map(|(idx, _c)| idx)
            .collect::<HashSet<usize>>(),
    }
}

fn read_image(lines: &[String]) -> Image {
    let points = lines
        .iter()
        .map(|s| s.chars().map(|c| c == '#').collect())
        .collect::<Vec<Vec<bool>>>();
    let rows = points.len();
    let cols = points[0].len();
    Image { points, rows, cols }
}

fn get_point(image: &Image, row: isize, col: isize, background: bool) -> bool {
    if row >= 0 && row < image.rows as isize && col >= 0 && col < image.cols as isize {
        image.points[row as usize][col as usize]
    } else {
        background
    }
}

fn make_bit_pattern_inf(image: &Image, row: isize, col: isize, background: bool) -> [bool; 9] {
    [
        get_point(image, row - 1, col - 1, background),
        get_point(image, row - 1, col, background),
        get_point(image, row - 1, col + 1, background),
        get_point(image, row, col - 1, background),
        get_point(image, row, col, background),
        get_point(image, row, col + 1, background),
        get_point(image, row + 1, col - 1, background),
        get_point(image, row + 1, col, background),
        get_point(image, row + 1, col + 1, background),
    ]
}

fn decode_bit_pattern(bits: &[bool; 9]) -> usize {
    // Convert bit value to decimal
    bits.iter()
        .enumerate()
        .filter(|(_idx, bvalue)| **bvalue)
        .map(|(idx, _)| 2_usize.pow(bits.len() as u32 - 1 - idx as u32))
        .sum::<usize>()
}

fn remap_image(image: &Image, pattern: &Pattern, background: bool) -> Image {
    let mut new_image = Image {
        points: image.points.clone(),
        rows: image.rows,
        cols: image.cols,
    };
    for r in 0..image.rows {
        for c in 0..image.cols {
            let dec_value: usize = decode_bit_pattern(&make_bit_pattern_inf(
                image, r as isize, c as isize, background,
            ));
            let is_lit: bool = pattern.lit.contains(&dec_value);
            new_image.points[r][c] = is_lit;
        }
    }
    new_image
}

fn pad_image(image: &Image, pad: usize, background: bool) -> Image {
    let mut new_image = Image {
        points: Vec::new(),
        rows: image.rows + 2 * pad,
        cols: image.cols + 2 * pad,
    };
    // init zeroes
    for _ in 0..new_image.rows {
        new_image.points.push(vec![background; new_image.cols]);
    }

    // copy center
    for r in 0..image.rows {
        for c in 0..image.cols {
            new_image.points[r + pad][c + pad] = image.points[r][c];
        }
    }
    new_image
}

fn score(image: &Image) -> u64 {
    image
        .points
        .iter()
        .map(|row| row.iter().map(|v| if *v { 1 } else { 0 }).sum::<u64>())
        .sum::<u64>()
}

fn process_gen(lines: Vec<String>, iter_count: usize) -> u64 {
    let pattern = read_pattern(&lines[0]);
    let orig_image = read_image(&lines[2..]);
    //println!("{:?}", pattern);
    println!("{:?}", orig_image);
    // pad once for all iterations, one pixel for each iteration
    let mut image = pad_image(&orig_image, iter_count as usize, false);
    for iter in 0..iter_count {
        /* Reddit:
         * The trick was to check 0th bit of algo and if it is '#', and 511th bit is '.', then pixel outside the algo are going to toggle on every iteration.
         * If both of them are '.', or just 0th is '.' then toggle not needed.
         */
        let bk_value = if pattern.lit.contains(&0) && !pattern.lit.contains(&511) {
            // flipper
            iter % 2 == 1
        } else {
            // stable
            false
        };
        image = remap_image(&image, &pattern, bk_value);
    }
    score(&image)
}

pub fn process_a(lines: Vec<String>) -> u64 {
    process_gen(lines, 2)
}

pub fn process_b(lines: Vec<String>) -> u64 {
    process_gen(lines, 50)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = r"#..#.
#....
##..#
..#..
..###
.#..#";

    fn sample_image() -> Image {
        let lines: Vec<String> = SAMPLE.split("\n").map(|s| String::from(s)).collect();
        read_image(&lines[..])
    }

    #[test]
    fn test_read() {
        let image = sample_image();
        assert_eq!(image.rows, 6);
        assert_eq!(image.cols, 5);
    }

    #[test]
    fn test_pad() {
        let image = sample_image();
        let padded_image = pad_image(&image, 2, true);
        assert_eq!(image.cols + 4, padded_image.cols)
    }

    #[test]
    fn test_kernel_center() {
        let bits = make_bit_pattern_inf(&sample_image(), 2, 2, false);
        let value = decode_bit_pattern(&bits);
        assert_eq!(value, 34);
    }

    #[test]
    fn test_kernel_lpad() {
        // 10/010/011
        let bits = make_bit_pattern_inf(&sample_image(), 1, 0, false);
        assert_eq!(
            bits,
            [false, true, false, false, true, false, false, true, true]
        );
        let value = decode_bit_pattern(&bits);
        assert_eq!(value, 147);
    }

    #[test]
    fn test_kernel_trpad() {
        // 10/010/011
        let bits = make_bit_pattern_inf(&sample_image(), 0, 3, false);
        assert_eq!(
            bits,
            [false, false, false, false, true, false, false, false, false]
        );
        let value = decode_bit_pattern(&bits);
        assert_eq!(value, 16);
    }
}
