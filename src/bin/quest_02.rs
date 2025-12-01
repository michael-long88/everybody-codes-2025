ec::solution!(2);

pub fn parse(input: &str) -> (i64, i64) {
    let sections: Vec<&str> = input.split("[").last().unwrap().split(",").collect();
    let value1 = sections[0].trim().parse().unwrap();
    let value2 = sections[1].trim_end_matches("]").parse().unwrap();

    (value1, value2)
}

pub fn add(x1: i64, y1: i64, x2: i64, y2: i64) -> (i64, i64) {
    (x1 + x2, y1 + y2)
}

pub fn multiply(x1: i64, y1: i64, x2: i64, y2: i64) -> (i64, i64) {
    (x1 * x2 - y1 * y2, x1 * y2 + y1 * x2)
}

pub fn divide(x1: i64, y1: i64, x2: i64, y2: i64) -> (i64, i64) {
    (x1 / x2, y1 / y2)
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let mut value1 = 0;
    let mut value2 = 0;

    let (sample_number1, sample_number2) = parse(notes);

    for _ in 0..3 {
        (value1, value2) = multiply(value1, value2, value1, value2);
        (value1, value2) = divide(value1, value2, 10, 10);
        (value1, value2) = add(value1, value2, sample_number1, sample_number2);
    }

    Some(format!("[{},{}]", value1, value2))
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<i32> {
    let (sample_number1, sample_number2) = parse(notes);
    let mut engraved_point_counter = 0;

    for x_step in (0..=1_000).step_by(10) {
        'check_loop: for y_step in (0..=1_000).step_by(10) {
            let mut value1 = 0;
            let mut value2 = 0;

            for _ in 0..100 {
                (value1, value2) = multiply(value1, value2, value1, value2);
                (value1, value2) = divide(value1, value2, 100_000, 100_000);
                (value1, value2) = add(
                    value1,
                    value2,
                    sample_number1 + x_step,
                    sample_number2 + y_step,
                );

                if value1.abs() > 1_000_000 || value2.abs() > 1_000_000 {
                    continue 'check_loop;
                }
            }
            engraved_point_counter += 1;
        }
    }

    Some(engraved_point_counter)
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<i32> {
    let (sample_number1, sample_number2) = parse(notes);
    let mut engraved_point_counter = 0;

    for x_step in 0..=1_000 {
        'check_loop: for y_step in 0..=1_000 {
            let mut value1 = 0;
            let mut value2 = 0;

            for _ in 0..100 {
                (value1, value2) = multiply(value1, value2, value1, value2);
                (value1, value2) = divide(value1, value2, 100_000, 100_000);
                (value1, value2) = add(
                    value1,
                    value2,
                    sample_number1 + x_step,
                    sample_number2 + y_step,
                );

                if value1.abs() > 1_000_000 || value2.abs() > 1_000_000 {
                    continue 'check_loop;
                }
            }
            engraved_point_counter += 1;
        }
    }

    Some(engraved_point_counter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(2, 1));
        assert_eq!(result, Some("[357,862]".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(2, 2));
        assert_eq!(result, Some(4076));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(2, 3));
        assert_eq!(result, Some(406_954));
    }
}
