pub fn part1(input: &Vec<usize>) -> usize {
    let mut program = input.clone();
    program[1] = 12;
    program[2] = 2;

    int_code(program)
}

pub fn part2(input: &Vec<usize>) -> usize {
    let mut res = 0;

    'outer: for i in 0..=99 {
        for j in 0..=99 {
            let mut program = input.clone();
            program[1] = i;
            program[2] = j;

            if 19690720 == int_code(program) {
                res = 100 * i + j;
                break 'outer;
            }
        }
    }

    res
}

fn int_code(mut program: Vec<usize>) -> usize {
    let mut instruction_pointer = 0;

    loop {
        let instruction = program[instruction_pointer];
        let parameter1 = if 99 == instruction { 0 } else { program[instruction_pointer + 1] };
        let parameter2 = if 99 == instruction { 0 } else { program[instruction_pointer + 2] };
        let parameter3 = if 99 == instruction { 0 } else { program[instruction_pointer + 3] };

        match instruction {
            1 => program[parameter3] = program[parameter1] + program[parameter2],
            2 => program[parameter3] = program[parameter1] * program[parameter2],
            99 => break,
            _ => {}
        }

        instruction_pointer += 4;
    }

    program[0]
}

#[cfg(test)]
mod tests {
    use helpers::parse_split_input;

    use super::*;

    #[test]
    fn test_int_code() {
        assert_eq!(int_code(parse_split_input("1,9,10,3,2,3,11,0,99,30,40,50", ",")), 3500);
        assert_eq!(int_code(parse_split_input("1,0,0,0,99", ",")), 2);
        assert_eq!(int_code(parse_split_input("1,1,1,4,99,5,6,0,99", ",")), 30);
    }
}
