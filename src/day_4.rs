use regex::Regex;
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

enum Height {
    CM(i32),
    IN(i32),
}

fn validate_color(text: &str) -> bool {
    let re = Regex::new(r"#[a-f0-9]{6}").unwrap();
    re.is_match(text)
}

fn validate_eye(text: &str) -> bool {
    let re = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
    re.is_match(text)
}

fn validate_passport_number(text: &str) -> bool {
    let re = Regex::new(r"^\d{9}$").unwrap();
    re.is_match(text)
}

impl<'a> Passport<'a> {
    fn is_valid_credential(&self) -> bool {
        NEEDED_KEYS.iter().all(|key| self.fields.contains_key(*key))
    }

    fn is_valid_strict(&self) -> bool {
        let byr = self
            .fields
            .get("byr")
            .and_then(|val| val.parse::<i32>().ok())
            .filter(|num| num >= &1920 && num <= &2002)
            .is_some();
        let iyr = self
            .fields
            .get("iyr")
            .and_then(|val| val.parse::<i32>().ok())
            .filter(|num| num >= &2010 && num <= &2020)
            .is_some();
        let eyr = self
            .fields
            .get("eyr")
            .and_then(|val| val.parse::<i32>().ok())
            .filter(|num| num >= &2020 && num <= &2030)
            .is_some();

        let hgt = self
            .fields
            .get("hgt")
            .and_then(|val| {
                if let Some(height) = val.strip_suffix("cm") {
                    height.parse::<i32>().ok().map(Height::CM)
                } else if let Some(height) = val.strip_suffix("in") {
                    height.parse::<i32>().ok().map(Height::IN)
                } else {
                    None
                }
            })
            .filter(|height| match height {
                Height::CM(height_cm) => height_cm >= &150 && height_cm <= &193,
                Height::IN(height_in) => height_in >= &59 && height_in <= &76,
            })
            .is_some();
        let hcl = self
            .fields
            .get("hcl")
            .filter(|val| validate_color(val))
            .is_some();
        let ecl = self
            .fields
            .get("ecl")
            .filter(|val| validate_eye(val))
            .is_some();
        let pid = self
            .fields
            .get("pid")
            .filter(|val| validate_passport_number(val))
            .is_some();
        byr && iyr && eyr && hgt && hcl && ecl && pid
    }
}

fn parse_file<'a>(input: &'a str) -> Vec<Passport<'a>> {
    input
        .split("\n\n")
        .map(|passport_text| passport_text.into())
        .collect()
}

fn task_1(input: &'_ str) -> usize {
    let passports = parse_file(input);
    passports
        .iter()
        .filter(|passport| passport.is_valid_credential())
        .count()
}

fn task_2(input: &'_ str) -> usize {
    let passports = parse_file(input);
    passports
        .iter()
        .filter(|passport| passport.is_valid_strict())
        .count()
}

pub fn run() {
    println!("Day 4 task 1: {}", task_1(input()));
    println!("Day 4 task 2: {}", task_2(input()));
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

    #[test]
    fn task_2_test() {
        assert_eq!(task_2(input()), 188)
    }

    #[test]
    fn color_1() {
        assert!(validate_color("#123abc"))
    }

    #[test]
    fn color_2() {
        assert!(!validate_color("#123abz"))
    }

    #[test]
    fn color_3() {
        assert!(!validate_color("123abc"))
    }

    #[test]
    fn eye_1() {
        assert!(validate_eye("brn"))
    }

    #[test]
    fn eye_2() {
        assert!(!validate_eye("wat"))
    }

    #[test]
    fn passport_1() {
        assert!(validate_passport_number("000000001"))
    }

    #[test]
    fn passport_2() {
        assert!(!validate_passport_number("0123456789"))
    }

    #[test]
    fn illegal_passwords_task_2() {
        let example = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let passports = parse_file(example);
        assert!(!passports.iter().any(|passport| passport.is_valid_strict()))
    }

    #[test]
    fn legal_passwords_task() {
        let example = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let passports = parse_file(example);
        assert!(passports.iter().all(|passport| passport.is_valid_strict()))
    }
}
