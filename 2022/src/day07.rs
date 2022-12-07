use crate::AocDay;

pub struct AocDay07 {
    root: INode,
}

#[derive(Debug)]
enum INode {
    File { size: usize },
    Directory { name: String, children: Vec<INode> },
}

impl INode {
    fn get_or_create_dir(&mut self, dir_name: &str) -> &mut INode {
        if let Self::Directory { children, .. } = self {
            let exists = children
                .iter_mut()
                .find(|x| match x {
                    Self::Directory { name, .. } => name == dir_name,
                    _ => false,
                })
                .is_some();

            if !exists {
                let new_dir = INode::Directory {
                    name: String::from(dir_name),
                    children: Vec::new(),
                };
                children.push(new_dir);
            }

            return children
                .iter_mut()
                .find(|x| match x {
                    Self::Directory { name, .. } => name == dir_name,
                    _ => false,
                })
                .unwrap();
        }

        unreachable!("should not be getting children on files, only on directories");
    }

    fn add_file(&mut self, file: INode) {
        if let Self::Directory { children, .. } = self {
            children.push(file);
            return;
        }

        unreachable!("can't add file to file");
    }

    fn get_directories(&self) -> Vec<&INode> {
        if let Self::Directory { children, .. } = self {
            return children
                .iter()
                .filter(|x| match x {
                    Self::Directory { .. } => true,
                    _ => false,
                })
                .collect();
        }

        unreachable!("can't get directories of file");
    }

    fn get_shallow_dir_size(&self) -> usize {
        if let Self::Directory { children, .. } = self {
            return children
                .iter()
                .filter_map(|x| match x {
                    Self::File { size, .. } => Some(size),
                    _ => None,
                })
                .sum();
        }

        unreachable!("can't get shallow size of file");
    }
}

fn preprocessing_recursive(
    curr_inode: &mut INode,
    mut lines: impl Iterator<Item = String>,
) -> impl Iterator<Item = String> {
    loop {
        match lines.next() {
            Some(line) => {
                let mut parts = line.split_ascii_whitespace();
                match parts.next().expect("invalid input") {
                    "$" => match parts.next().expect("invalid dollar command") {
                        "cd" => {
                            let path = parts.next().expect("cd missing path");
                            match path {
                                "/" => unreachable!("only the first line does this"),
                                ".." => {
                                    return lines;
                                }
                                _ => {
                                    let new_inode = curr_inode.get_or_create_dir(path);
                                    lines = preprocessing_recursive(new_inode, lines);
                                }
                            }
                        }
                        "ls" => {}
                        _ => unreachable!("not part of the problem input"),
                    },
                    "dir" => {} // ignore empty dirs, just create them on cd
                    size @ _ => {
                        // we can ignore file name, it's not used for anything
                        curr_inode.add_file(INode::File {
                            size: size.parse().expect("size not a number"),
                        });
                    }
                }
            }
            None => break,
        }
    }
    lines
}

// returns (directory_size, accumulated_size)
fn count_directories_size(inode: &INode, acc_size: usize) -> (usize, usize) {
    let mut dir_size = 0;
    let mut acc_size = acc_size;
    let children = inode.get_directories();

    for dir in children {
        let (child_dir_size, new_acc_size) = count_directories_size(dir, acc_size);
        acc_size = new_acc_size;
        dir_size += child_dir_size;
    }

    dir_size += inode.get_shallow_dir_size();

    if dir_size <= 100_000 {
        acc_size += dir_size;
    }

    (dir_size, acc_size)
}

// returns (directory_size, target_dir_size)
fn find_smallest_big_enough_size(
    inode: &INode,
    acc_size: usize,
    needed_size: usize,
) -> (usize, usize) {
    let mut dir_size = 0;
    let mut acc_size = acc_size;
    let children = inode.get_directories();

    for dir in children {
        let (child_dir_size, new_acc_size) =
            find_smallest_big_enough_size(dir, acc_size, needed_size);
        acc_size = new_acc_size;
        dir_size += child_dir_size;
    }

    dir_size += inode.get_shallow_dir_size();

    if dir_size >= needed_size && dir_size < acc_size {
        acc_size = dir_size;
    }

    (dir_size, acc_size)
}

impl AocDay<usize, usize> for AocDay07 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Self {
        let mut root = INode::Directory {
            name: String::new(),
            children: Vec::new(),
        };

        lines.next().unwrap(); // ignore $ cd /
        let mut lines = preprocessing_recursive(&mut root, lines);
        assert!(lines.next().is_none());

        return AocDay07 { root };
    }

    fn part1(&self) -> usize {
        let (_, total) = count_directories_size(&self.root, 0);

        total
    }
    fn part2(&self) -> usize {
        let (root_size, _) = count_directories_size(&self.root, 0);
        let needed_size = root_size - (70_000_000 - 30_000_000);
        let (_, total) = find_smallest_big_enough_size(&self.root, usize::MAX, needed_size);

        total
    }
}

#[cfg(test)]
mod day07tests {
    use super::*;

    const INPUT: &'static [&'static str] = &[
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k",
    ];

    #[test]
    fn part1() {
        let day = AocDay07::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 95437);
    }

    #[test]
    fn part2() {
        let day = AocDay07::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 24933642);
    }
}
