use std::collections::HashMap;
use std::include_str;

fn input() -> &'static str {
    include_str!("inputs/day_4.txt")
}

const NEEDED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> From<&'a str> for Passport<'a> {
    fn from(input: &'a str) -> Self {
        let fields: HashMap<&'a str, &'a str> = input
            .split_whitespace()
            .map(|pair| pair.split(':'))
            .filter_map(|mut kv| {
                if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
                    Some((key, value))
                } else {
                    None
                }
            })
            .collect();
        Passport { fields }
    }
}

impl<'a> Passport<'a> {
    fn is_valid_credential(&self) -> bool {
        NEEDED_KEYS.iter().all(|key| self.fields.contains_key(*key))
    }
}

fn parse_file<'a>(input: &'a str) -> Vec<Passport<'a>> {
    input
        .split("\n\n")
        .map(|passport_text| passport_text.into())
        .collect()
}

fn task_1<'a>(input: &'a str) -> usize {
    let passports = parse_file(input);
    passports
        .iter()
        .filter(|passport| passport.is_valid_credential())
        .count()
}

pub fn run() {
    println!("Day 4 task 1: {}", task_1(input()));
    // println!("Day 4 task 2: {}", task_2(input()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_on_empty_line() {
        let example = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let passports = example.split("\n\n");
        assert_eq!(passports.count(), 4);
    }

    #[test]
    fn validate_example_1() {
        let example = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let passport: Passport = example.into();
        assert!(passport.is_valid_credential());
    }

    #[test]
    fn validate_example_2() {
        let example = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929";
        let passport: Passport = example.into();
        assert!(!passport.is_valid_credential());
    }

    #[test]
    fn validate_example_3() {
        let example = "hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm";
        let passport: Passport = example.into();
        assert!(passport.is_valid_credential());
    }

    #[test]
    fn validate_example_4() {
        let example = "hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let passport: Passport = example.into();
        assert!(!passport.is_valid_credential());
    }

    #[test]
    fn task_1_test() {
        assert_eq!(task_1(input()), 239)
    }
}
