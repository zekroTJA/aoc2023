fn get_digits(line: &str, with_literals: bool) -> String {
    let mut number = String::new();
    let mut buff = String::new();

    let literals = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for c in line.chars() {
        if c.is_ascii_digit() {
            number.push(c);
            buff.clear();
            continue;
        }

        if with_literals {
            buff.push(c);
            let literal = literals
                .iter()
                .enumerate()
                .find(|(_, &lit)| buff.ends_with(lit))
                .map(|(idx, _)| idx + 1);

            if let Some(literal) = literal {
                number.push_str(&literal.to_string());
            }
        }
    }

    number.parse().unwrap()
}

fn get_combined_number(line: &str, with_literals: bool) -> u32 {
    let digits: Vec<char> = get_digits(line, with_literals).chars().collect();

    let number = if digits.len() == 1 {
        String::from_utf8(vec![digits[0] as u8, digits[0] as u8])
    } else {
        String::from_utf8(vec![digits[0] as u8, *digits.last().unwrap() as u8])
    };

    number.unwrap().parse().unwrap()
}

fn main() {
    let input: String = lib::read_input!();

    let lines: Vec<&str> = input.split('\n').collect();

    let d1: u32 = lines.iter().map(|l| get_combined_number(l, false)).sum();

    println!("Day 1 Solution: {d1}");

    let d2: u32 = lines.iter().map(|l| get_combined_number(l, true)).sum();

    println!("Day 2 Solution: {d2}");
}
