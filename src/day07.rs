use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|v| v.to_string()).collect()
}

pub struct FileSystem {
    root: Folder,
    cwd: Vec<String>,
}

impl<'a> FileSystem {
    pub fn new() -> FileSystem{
        FileSystem{root: Folder::new(), cwd: vec!["/".to_string()]}
    }

    pub fn cd(&mut self, path: &str) {
        match path {
            ".." => _ = self.cwd.pop(),
            "/" => self.cwd = vec!["/".to_string()],
            _ => self.cwd.push(path.to_string()) 
        }
    }
    pub fn traverse_path<'p>(&mut self, path: &'p [String]) -> (&mut Folder, &'p [String]) {
        self.root.traverse_path(path)
    }
}

pub struct Folder {
    files: Vec<(i32, String)>,
    folders: HashMap<String, Folder>,
}

impl Folder {
    pub fn new() -> Folder {
        Folder {files: Vec::<(i32, String)>::new(), folders: HashMap::new()}
    }


    pub fn traverse_path<'p>(&mut self, mut path: &'p [String]) -> (&mut Folder, &'p [String]){
        if self.folders.contains_key(&path[0]){
            self.folders[path[0]].traverse_path(path[1..])
        } else {
            (self, path)
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &Vec<String>) -> i32 {
    let fs: FileSystem = FileSystem::new();
    1
}

//#[aoc(day7, part2)]
//pub fn part2(input: &Vec<&str>) -> i32 {
//    todo!();
//}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &'static str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 95437);
    }

    //#[test]
    //fn example_part2() {
    //    assert_eq!(part2(&input_generator(EXAMPLE)), 12);
    //}
}
