use std::collections::LinkedList;

use crate::AocDay;

pub struct AocDay06 {
    datastream: String,
}

struct StartOfPacketMarker {
    size: usize,
    buffer: LinkedList<u8>,
}

impl StartOfPacketMarker {
    fn new(size: usize) -> Self {
        StartOfPacketMarker {
            size,
            buffer: LinkedList::new(),
        }
    }

    fn push(&mut self, entry: u8) {
        if self.buffer.len() == self.size {
            self.buffer.pop_back();
        }
        self.buffer.push_front(entry);
    }

    fn is_valid(&self) -> bool {
        let mut v: Vec<u8> = self.buffer.iter().copied().collect();

        v.sort();
        v.dedup();

        v.len() == self.size
    }
}

fn find_start_of_packet_start_index(size: usize, datastream: &String) -> usize {
    let mut start_of_packet = StartOfPacketMarker::new(size);

    for (i, c) in datastream.as_bytes().iter().enumerate() {
        start_of_packet.push(*c);
        if start_of_packet.is_valid() {
            return i + 1;
        }
    }

    0
}

impl AocDay<usize, usize> for AocDay06 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Self {
        let datastream = lines.next().expect("input must have at least a line");
        AocDay06 { datastream }
    }
    fn part1(&self) -> usize {
        find_start_of_packet_start_index(4, &self.datastream)
    }
    fn part2(&self) -> usize {
        find_start_of_packet_start_index(14, &self.datastream)
    }
}

#[cfg(test)]
mod day06tests {
    use super::*;

    const INPUT1: &[&str] = &["mjqjpqmgbljsphdztnvjfqwrcgsmlb"];
    const INPUT2: &[&str] = &["bvwbjplbgvbhsrlpgdmjqwftvncz"];
    const INPUT3: &[&str] = &["nppdvjthqldpwncqszvftbrmjlhg"];
    const INPUT4: &[&str] = &["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"];
    const INPUT5: &[&str] = &["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"];

    #[test]
    fn part1_input1() {
        let day = AocDay06::preprocessing(INPUT1.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 7);
    }

    #[test]
    fn part1_input2() {
        let day = AocDay06::preprocessing(INPUT2.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 5);
    }

    #[test]
    fn part1_input3() {
        let day = AocDay06::preprocessing(INPUT3.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 6);
    }

    #[test]
    fn part1_input4() {
        let day = AocDay06::preprocessing(INPUT4.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 10);
    }

    #[test]
    fn part1_input5() {
        let day = AocDay06::preprocessing(INPUT5.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 11);
    }

    #[test]
    fn part2_input1() {
        let day = AocDay06::preprocessing(INPUT1.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 19);
    }

    #[test]
    fn part2_input2() {
        let day = AocDay06::preprocessing(INPUT2.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 23);
    }

    #[test]
    fn part2_input3() {
        let day = AocDay06::preprocessing(INPUT3.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 23);
    }

    #[test]
    fn part2_input4() {
        let day = AocDay06::preprocessing(INPUT4.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 29);
    }

    #[test]
    fn part2_input5() {
        let day = AocDay06::preprocessing(INPUT5.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 26);
    }
}
