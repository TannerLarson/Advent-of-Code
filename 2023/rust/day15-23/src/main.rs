use std::{collections::HashMap, str::FromStr};

fn main() {
    let input: Vec<String> = include_str!("input.txt")
        .split(',')
        .map(|s| s.to_string().replace('\n', ""))
        .collect();

    let part_1: u32 = input.iter().map(|s| hash(s) as u32).sum();
    println!("Part 1: {}", part_1);

    let mut boxes = Boxes(HashMap::from_iter((0..=255_u8).map(|i| (i, Vec::new()))));
    for instruction in input {
        boxes.execute_instruction(&instruction)
    }
    println!("Part 2: {}", boxes.focusing_power())
}

fn hash(string: &str) -> u8 {
    let mut current_value = 0_u32;
    for c in string.chars() {
        current_value += (c as u8) as u32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value as u8
}

struct Lens {
    label: String,
    focal_length: u8,
}

impl Lens {
    fn box_number(&self) -> u8 {
        hash(&self.label)
    }
}

struct Boxes(HashMap<u8, Vec<Lens>>);

impl Boxes {
    fn execute_instruction(&mut self, s: &str) {
        let last_char = s.chars().last().unwrap();
        if last_char.is_numeric() {
            let lens = Lens {
                label: s.split('=').next().unwrap().to_string(),
                focal_length: last_char.to_digit(10).unwrap() as u8,
            };
            self.add_lens(lens)
        } else {
            self.remove_lens(Lens {
                label: s.split('-').next().unwrap().to_string(),
                focal_length: 0,
            })
        }
    }

    fn remove_lens(&mut self, lens: Lens) {
        let my_box = self.0.get_mut(&lens.box_number()).unwrap();
        if let Some(i_lens) = my_box.iter().position(|l| l.label == lens.label) {
            my_box.remove(i_lens);
        }
    }

    fn add_lens(&mut self, lens: Lens) {
        let my_box = self.0.get_mut(&lens.box_number()).unwrap();
        if let Some(i_lens) = my_box.iter().position(|l| l.label == lens.label) {
            my_box[i_lens] = lens
        } else {
            my_box.push(lens)
        }
    }

    fn focusing_power(&self) -> u32 {
        self.0
            .iter()
            .map(|(box_number, lenses)| {
                (*box_number as u32 + 1)
                    * lenses
                        .iter()
                        .enumerate()
                        .map(|(i, lens)| (i + 1) as u32 * lens.focal_length as u32)
                        .sum::<u32>()
            })
            .sum()
    }
}
