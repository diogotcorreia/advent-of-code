use std::{collections::BTreeSet, io::BufRead, ops::Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Image {
    lit: BTreeSet<Pos>,
    top_left: Pos,
    bottom_right: Pos,
    outer_state: bool, // the "infinite" area of the image can be both lit or not lit as well
}

impl Image {
    fn new() -> Image {
        Image {
            lit: BTreeSet::new(),
            top_left: Pos { x: 0, y: 0 },
            bottom_right: Pos { x: 0, y: 0 },
            outer_state: false,
        }
    }

    fn set_lit(&mut self, pos: &Pos) {
        if !self.lit.contains(pos) {
            self.lit.insert(pos.clone());
            if self.top_left.x > pos.x || self.top_left.y > pos.y {
                self.top_left = Pos {
                    x: self.top_left.x.min(pos.x),
                    y: self.top_left.y.min(pos.y),
                };
            }
            if self.bottom_right.x <= pos.x || self.bottom_right.y <= pos.y {
                self.bottom_right = Pos {
                    x: self.bottom_right.x.max(pos.x + 1),
                    y: self.bottom_right.y.max(pos.y + 1),
                };
            }
        }
    }

    fn is_lit(&self, pos: &Pos) -> bool {
        if pos.x < self.top_left.x
            || pos.y < self.top_left.y
            || pos.x >= self.bottom_right.x
            || pos.y >= self.bottom_right.y
        {
            self.outer_state
        } else {
            self.lit.contains(pos)
        }
    }

    fn get_x_range(&self) -> Range<i32> {
        (self.top_left.x - 1)..(self.bottom_right.x + 1)
    }

    fn get_y_range(&self) -> Range<i32> {
        (self.top_left.y - 1)..(self.bottom_right.y + 1)
    }
}

fn main() {
    let enhancement_algo = {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");
        let mut enhancement_algo = [false; 512];
        for (i, v) in input.trim().chars().enumerate() {
            match v {
                '.' => enhancement_algo[i] = false,
                '#' => enhancement_algo[i] = true,
                _ => unreachable!("invalid chars on image enhancement algorithm"),
            }
        }

        enhancement_algo
    };

    let mut image = {
        let mut image = Image::new();
        for (i, line) in std::io::stdin()
            .lock()
            .lines()
            .filter_map(|l| l.ok())
            .filter(|l| l.trim().len() > 0)
            .enumerate()
        {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '#' => image.set_lit(&Pos {
                        x: j as i32,
                        y: i as i32,
                    }),
                    _ => {}
                };
            }
        }
        image
    };

    for _ in 0..2 {
        image = enhance(&image, &enhancement_algo);
    }

    println!("Part 1: {}", image.lit.len());

    for _ in 2..50 {
        image = enhance(&image, &enhancement_algo);
    }

    println!("Part 2: {}", image.lit.len());
}

fn enhance(image: &Image, enhancement_algo: &[bool; 512]) -> Image {
    let mut new_image = Image::new();

    for y in image.get_y_range() {
        for x in image.get_x_range() {
            let mut new_pixel_index = 0;
            let mut bin_ptr: u16 = 1;
            for i in (-1..=1).rev() {
                for j in (-1..=1).rev() {
                    if image.is_lit(&Pos { x: x + j, y: y + i }) {
                        new_pixel_index |= bin_ptr;
                    }
                    bin_ptr <<= 1;
                }
            }
            if enhancement_algo[new_pixel_index as usize] {
                new_image.set_lit(&Pos { x, y });
            }
        }
    }

    if image.outer_state {
        new_image.outer_state = enhancement_algo[511];
    } else {
        new_image.outer_state = enhancement_algo[0];
    }

    new_image
}
