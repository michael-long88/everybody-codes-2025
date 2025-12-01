ec::solution!(1);

pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn convert_from_char(s: &char) -> Option<Direction> {
        match s {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

pub struct Instruction {
    pub direction: Direction,
    pub value: usize,
}

impl Instruction {
    pub fn new(direction: &char, value: usize) -> Self {
        Instruction {
            direction: Direction::convert_from_char(direction).unwrap(),
            value,
        }
    }

    pub fn get_name_index(&self, names: &[&str], current_index: usize, part1: bool) -> usize {
        if part1 {
            match self.direction {
                Direction::Left => current_index.saturating_sub(self.value),
                Direction::Right => {
                    if current_index + self.value >= names.len() {
                        names.len() - 1
                    } else {
                        current_index + self.value
                    }
                }
            }
        } else {
            match self.direction {
                Direction::Left => {
                    (current_index + names.len() - (self.value % names.len())) % names.len()
                }
                Direction::Right => (current_index + self.value) % names.len(),
            }
        }
    }

    pub fn print(&self) {
        match self.direction {
            Direction::Left => println!("Move Left by {}", self.value),
            Direction::Right => println!("Move Right by {}", self.value),
        }
    }
}

pub fn parse(input: &str) -> (Vec<&str>, Vec<Instruction>) {
    let lines: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|line| line.split(',').collect())
        .collect();

    let names = lines[0].clone();
    let instructions = lines[1]
        .iter()
        .map(|instruction| {
            let instruction_pieces = instruction.trim().chars().collect::<Vec<char>>();
            Instruction::new(
                &instruction_pieces[0],
                instruction_pieces[1..]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            )
        })
        .collect();

    (names, instructions)
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let (names, instructions) = parse(notes);

    let mut current_index = 0;
    for instruction in instructions.iter() {
        current_index = instruction.get_name_index(&names, current_index, true);
    }

    Some(names[current_index].to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let (names, instructions) = parse(notes);

    let mut current_index = 0;
    for instruction in instructions.iter() {
        current_index = instruction.get_name_index(&names, current_index, false);
    }

    Some(names[current_index].to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let (mut names, instructions) = parse(notes);

    let current_index = 0;
    for instruction in instructions.iter() {
        let new_index = instruction.get_name_index(&names, current_index, false);
        names.swap(current_index, new_index);
    }

    Some(names[current_index].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(1, 1));
        assert_eq!(result, Some("Fyrryn".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(1, 2));
        assert_eq!(result, Some("Elarzris".to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(1, 3));
        assert_eq!(result, Some("Drakzyph".to_string()));
    }
}
