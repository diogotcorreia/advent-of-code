use crate::AocDay;

fn hash(s: &str) -> usize {
    s.bytes()
        .fold(0, |acc, b| ((acc + usize::from(b)) * 17) % 256)
}

pub struct AocDay15 {
    steps: Vec<String>,
}

impl AocDay<usize, usize> for AocDay15 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Self {
        let line = lines.next().expect("input must contain at least one line");
        let steps = line.trim().split(',').map(String::from).collect();

        AocDay15 { steps }
    }
    fn part1(&self) -> usize {
        self.steps.iter().map(|s| hash(s)).sum()
    }
    fn part2(&self) -> usize {
        let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];

        for step in &self.steps {
            let (label, value) = step
                .split_once(['-', '='])
                .expect("failed to find action for step");

            let box_i = hash(label);
            let lens_box = boxes.get_mut(box_i).expect("box out of range");

            if step.contains('-') {
                lens_box.retain(|(l, _)| l != label);
            } else {
                let value: usize = value.parse().expect("failed to parse focal length of lens");
                if !lens_box
                    .iter_mut()
                    .filter(|(l, _)| l == label)
                    .fold(false, |_, (_, v)| {
                        *v = value;
                        true // know if any value was changed
                    })
                {
                    lens_box.push((label.to_string(), value));
                }
            }
        }

        boxes
            .iter()
            .enumerate()
            .flat_map(|(i, lens_box)| {
                lens_box
                    .iter()
                    .enumerate()
                    .map(move |(j, (_, focal_len))| (i + 1) * (j + 1) * focal_len)
            })
            .sum()
    }
}

#[cfg(test)]
mod day15tests {
    use super::*;

    const INPUT: &[&str] = &["rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"];

    #[test]
    fn part1() {
        let day = AocDay15::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 1320);
    }

    #[test]
    fn part2() {
        let day = AocDay15::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 145);
    }
}
