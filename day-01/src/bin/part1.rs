fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        let mut first = '\0';
        let mut last = '\0';
        for letter in line.chars() {
            if letter.is_digit(10) {
                if first == '\0' {
                    first = letter;
                } else {
                    last = letter;
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
        let input = include_str!("./test1.txt");
        let result = part1(input);
        assert_eq!(result, "142");
    }
}