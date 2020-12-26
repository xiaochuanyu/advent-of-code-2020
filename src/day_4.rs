use crate::util;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};

pub fn part_1() {
    fn is_valid(keys: &HashSet<String>) -> bool {
        let required_fields: HashSet<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .cloned()
            .map(String::from)
            .collect(); // wow so much work for HashSet<String>

        keys.is_superset(&required_fields)
    }
    let file = util::read_input();
    let mut keys = HashSet::new();
    let mut num_valid = 0;
    for line in file.lines().map(|result| result.unwrap()) {
        if line.len() == 0 {
            if is_valid(&keys) {
                num_valid += 1;
            }
            keys.clear();
        } else {
            let fields = line.split(" ");
            for field in fields {
                let key = field.split(":").nth(0).unwrap();
                keys.insert(key.to_owned());
            }
        }
    }
    if keys.len() != 0 {
        if is_valid(&keys) {
            num_valid += 1;
        }
    }
    println!("{}", num_valid);
}

pub fn part_2() {
    type CheckResult = Result<(), ()>;
    fn check_year(year: &str, min: i32, max: i32) -> CheckResult {
        let year: i32 = year.parse().unwrap();
        if year >= min && year <= max {
            Ok(())
        } else {
            Err(())
        }
    }
    fn check_height(height: &str) -> CheckResult {
        if height.len() < 3 {
            return Err(());
        }
        let value :u32 = height[..height.len() - 2].parse().or(Err(()))?;
        if height.ends_with("cm") && value >= 150 && value <= 193{
            return Ok(());
        } else if height.ends_with("in") && value >= 59 && value <= 76 {
            return Ok(());
        }
        Err(())
    }
    fn check_hair_color(hcl: &str) -> CheckResult {
        let mut state = 0;
        if hcl.len() != 7 {
            return Err(());
        }
        for c in hcl.chars() {
            if state == 0 {
                if c == '#' {
                    state = 1;
                    continue;
                }
                return Err(());
            } else if state == 1 {
                if c.is_alphanumeric() {
                    continue;
                }
                return Err(());
            }
        }
        Ok(())
    }
    fn check_eye_color(ecl: &str) -> CheckResult {
        let choices: HashSet<_> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .cloned()
            .collect(); // wow so much work for HashSet<String>
        if choices.contains(ecl) {
            return Ok(());
        }
        Err(())
    }
    fn check_passport_id(pid: &str) -> CheckResult {
        if pid.len() != 9 {
            return Err(());
        }
        pid.parse::<u32>().or(Err(()))?;
        Ok(())
    }
    fn is_valid(key_vals: &HashMap<String, String>) -> CheckResult {
        let byr = key_vals.get("byr").ok_or(())?;
        check_year(&byr, 1920, 2002)?;
        let iyr = key_vals.get("iyr").ok_or(())?;
        check_year(&iyr, 2010, 2020)?;
        let eyr = key_vals.get("eyr").ok_or(())?;
        check_year(&eyr, 2020, 2030)?;
        let hgt = key_vals.get("hgt").ok_or(())?;
        check_height(hgt)?;
        let hcl = key_vals.get("hcl").ok_or(())?;
        check_hair_color(hcl)?;
        let ecl = key_vals.get("ecl").ok_or(())?;
        check_eye_color(ecl)?;
        let pid = key_vals.get("pid").ok_or(())?;
        check_passport_id(pid)?;
        Ok(())
    }

    let file = util::read_input();
    let mut key_vals: HashMap<String, String> = HashMap::new();
    let mut num_valid = 0;
    for line in file.lines().map(|result| result.unwrap()) {
        if line.len() == 0 {
            if is_valid(&key_vals).is_ok() {
                num_valid += 1;
            }
            key_vals.clear();
        } else {
            let fields = line.split(" ");
            for field in fields {
                let key_val = field.split(":").collect::<Vec<&str>>();
                key_vals.insert(key_val[0].to_owned(), key_val[1].to_owned());
            }
        }
    }
    if key_vals.len() != 0 {
        if is_valid(&key_vals).is_ok() {
            num_valid += 1;
        }
    }
    println!("{}", num_valid);
}
