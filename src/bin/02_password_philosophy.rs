use std::collections::HashMap;

struct Password {
    min: usize,
    max: usize,
    letter: char,

    password: String,
}

impl Password {
    fn new(l: &str) -> Password {
        let parts: Vec<&str> = l.trim().splitn(2, ": ").collect();
        let policy_parts: Vec<&str> = parts[0].splitn(2, " ").collect();
        let number_parts: Vec<&str> = policy_parts[0].splitn(2, "-").collect();

        Password {
            min: number_parts[0]
                .parse()
                .expect("unable to parse policy min value"),
            max: number_parts[1]
                .parse()
                .expect("unable to parse policy max value"),
            letter: policy_parts[1]
                .chars()
                .nth(0)
                .expect("unable to get letter from policy"),

            password: String::from(parts[1]),
        }
    }

    fn valid(&self) -> bool {
        let mut count: HashMap<char, usize> = HashMap::with_capacity(self.password.len());
        for c in self.password.chars() {
            count.entry(c).and_modify(|f| *f += 1).or_insert(1);
        }

        match count.get(&self.letter) {
            Some(freq) => self.min <= *freq && self.max >= *freq,
            None => false,
        }
    }

    fn valid_2(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        (chars[self.min - 1] == self.letter) ^ (chars[self.max - 1] == self.letter)
    }
}

fn main() {
    let passwords = adventofcode::read_input("inputs/day02.txt", |l| Password::new(l));
    println!("{}", passwords.iter().filter(|p| p.valid()).count());
    println!("{}", passwords.iter().filter(|p| p.valid_2()).count());
}
