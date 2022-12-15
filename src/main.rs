use colored::{Color, Colorize};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
    thread, time::{self, Instant},
};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

fn main() {
    // println!("d1");
    // d1();
    // println!("d2");
    // d2();
    // println!("d3");
    // d3();
    // println!("d4");
    // d4();
    // println!("d5");
    // d5();
    // println!("d6");
    // d6();
    // println!("d7");
    // d7();
    // println!("d8");
    // d8();
    // println!("d9");
    // d9();
    // println!("d10");
    // d10();
    // println!("d11");
    // d11();
    println!("d12");
    d12();
    // println!("d13");
    // d13();
    // println!("d14");
    // d14();
}

fn read_input(file: &str) -> Vec<String> {
    let mut file = File::open(format!("input_tw/{}", file)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect::<Vec<_>>()
}

fn d1() {
    let lines = read_input("in_d1");
    let mut calories = Vec::from([0]);
    let mut max_calories = 0;
    let mut elf = 0;
    lines.iter().for_each(|a| {
        // let mut last_calories = calories.last_mut();
        if a.len() > 0 {
            if let Ok(calorie) = a.parse::<i32>() {
                calories.get_mut(elf).map(|c| *c += calorie);
            }
        } else {
            if let Some(calorie) = calories.get(elf) {
                if *calorie > max_calories {
                    max_calories = *calorie;
                }
            }
            elf += 1;
            calories.push(0);
        }
    });
    calories.sort();
    let last_three = calories
        .get(calories.len() - 3..=calories.len() - 1)
        .unwrap();
    println!("p1 {}", max_calories);
    println!("p2 {}", last_three.iter().sum::<i32>());
}
fn d2() {
    #[derive(Debug, Clone, Copy)]
    enum Hand {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }
    #[derive(Debug, Clone, Copy)]
    enum RoundResult {
        Loose = 0,
        Draw = 3,
        Win = 6,
    }
    let lines = read_input("in_d2");
    let mut score_sum = 0;
    let mut score_sum_p2 = 0;
    for line in lines {
        let mut hands = line.split_whitespace();
        let hand_opponent_str = hands.next().unwrap();
        let hand_elf_str = hands.next().unwrap();
        let hand_opponent = match hand_opponent_str {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("Invalid hand"),
        };
        let hand_elf = match hand_elf_str {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => panic!("Invalid hand"),
        };
        let hand_result = match hand_elf_str {
            "X" => RoundResult::Loose,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => panic!("Invalid hand"),
        };
        // p1
        let match_result_score = match (hand_opponent, hand_elf) {
            (Hand::Rock, Hand::Scissors)
            | (Hand::Scissors, Hand::Paper)
            | (Hand::Paper, Hand::Rock) => RoundResult::Loose,
            (Hand::Paper, Hand::Scissors)
            | (Hand::Rock, Hand::Paper)
            | (Hand::Scissors, Hand::Rock) => RoundResult::Win,
            (Hand::Rock, Hand::Rock)
            | (Hand::Paper, Hand::Paper)
            | (Hand::Scissors, Hand::Scissors) => RoundResult::Draw,
        };
        score_sum += hand_elf as u32 + match_result_score as u32;
        // p2
        let hand_elf = match (hand_opponent, hand_result) {
            (Hand::Rock, RoundResult::Loose) => Hand::Scissors,
            (Hand::Rock, RoundResult::Draw) => Hand::Rock,
            (Hand::Rock, RoundResult::Win) => Hand::Paper,
            (Hand::Paper, RoundResult::Loose) => Hand::Rock,
            (Hand::Paper, RoundResult::Draw) => Hand::Paper,
            (Hand::Paper, RoundResult::Win) => Hand::Scissors,
            (Hand::Scissors, RoundResult::Loose) => Hand::Paper,
            (Hand::Scissors, RoundResult::Draw) => Hand::Scissors,
            (Hand::Scissors, RoundResult::Win) => Hand::Rock,
        };
        score_sum_p2 += hand_elf as u32 + hand_result as u32;
    }
    println!("p1 {}", score_sum);
    println!("p2 {}", score_sum_p2);
}

fn d3() {
    let lines = read_input("in_d3");
    let mut score = 0;
    // println!("{}", 'A' as u32 - 96);
    let lines = lines
        .into_iter()
        .map(|c| {
            c.chars()
                .into_iter()
                .map(|c| match c {
                    'A'..='Z' => c as u32 - 64 + 26,
                    'a'..='z' => c as u32 - 96,
                    _ => panic!("Invalid char"),
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    for line in lines.clone() {
        if line.len() % 2 != 0 {
            panic!("Invalid line length");
        }
        let chars = line;
        let (item_1, item_2) = chars.split_at(chars.len() / 2);
        let duplicates = item_1
            .iter()
            .filter(|&i| item_2.contains(i))
            .collect::<Vec<&u32>>();

        score += **duplicates.get(0).unwrap();
    }
    println!("p1 {}", score);

    let mut score = 0;
    for elf_group in lines.chunks(3) {
        let duplicates = elf_group[0]
            .iter()
            .filter(|&i| elf_group[1].contains(i))
            .collect::<Vec<&u32>>();
        let mut triples = Vec::new();
        duplicates.iter().for_each(|&i| {
            if elf_group[2].contains(i) {
                triples.push(i);
            }
        });
        score += **triples.get(0).unwrap();
    }
    println!("p2 {}", score);
}

fn d4() {
    let lines = read_input("in_d4");
    let mut complete_overlaps = 0;
    let mut overlaps = 0;
    for line in lines {
        if let [first_range, second_range] = line.split(",").collect::<Vec<&str>>()[..] {
            if let [first_start, first_end] = first_range
                .split("-")
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()[..]
            {
                if let [second_start, second_end] = second_range
                    .split("-")
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()[..]
                {
                    if first_start <= second_start && first_end >= second_end {
                        complete_overlaps += 1;
                        overlaps += 1;
                    } else if second_start <= first_start && second_end >= first_end {
                        complete_overlaps += 1;
                        overlaps += 1;
                    } else {
                        if first_end >= second_start && first_end <= second_end {
                            overlaps += 1;
                        } else if second_end >= first_start && second_end <= first_end {
                            overlaps += 1;
                        }
                    }
                }
            }
        }
    }
    println!("p1 {}", complete_overlaps);
    println!("p2 {}", overlaps);
}

fn d5() {
    let lines = read_input("in_d5");
    let mut reading_crates = true;
    let mut crates = Vec::new();
    for line in lines.clone() {
        if line.is_empty() {
            reading_crates = false;
            continue;
        }
        if reading_crates {
            let blocks = line.chars().into_iter().collect::<Vec<char>>();
            if blocks[1] == '1' {
                continue;
            }
            let mut stack = 0;
            if crates.is_empty() {
                crates.resize(blocks.chunks(4).len(), VecDeque::new());
            }
            for block in blocks.chunks(4) {
                stack += 1;
                let crate_char = block.get(1).unwrap().to_string();
                let crate_char = crate_char.trim();
                if crate_char.is_empty() {
                    continue;
                }
                if let Some(count) = crates.get_mut(stack - 1) {
                    count.push_back(crate_char.to_owned());
                } else {
                    panic!("stack out of bounds");
                }
            }
        } else {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            if let [_, count, _, from, _, to] = split[..] {
                let from = from.parse::<usize>().unwrap();
                let to = to.parse::<usize>().unwrap();
                let count = count.parse::<usize>().unwrap();
                let from = crates.get_mut(from - 1).unwrap();
                let mut moved_crates = Vec::new();
                while moved_crates.len() < count {
                    let crate_char = from.pop_front().unwrap();
                    moved_crates.push(crate_char);
                }
                let to = crates.get_mut(to - 1).unwrap();
                for crate_char in moved_crates {
                    to.push_front(crate_char);
                }
            } else {
                panic!("invalid line");
            }
        }
    }
    let mut output = String::new();
    for stack in crates {
        output.push_str(stack.front().unwrap());
    }
    println!("p1 {}", output);
    reading_crates = true;
    let mut crates = Vec::new();
    for line in lines {
        if line.is_empty() {
            reading_crates = false;
            continue;
        }
        if reading_crates {
            let blocks = line.chars().into_iter().collect::<Vec<char>>();
            if blocks[1] == '1' {
                continue;
            }
            let mut stack = 0;
            if crates.is_empty() {
                crates.resize(blocks.chunks(4).len(), VecDeque::new());
            }
            for block in blocks.chunks(4) {
                stack += 1;
                let crate_char = block.get(1).unwrap().to_string();
                let crate_char = crate_char.trim();
                if crate_char.is_empty() {
                    continue;
                }
                if let Some(count) = crates.get_mut(stack - 1) {
                    count.push_back(crate_char.to_owned());
                } else {
                    panic!("stack out of bounds");
                }
            }
        } else {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            if let [_, count, _, from, _, to] = split[..] {
                let from = from.parse::<usize>().unwrap();
                let to = to.parse::<usize>().unwrap();
                let count = count.parse::<usize>().unwrap();
                let from = crates.get_mut(from - 1).unwrap();
                let mut moved_crates = Vec::new();
                while moved_crates.len() < count {
                    let crate_char = from.pop_front().unwrap();
                    moved_crates.push(crate_char);
                }
                let to = crates.get_mut(to - 1).unwrap();
                moved_crates.reverse();
                for crate_char in moved_crates {
                    to.push_front(crate_char);
                }
            } else {
                println!("{:?}, {}", split, line);
                panic!("invalid line");
            }
        }
    }
    let mut output = String::new();
    for stack in crates {
        output.push_str(stack.front().unwrap());
    }
    println!("p2 {}", output);
}

fn d6() {
    let lines = read_input("in_d6");
    check_buf(lines.clone());
    fn check_buf(lines: Vec<String>) {
        let mut buf = VecDeque::with_capacity(4);
        let mut msg = VecDeque::with_capacity(14);
        let mut marker = None;
        let mut message = None;
        for line in lines {
            for (index, c) in line.chars().enumerate() {
                buf.push_front(c.to_string());
                msg.push_front(c.to_string());
                buf.truncate(4);
                msg.truncate(14);
                // check if buf is has duplicates
                if buf.len() == 4 && marker.is_none() {
                    if !has_duplicates(&buf) {
                        marker = Some(index + 1);
                    }
                }
                // check if msg is has duplicates
                if msg.len() == 14 && message.is_none() {
                    if !has_duplicates(&msg) {
                        message = Some(index + 1);
                    }
                }
            }
        }
        println!("p1 {}", marker.unwrap_or_default());
        println!("p2 {}", message.unwrap_or_default());
    }

    fn has_duplicates(buf: &VecDeque<String>) -> bool {
        for i in 0..buf.len() {
            for j in 0..buf.len() {
                if i != j && buf[i] == buf[j] {
                    return true;
                }
            }
        }
        false
    }
}

fn d7() {
    let lines = read_input("in_d7");

    #[derive(Debug, Clone)]
    enum F {
        Older(Folder),
        Ile(usize),
    }

    impl F {
        fn size(&self) -> usize {
            match self {
                F::Older(folder) => folder.size(),
                F::Ile(size) => *size,
            }
        }
    }

    #[derive(Debug, Clone)]
    struct Folder {
        size: usize,
        children: HashMap<String, Box<F>>,
    }

    let mut file_system = Folder {
        size: 0,
        children: HashMap::new(),
    };

    impl Folder {
        fn size(&self) -> usize {
            let mut size = 0;
            for child in self.children.values() {
                size += child.size();
            }
            size
        }

        fn get_path_mut(&mut self, path: Vec<String>) -> Option<&mut Folder> {
            let mut current_folder = self;
            for directory in path {
                if directory.is_empty() {
                    continue;
                }
                if let Some(f) = current_folder.children.get_mut(&directory) {
                    match &mut **f {
                        F::Older(folder) => current_folder = folder,
                        F::Ile(_) => panic!("not a folder"),
                    };
                } else {
                    return None;
                }
            }
            Some(current_folder)
        }
    }

    let total_space = 70_000_000;
    let required_space = 30_000_000;

    let mut current_path = Vec::new();
    let mut small_dirs = HashMap::new();
    let mut all_dirs = HashMap::new();
    for line in lines {
        let command_line = line.split_whitespace().into_iter().collect::<Vec<_>>();
        if line.starts_with("$") {
            if let [_, command, ..] = command_line[..] {
                if command == "cd" {
                    let directory = command_line[2];
                    if directory == ".." {
                        current_path.pop();
                    } else if directory != "/" {
                        current_path.push(directory.to_string());
                    }
                }
            }
        } else {
            if let Some(&"dir") = command_line.first() {
                let dir = file_system.get_path_mut(current_path.clone()).unwrap();
                dir.children
                    .entry(command_line.last().unwrap().to_string())
                    .or_insert(Box::new(F::Older(Folder {
                        size: 0,
                        children: HashMap::new(),
                    })));
            } else {
                let size = command_line.first().unwrap().parse::<usize>().unwrap();
                let dir = file_system.get_path_mut(current_path.clone()).unwrap();
                dir.children.insert(
                    command_line.last().unwrap().to_string(),
                    Box::new(F::Ile(size)),
                );
            }
        }
        let mut dir = file_system.get_path_mut(current_path.clone()).unwrap();
        dir.size = dir.size();
        if !current_path.join("/").is_empty() {
            all_dirs.insert(current_path.join("/"), dir.size);
        }
        if dir.size <= 100_000 {
            small_dirs.insert(current_path.join("/"), dir.size);
        } else {
            small_dirs.remove(&current_path.join("/"));
        }
    }
    file_system.size = file_system.size();
    println!(
        "p1 {}",
        small_dirs.values().into_iter().map(|d| *d).sum::<usize>()
    );

    let dir = file_system.get_path_mut(current_path.clone()).unwrap();
    all_dirs.insert(current_path.join("/"), dir.size);
    all_dirs.insert("/".to_string(), file_system.size);
    let free_space = total_space - file_system.size;
    let space_to_free = required_space - free_space;
    let mut dir_sizes = all_dirs
        .values()
        .into_iter()
        .filter_map(|d| if d >= &space_to_free { Some(d) } else { None })
        .collect::<Vec<_>>();
    dir_sizes.sort();
    println!("p2 {}", dir_sizes.first().unwrap(),);
}

fn d8() {
    let lines = read_input("in_d8");
    let matrix_width = lines.first().unwrap().len();
    let matrix_height = lines.len();

    let mut matrix: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        matrix.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    struct Visible {
        top: u32,
        bottom: u32,
        left: u32,
        right: u32,
    }
    let comp: Vec<_> = (0..matrix_height)
        .into_par_iter()
        .map(|row_index| {
            (0..matrix_width)
                .into_iter()
                .map(|col_index| {
                    let value = matrix[row_index][col_index];
                    let mut visible = Visible {
                        top: 0,
                        bottom: 0,
                        left: 0,
                        right: 0,
                    };
                    let mut visibility = Visible {
                        top: 0,
                        bottom: 0,
                        left: 0,
                        right: 0,
                    };
                    // move top
                    for i in 1..=row_index {
                        visibility.top += 1 as u32;
                        if matrix[row_index - i][col_index] >= value {
                            break;
                        }
                        visible.top = i as u32;
                    }
                    // move bottom
                    for i in 1..matrix_height - row_index {
                        visibility.bottom += 1 as u32;
                        if matrix[row_index + i][col_index] >= value {
                            break;
                        }
                        visible.bottom = i as u32;
                    }
                    // move left
                    for i in 1..=col_index {
                        visibility.left += 1 as u32;
                        if matrix[row_index][col_index - i] >= value {
                            break;
                        }
                        visible.left = i as u32;
                    }
                    // move right
                    for i in 1..matrix_width - col_index {
                        visibility.right += 1 as u32;
                        if matrix[row_index][col_index + i] >= value {
                            break;
                        }
                        visible.right = i as u32;
                    }
                    (
                        if visible.right == (matrix_width - col_index - 1) as u32
                            || visible.left == col_index as u32
                            || visible.top == row_index as u32
                            || visible.bottom == (matrix_height - row_index - 1) as u32
                        {
                            1
                        } else {
                            0
                        },
                        visibility,
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();
    println!(
        "p1 {}",
        comp.iter().flatten().filter(|(v, _)| *v == 1).count()
    );
    let max_vis = comp
        .iter()
        .flatten()
        .map(|(_, v)| v)
        .max_by_key(|v| v.top * v.bottom * v.left * v.right)
        .unwrap();
    println!(
        "p2 {}",
        max_vis.top * max_vis.bottom * max_vis.left * max_vis.right
    );
}

fn d9() {
    let lines = read_input("in_d9_t");
    let start = (0, 0);
    let mut knots: [(i32, i32); 10] = [start; 10];
    let mut tail_visited = HashMap::new();

    for (i, knot) in knots.iter().enumerate() {
        let mut set = HashSet::new();
        set.insert(*knot);
        tail_visited.insert(i, set);
    }

    let _print = |tail_visited: HashMap<usize, HashSet<(i32, i32)>>, knots: [(i32, i32); 10]| {
        let max_x_visited = tail_visited
            .iter()
            .map(|(i, set)| set.iter().max_by_key(|m| m.0).unwrap().0)
            .max()
            .unwrap();
        let min_x_visited = tail_visited
            .iter()
            .map(|(i, set)| set.iter().min_by_key(|m| m.0).unwrap().0)
            .min()
            .unwrap();
        let max_y_visited = tail_visited
            .iter()
            .map(|(i, set)| set.iter().max_by_key(|m| m.1).unwrap().1)
            .max()
            .unwrap();
        let min_y_visited = tail_visited
            .iter()
            .map(|(i, set)| set.iter().min_by_key(|m| m.1).unwrap().1)
            .min()
            .unwrap();
        let mut iter: Vec<_> = tail_visited.iter().collect();
        iter.sort_by_key(|(i, _)| *i);
        format!(
            "{}",
            (min_x_visited - 10..=max_x_visited + 10)
                .rev()
                .into_iter()
                .map(|y| {
                    (min_y_visited - 10..=max_y_visited + 10)
                        .into_iter()
                        .map(|x| {
                            if knots.contains(&(x, y)) {
                                format!(
                                    "{}{}",
                                    knots
                                        .iter()
                                        .position(|k| *k == (x, y))
                                        .unwrap()
                                        .to_string()
                                        .bright_blue()
                                        .on_white()
                                        .bold(),
                                    " ".to_string().on_white()
                                )
                            } else {
                                format!(
                                    "{}",
                                    tail_visited
                                        .get(&9)
                                        .unwrap()
                                        .contains(&(x, y))
                                        .then(|| "  ".to_string().on_white())
                                        .unwrap_or_else(|| "  ".to_string().black())
                                )
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect::<Vec<String>>()
                .join(
                    "
"
                )
        )
    };

    let mut last_visited = Vec::new();

    for line in lines {
        if let [direction, movement] = line.split_whitespace().collect::<Vec<_>>()[..] {
            let movement = movement.parse::<i32>().unwrap();
            let _distance = |head: &(i32, i32), tail: &(i32, i32)| {
                (head.0 - tail.0).abs().max((head.1 - tail.1).abs())
            };
            let _move = |head: &mut (i32, i32)| match direction {
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                _ => panic!("unknown direction"),
            };
            println!(
                "{} {}",
                direction.to_string().cyan().bold(),
                movement.to_string().white()
            );
            for _ in 0..movement {
                _move(&mut knots[0]);

                for index in 1..knots.len() {
                    if _distance(&knots[index - 1], &knots[index]) > 1 {
                        knots[index].0 += (knots[index - 1].0 - knots[index].0).clamp(-1, 1);
                        knots[index].1 += (knots[index - 1].1 - knots[index].1).clamp(-1, 1);
                    }
                }
                last_visited.push(*knots.last().unwrap());

                for (i, knot) in knots.iter().enumerate() {
                    tail_visited.get_mut(&i).unwrap().insert(*knot);
                }
            }
        }
    }
    println!(
        "{}",
        _print(tail_visited.clone(), knots.clone()).white().bold()
    );
    println!("p1 {}", tail_visited.get(&1).unwrap().len());
    println!("p2 {}", tail_visited.get(&9).unwrap().len());
}

fn d10() {
    let lines = read_input("in_d10");
    let mut cycle: i32 = 0;
    let mut register = 1;
    let mut loc = 20;
    let mut sum_poi = 0;
    let mut display = [false; 40 * 6];
    let mut _check = |regis: i32| {
        cycle += 1;
        if cycle == loc {
            sum_poi += regis * cycle;
            loc += 40;
        }
        let is_overlapping = regis - 1 == (cycle - 1) % 40
            || regis == (cycle - 1) % 40
            || regis + 1 == (cycle - 1) % 40;
        display[cycle as usize - 1] = is_overlapping;
    };
    for line in lines {
        if let [_, value] = line.split_whitespace().collect::<Vec<_>>()[..] {
            _check(register);
            _check(register);
            register += value.parse::<i32>().unwrap();
        } else {
            _check(register);
        }
    }
    println!("p1 {}", sum_poi);
    println!("p2");

    display.iter().enumerate().for_each(|(i, b)| {
        if i == 0 {
            print!("{} ", "██ ".yellow());
            print!("{}", "█ ".repeat(39).red().on_green());
            println!("  {}", "██ ".yellow());
            println!();
        }
        if i % 40 == 0 {
            print!("{} ", "██ ".yellow());
        }
        if *b {
            print!("{}", "██".bright_white());
        } else {
            print!("{}", "  ");
        }
        if i % 40 == 39 {
            println!("{}", "██ ".yellow());
        }
        if i == display.len() - 1 {
            println!();
            print!("{} ", "██ ".yellow());
            print!("{}", " █".repeat(39).red().on_bright_green());
            println!("  {}", "██ ".yellow());
        }
    });
}

fn d11() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    struct MonkeyId(u128);
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct WorryLevel(u128);
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Value {
        Number(u128),
        Old,
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Operation {
        Add(Value),
        Multiply(Value),
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Monkey {
        id: MonkeyId,
        items: VecDeque<WorryLevel>,
        operation: Operation,
        test: (WorryLevel, MonkeyId, MonkeyId),
        inspection_count: u128,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct MonkeyBuilder {
        id: MonkeyId,
        items: Vec<WorryLevel>,
        operation: Option<Operation>,
        test: (Option<WorryLevel>, Option<MonkeyId>, Option<MonkeyId>),
    }

    impl MonkeyBuilder {
        fn new(id: u128) -> Self {
            Self {
                id: MonkeyId(id),
                items: Vec::new(),
                operation: None,
                test: (None, None, None),
            }
        }
        fn items(&mut self, items: Vec<String>) {
            self.items = items
                .iter()
                .map(|i| WorryLevel(i.parse::<u128>().unwrap()))
                .collect();
        }

        fn operation(&mut self, operation: String) {
            let operation = operation.split(" ").skip(3);
            let operation = match operation.collect::<Vec<_>>()[..] {
                ["+", value] => Operation::Add(
                    value
                        .parse::<u128>()
                        .map_or(Value::Old, |v| Value::Number(v)),
                ),
                ["*", value] => Operation::Multiply(
                    value
                        .parse::<u128>()
                        .map_or(Value::Old, |v| Value::Number(v)),
                ),
                _ => panic!("unknown operation"),
            };
            self.operation = Some(operation);
        }

        fn test_for(&mut self, test_for: u128) {
            self.test.0 = Some(WorryLevel(test_for));
        }

        fn if_true(&mut self, if_true: u128) {
            self.test.1 = Some(MonkeyId(if_true));
        }

        fn if_false(&mut self, if_false: u128) {
            self.test.2 = Some(MonkeyId(if_false));
        }
    }

    impl Monkey {
        fn inspect(&mut self) -> Option<u128> {
            if let Some(new_worry_level) = self.operate() {
                self.inspection_count += 1;
                let WorryLevel(worry_level) = self.items.get_mut(0).unwrap();
                *worry_level = new_worry_level;
                Some(*worry_level)
            } else {
                None
            }
        }

        fn throw(&mut self) -> (WorryLevel, MonkeyId) {
            let worry_level = self.items.pop_front().unwrap();
            if worry_level.0 % self.test.0 .0 == 0 {
                (worry_level, self.test.1)
            } else {
                (worry_level, self.test.2)
            }
        }

        fn operate(&self) -> Option<u128> {
            if let Some(WorryLevel(worry_level)) = self.items.front() {
                Some(match &self.operation {
                    Operation::Add(Value::Number(value)) => {
                        worry_level.checked_add(*value).unwrap()
                    }
                    Operation::Add(Value::Old) => worry_level.checked_pow(2).unwrap(),
                    Operation::Multiply(Value::Number(value)) => {
                        worry_level.checked_mul(*value).unwrap()
                    }
                    Operation::Multiply(Value::Old) => worry_level.checked_pow(2).unwrap(),
                })
            } else {
                None
            }
        }
    }

    let mut monkey_builder = None;
    let mut monkeys: HashMap<MonkeyId, Monkey> = HashMap::new();

    let lines = read_input("in_d11");
    for line in lines {
        match line.split(":").collect::<Vec<_>>()[..] {
            [monkey, ""] => {
                let monkey = monkey.split_whitespace().skip(1);
                let monkey = monkey.collect::<Vec<_>>()[0];
                let id = monkey.parse::<u128>().unwrap();
                monkey_builder = Some(MonkeyBuilder::new(id));
            }
            ["  Starting items", items] => {
                let items = items.split(", ").map(|i| i.trim().to_string()).collect();
                monkey_builder.as_mut().unwrap().items(items);
            }
            ["  Operation", operation] => {
                monkey_builder
                    .as_mut()
                    .unwrap()
                    .operation(operation.trim().to_string());
            }
            ["  Test", test] => {
                let test = test.split_whitespace().skip(2);
                let test = test.collect::<Vec<_>>()[0];
                monkey_builder
                    .as_mut()
                    .unwrap()
                    .test_for(test.parse::<u128>().unwrap());
            }
            ["    If true", if_true] => {
                let if_true = if_true.split_whitespace().skip(3);
                let if_true = if_true.collect::<Vec<_>>()[0];
                monkey_builder
                    .as_mut()
                    .unwrap()
                    .if_true(if_true.parse::<u128>().unwrap());
            }
            ["    If false", if_false] => {
                let if_false = if_false.split_whitespace().skip(3);
                let if_false = if_false.collect::<Vec<_>>()[0];
                monkey_builder
                    .as_mut()
                    .unwrap()
                    .if_false(if_false.parse::<u128>().unwrap());
                let monkey_builder = monkey_builder.take().unwrap();
                monkeys.insert(
                    monkey_builder.id,
                    Monkey {
                        id: monkey_builder.id,
                        items: monkey_builder.items.into(),
                        operation: monkey_builder.operation.unwrap(),
                        test: (
                            monkey_builder.test.0.unwrap(),
                            monkey_builder.test.1.unwrap(),
                            monkey_builder.test.2.unwrap(),
                        ),
                        inspection_count: 0,
                    },
                );
            }
            _ => {}
        }
    }
    let cloned = monkeys.clone();
    let mut ordered_monkeys = cloned.iter().map(|(id, _)| id).collect::<Vec<_>>();
    ordered_monkeys.sort();
    let ordered_monkeys = ordered_monkeys.clone();
    let common_divider: Vec<_> = monkeys
        .clone()
        .into_iter()
        .map(|(_, monkey)| monkey.test.0 .0)
        .collect();
    let common_divider = common_divider.iter().fold(1, |acc, x| acc * x);
    for round in 1..=10_000 {
        // println!("Round {}", round);
        for monkey_id in ordered_monkeys.iter() {
            loop {
                let monkey = monkeys.get_mut(monkey_id).unwrap();

                let (item, next_monkey) = if monkey.inspect().is_some() {
                    let (item, next) = monkey.throw();
                    let mut item_clone = item.clone();
                    item_clone.0 = item_clone.0 % common_divider;
                    (item_clone, next)
                } else {
                    break;
                };

                let next_monkey = monkeys.get_mut(&next_monkey).unwrap();
                next_monkey.items.push_back(item);
            }
        }
        if round == 20 || round == 10_000 {
            let mut inspection_counts = monkeys
                .iter()
                .map(|(_, m)| m.inspection_count)
                .collect::<Vec<_>>();
            inspection_counts.sort();
            inspection_counts.reverse();
            println!(
                "round {}: {}",
                round,
                inspection_counts[0] * inspection_counts[1]
            );
        }
    }
}

fn d12() {
    let start = Instant::now();
    let lines = read_input("in_d12");
    let mut terrain = Vec::new();
    let mut start_pos = None;
    let mut end_pos = None;
    let mut a_pos = Vec::new();
    #[derive(Debug, Clone, Copy)]
    enum Tile {
        Value(u8),
        Start,
        End,
    }
    impl Tile {
        fn value(&self) -> u8 {
            match self {
                Tile::Value(v) => *v,
                Tile::Start => 0,
                Tile::End => 'z' as u8 - 'a' as u8,
            }
        }
    }
    for (x, line) in lines.iter().enumerate() {
        let row = line
            .chars()
            .enumerate()
            .map(|(y, c)| match c {
                'S' => {
                    start_pos = Some((y, x));
                    Tile::Start
                }
                'E' => {
                    end_pos = Some((y, x));
                    Tile::End
                }
                'a' => {
                    a_pos.push((y, x));
                    Tile::Value(0)
                }
                'a'..='z' => Tile::Value(c as u8 - 'a' as u8),
                _ => panic!(),
            })
            .collect::<Vec<_>>();
        terrain.push(row);
    }

    fn get_adjacent(pos: (usize, usize), terrain: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();
        let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let tile = terrain[pos.1][pos.0];
        for dir in dirs {
            let x = pos.0 as i32 + dir.0;
            let y = pos.1 as i32 + dir.1;
            if x >= 0 && y >= 0 {
                if let Some(neighbor) = terrain.get(y as usize).and_then(|row| row.get(x as usize))
                {
                    if neighbor.value() <= tile.value() + 1 {
                        adjacent.push((x as usize, y as usize));
                    }
                }
            }
        }
        adjacent
    }
    let mut visited = HashSet::new();
    let mut current = vec![vec![start_pos.unwrap()], a_pos].concat();
    visited.insert(start_pos.unwrap());
    for i in 0.. {
        let mut next = Vec::new();
        for pos in current.iter() {
            if pos == &end_pos.unwrap() {
                println!("p1: {}", i);
                let duration = start.elapsed();
                println!("Time elapsed: {:?}", duration);
                return;
            }
            for adj in get_adjacent(*pos, &terrain) {
                if !visited.contains(&adj) {
                    visited.insert(adj);
                    next.push(adj);
                }
            }
        }
        current = next;
    }
}

fn d13() {
    let lines = read_input("in_d13");

    #[derive(Debug, Clone)]
    enum Value {
        Int(i64),
        List(Vec<Value>),
    }

    #[derive(Debug, Clone)]
    struct Packet {
        values: Value,
    }

    impl From<String> for Packet {
        fn from(s: String) -> Self {
            let mut values = HashMap::new();
            values.insert(0, Value::List(Vec::new()));
            let mut current = String::new();
            let mut depth = 0;
            for c in s.chars() {
                match c {
                    '[' => {
                        depth += 1;
                        values.insert(depth, Value::List(Vec::new()));
                    }
                    ']' => {
                        let value = if let Some(Value::List(list)) = &mut values.remove(&(depth)) {
                            if !current.is_empty() {
                                list.push(Value::Int(current.parse().unwrap()));
                                current.clear();
                            }
                            list.clone()
                        } else {
                            panic!("value {:?} is not a list", current)
                        };
                        depth -= 1;
                        if let Some(Value::List(list)) = &mut values.get_mut(&(depth)) {
                            list.push(Value::List(value));
                        }
                    }
                    ',' => {
                        if let Some(Value::List(list)) = &mut values.get_mut(&(depth)) {
                            if !current.is_empty() {
                                list.push(Value::Int(current.parse().unwrap()));
                                current.clear();
                            }
                        }
                    }
                    v => current.push(v),
                }
            }
            Packet {
                values: values.get(&0).unwrap().clone(),
            }
        }
    }

    fn compare_values(v1: &Value, v2: &Value) -> Ordering {
        match (v1, v2) {
            (Value::Int(i1), Value::Int(i2)) => i1.cmp(i2),
            (Value::List(l1), Value::List(l2)) => {
                for (v1, v2) in l1.iter().zip(l2.iter()) {
                    let cmp = compare_values(v1, v2);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                if l1.len() < l2.len() {
                    return Ordering::Less;
                } else if l1.len() > l2.len() {
                    return Ordering::Greater;
                }
                return Ordering::Equal;
            }
            (Value::Int(_), Value::List(_)) => {
                return compare_values(&Value::List(vec![v1.clone()]), v2);
            }
            (Value::List(_), Value::Int(_)) => {
                return compare_values(v1, &Value::List(vec![v2.clone()]));
            }
        }
    }

    let mut list = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|p| Packet::from(p.to_string()))
        .collect::<Vec<_>>();

    let p: Vec<[Packet; 2]> = list
        .chunks_exact(2)
        .map(|c| [c[0].clone(), c[1].clone()])
        .collect();
    let mut sum_of_correct = 0;
    for (i, [p_left, p_right]) in p.iter().enumerate() {
        let cmp = compare_values(&p_left.values, &p_right.values);
        if Ordering::Less == cmp {
            sum_of_correct += i + 1;
        }
    }
    println!("p1: {}", sum_of_correct.to_string().bold().white());

    let divider_packet1 = Value::List(vec![Value::List(vec![Value::Int(2)])]);
    let divider_packet2 = Value::List(vec![Value::List(vec![Value::Int(6)])]);

    list.push(Packet {
        values: divider_packet1.clone(),
    });

    list.push(Packet {
        values: divider_packet2.clone(),
    });

    list.sort_by(|p1, p2| compare_values(&p1.values, &p2.values));

    let div1_index = list
        .iter()
        .position(|p| compare_values(&p.values, &divider_packet1) == Ordering::Equal)
        .unwrap();

    let div2_index = list
        .iter()
        .position(|p| compare_values(&p.values, &divider_packet2) == Ordering::Equal)
        .unwrap();

    println!("p2: {}", (div1_index + 1) * (div2_index + 1));
}

fn d14() {
    let lines = read_input("in_d14");
    let sand = (500, 0);
    let mut stone_veins = Vec::new();

    let mut max = (0, 0);
    let mut min = ((0 as u32).overflowing_sub(1).0, 0);

    for line in lines {
        let inputs: Vec<(u32, u32)> = line
            .split(" -> ")
            .collect::<Vec<_>>()
            .iter()
            .map(
                |p| match p.split(",").map(|n| n.parse().unwrap()).collect::<Vec<_>>()[..] {
                    [x, y] => {
                        if x > max.0 {
                            max.0 = x;
                        }
                        if y > max.1 {
                            max.1 = y;
                        }
                        if x < min.0 {
                            min.0 = x;
                        }
                        if y < min.1 {
                            min.1 = y;
                        }
                        (x, y)
                    }
                    _ => panic!("invalid input"),
                },
            )
            .collect();
        stone_veins.push(inputs);
    }

    fn is_stone(x: u32, y: u32, stone_veins: &Vec<Vec<(u32, u32)>>, max_y: &u32) -> bool {
        if y == *max_y + 2 {
            return true;
        }
        stone_veins.iter().any(|vein| {
            (0..vein.len() - 1).any(|i| {
                let (x1, y1) = vein[i];
                let (x2, y2) = vein[i + 1];
                if x1 == x2 {
                    x == x1 && ((y >= y1 && y <= y2) || (y >= y2 && y <= y1))
                } else if y1 == y2 {
                    y == y1 && ((x >= x1 && x <= x2) || (x >= x2 && x <= x1))
                } else {
                    false
                }
            })
        })
    }

    fn is_sand(x: u32, y: u32, sands: &Vec<(u32, u32)>) -> bool {
        sands.iter().any(|(x1, y1)| x == *x1 && y == *y1)
    }

    fn is_occupied(
        x: u32,
        y: u32,
        stone_veins: &Vec<Vec<(u32, u32)>>,
        sands: &Vec<(u32, u32)>,
        max_y: &u32,
    ) -> bool {
        is_sand(x, y, sands) || is_stone(x, y, stone_veins, max_y)
    }

    let mut can_move = true;
    let mut current_sand = sand.clone();
    let mut sands = Vec::new();
    let mut trail = Vec::new();
    let mut finished = false;

    for i in 0.. {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        if !can_move {
            sands.push(current_sand);
            current_sand = sand.clone();
            can_move = true;
        }
        while can_move {
            trail.push(current_sand.clone());
            if !is_occupied(
                current_sand.0,
                current_sand.1 + 1,
                &stone_veins,
                &sands,
                &max.1,
            ) {
                current_sand.1 += 1;
                continue;
            }
            if !is_occupied(
                current_sand.0 - 1,
                current_sand.1 + 1,
                &stone_veins,
                &sands,
                &max.1,
            ) {
                current_sand.0 -= 1;
                current_sand.1 += 1;
                continue;
            }
            if !is_occupied(
                current_sand.0 + 1,
                current_sand.1 + 1,
                &stone_veins,
                &sands,
                &max.1,
            ) {
                current_sand.0 += 1;
                current_sand.1 += 1;
                continue;
            }
            if current_sand == (500, 0) {
                println!("[{}], start reached", sands.len() + 1);
                finished = true;
                break;
            }
            can_move = false;
        }
        if finished {
            break;
        }
    }
    for y in min.1..=(max.1 + 3) {
        for x in (min.0 - 1)..=(max.0 + 1) {
            let is_stone = is_stone(x, y, &stone_veins, &max.1);
            let is_sand_spawner = (x, y) == sand;
            let is_sand_block = is_sand(x, y, &sands);
            let is_trail = is_sand(x, y, &trail);
            print!(
                "{}",
                if is_sand_spawner {
                    "  ".to_string().on_purple()
                } else if is_stone {
                    "  ".to_string().on_bright_black()
                } else if is_sand_block {
                    "  ".to_string().on_yellow()
                } else if is_trail {
                    "  ".to_string().on_bright_yellow()
                } else {
                    "██".to_string().on_black().black()
                }
            );
        }
        println!();
    }
}
