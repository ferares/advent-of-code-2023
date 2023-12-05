fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

struct DigitOccurrence {
    index: i32,
    digit: char,
}

fn find_digit_occurrences(input: &str) -> Vec<DigitOccurrence> {
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut occurrences: Vec<DigitOccurrence> = Vec::new();
    for (index, digit) in digits.iter().enumerate() {
        let occurrence_indexes = input.match_indices(digit);
        for occurrence_index in occurrence_indexes {
            occurrences.push(DigitOccurrence {
                index: occurrence_index.0 as i32,
                digit: (index + 1).to_string().chars().next().unwrap()
            })
        }
    }
    occurrences.sort_by(|a, b| a.index.cmp(&b.index));
    return occurrences;
}

fn part2(input: &str) -> String {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        let mut first_index = line.len() as i32;
        let mut last_index = 0;
        let mut first = '\0';
        let mut last = '\0';
        let occurrences = find_digit_occurrences(line);
        if occurrences.len() > 0 {
            first_index = occurrences.first().unwrap().index;
            first = occurrences.first().unwrap().digit;
            last_index = occurrences.last().unwrap().index;
            last = occurrences.last().unwrap().digit;
        }
        for (index, letter) in line.chars().enumerate() {
            if letter.is_digit(10) {
                if (index as i32) < first_index {
                    first = letter;
                    first_index = index as i32;
                } else if index as i32 > last_index {
                    last = letter;
                    last_index = index as i32;
                }
            }
        }
        let mut number = "".to_string();
        if first != '\0' {
            number.push(first);
            if last != '\0' {
                number.push(last);
            } else {
                number.push(first);
            }
        } else {
            number.push('0');
        }
        sum += number.parse::<i32>().unwrap();
    }
    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = include_str!("./test2.txt");
        let result = part2(input);
        assert_eq!(result, "281");
    }
}