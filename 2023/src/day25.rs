use std::collections::HashMap;

use ndarray::Array2;

use crate::AocDay;

// Stoer–Wagner algorithm
fn find_global_min_cut(mut edges: Array2<i32>) -> usize {
    let mut best: (i32, Vec<usize>) = (i32::MAX, vec![]);
    let n = edges.ncols();
    let mut group: Vec<Vec<usize>> = (0..n).map(|i| vec![i]).collect();

    for phase in 1..n {
        let mut w: Vec<i32> = edges.row(0).clone().to_vec();
        let mut s = 0;
        let mut t = 0;
        for _ in 0..(n - phase) {
            w[t] = i32::MIN;
            s = t;
            t = w
                .iter()
                .enumerate()
                .max_by_key(|(_, v)| *v)
                .map(|(index, _)| index)
                .unwrap();

            for i in 0..n {
                w[i] += edges[(t, i)];
            }
        }
        best = best.min((w[t] - edges[(t, t)], group[t].clone()));
        for x in group[t].clone() {
            group[s].push(x);
        }
        for i in 0..n {
            edges[(s, i)] += edges[(t, i)];
            edges[(i, s)] = edges[(s, i)];
        }
        edges[(0, t)] = i32::MIN;
    }

    assert_eq!(best.0, 3);
    best.1.len()
}

pub struct AocDay25 {
    edges: Array2<i32>,
}

impl AocDay<usize, String> for AocDay25 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut i = 0;
        let mut conversion: HashMap<String, usize> = HashMap::new();
        let mut edges_list: Vec<(usize, usize)> = Vec::new();

        for line in lines {
            let (from, to_list) = line.split_once(": ").unwrap();
            let from_i = *conversion.entry(from.to_string()).or_insert_with(|| {
                i += 1;
                i - 1
            });
            for to in to_list.split_whitespace() {
                let to_i = *conversion.entry(to.to_string()).or_insert_with(|| {
                    i += 1;
                    i - 1
                });
                edges_list.push((from_i, to_i));
            }
        }

        let mut edges = Array2::zeros((i, i));

        for edge in edges_list {
            edges[(edge.0, edge.1)] = 1;
            edges[(edge.1, edge.0)] = 1;
        }

        AocDay25 { edges }
    }
    fn part1(&self) -> usize {
        let group_size = find_global_min_cut(self.edges.clone());
        group_size * (self.edges.ncols() - group_size)
    }
    fn part2(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod day25tests {
    use super::*;

    const INPUT: &[&str] = &[
        "jqt: rhn xhk nvd",
        "rsh: frs pzl lsr",
        "xhk: hfx",
        "cmg: qnr nvd lhk bvb",
        "rhn: xhk bvb hfx",
        "bvb: xhk hfx",
        "pzl: lsr hfx nvd",
        "qnr: nvd",
        "ntq: jqt hfx bvb xhk",
        "nvd: lhk",
        "lsr: lhk",
        "rzs: qnr cmg lsr rsh",
        "frs: qnr lhk lsr",
    ];

    #[test]
    fn part1() {
        let day = AocDay25::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 54);
    }
}
