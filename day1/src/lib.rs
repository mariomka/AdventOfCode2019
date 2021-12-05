use helpers::debug;

pub fn part1(input: &Vec<isize>) -> isize {
    input.iter().map(|mass| mass / 3 - 2).sum()
}

pub fn part2(input: &Vec<isize>) -> isize {
    input.iter().map(|mass| calc_fuel(*mass)).sum()
}

fn calc_fuel(mass: isize) -> isize {
    let fuel = mass / 3 - 2;

    if fuel > 0 {
        fuel + calc_fuel(fuel)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use helpers::{input_lines, parse_input};

    use super::*;

    fn input<'a>() -> Vec<isize> {
        let input = "\
12
14
1969
100756";
        parse_input(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 2 + 2 + 654 + 33583)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 2 + 2 + 966 + 50346)
    }
}
