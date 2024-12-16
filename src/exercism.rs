#![allow(dead_code, unused_variables, clippy::new_without_default)]
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::{default, usize};

pub fn call() {
    println!("namah prabhu /\\ /\\");
    // binary_search();
    // etl();
    // grade_school();
    // valid_isbn();
    // isogram();
    // dna_nucleotide();
}

fn dna_nucleotide() {
    let res = n_count('X', "A");
    println!("{:?}", res);
}

const VALIDATE_NUCLEOTIE: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn n_count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut counts = nucleotide_counts(dna)?;
    counts.remove(&nucleotide).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = VALIDATE_NUCLEOTIE.iter().map(|&c| (c, 0)).collect();
    for c in dna.chars() {
        map.get_mut(&c).map(|count| *count += 1).ok_or(c)?;
    }
    Ok(map)
}

fn validate_nucleotie(dna: &str) -> Result<(), char> {
    dna.chars()
        .find(|&c| !VALIDATE_NUCLEOTIE.contains(&c))
        .map_or(Ok(()), Err)
}

fn isogram() {
    let s = "eleven";
    let s = "igoram";
    let res = check_isogram(s);
    println!("{:?}", res);
}

fn check_isogram(candidate: &str) -> bool {
    let mut set = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| set.insert(c))
}

fn valid_isbn() {
    // let s = "3-598-2X507-9";
    let s = "3-598-21508-8";
    let res = is_valid_isbn(s);
    println!("{:?}", res);
}

fn is_valid_isbn(s: &str) -> bool {
    s.chars()
        .try_fold((0, 0), |(count, sum), c| match (count, c) {
            (_, '0'..='9') => Some((
                count + 1,
                (sum + (10 - count) * c.to_digit(10).unwrap()) % 11,
            )),
            (9, 'X') => Some((count + 1, (sum + 10) % 11)),
            (_, '-') => Some((count, sum)),
            _ => None,
        })
        == Some((10, 0))
}

fn is_valid_isbn_4(s: &str) -> bool {
    s.chars()
        .fold((0, 0, true), |(count, sum, is_valid), c| match c {
            '0'..='9' => (
                count + 1,
                ((sum + (10 - count) * c.to_digit(10).unwrap()) % 11),
                is_valid,
            ),
            'X' => (count + 1, (sum + 10) % 11, is_valid && count == 9),
            '-' => (count, sum, is_valid),
            _ => (0, 0, false),
        })
        == (10, 0, true)
}

fn is_valid_isbn_3(s: &str) -> bool {
    s.chars()
        .filter(|&c| c != '-')
        .enumerate()
        .map(|(i, c)| match (i, c) {
            (9, 'X') => Some(10),
            (_, c) => c.to_digit(10),
        })
        .collect::<Option<Vec<u32>>>()
        .map(|digits| {
            if digits.len() == 10 {
                digits
                    .iter()
                    .enumerate()
                    .map(|(i, e)| (10 - i) as u32 * e)
                    .sum::<u32>()
                    % 11
                    == 0
            } else {
                false
            }
        })
        .unwrap_or(false)
}

fn is_valid_isbn_2(s: &str) -> bool {
    let str = str::replace(s, "-", "");
    if str.len() != 10 {
        return false;
    }
    let mut sum = 0;
    for (i, c) in str.char_indices() {
        if c.is_ascii_digit() {
            sum += c.to_digit(10).unwrap() * (10 - i) as u32;
        } else if i == 9 && c == 'X' {
            sum += 10;
        } else {
            return false;
        }
    }
    sum % 11 == 0
}

fn is_valid_isbn_1(s: &str) -> bool {
    let r = s
        .split('-')
        .flat_map(|word| word.chars())
        .collect::<Vec<_>>();
    if r.len() != 10 || contains_invalid_isbn(&r) {
        return false;
    }
    let res = r
        .iter()
        .zip((1..=10).rev())
        .map(|(x, d)| (*x).to_string().parse::<u32>().unwrap_or(10) * d)
        .sum::<u32>();
    res % 11 == 0
}

fn contains_invalid_isbn(slice: &[char]) -> bool {
    let all_digits = slice.iter().all(|c| c.is_ascii_digit());
    if all_digits {
        return false;
    }
    let invalid_chars = slice.iter().any(|c| !c.is_ascii_digit() && *c != 'X');
    if invalid_chars {
        return true;
    }
    if let Some('X') = slice.iter().last() {
        return false;
    }

    true
}

fn grade_school() {
    let mut s = School::new();
    s.add(2, "Blair");
    s.add(2, "James");
    s.add(2, "James");
    s.add(2, "Paul");
    let grade = s.grade(2);
    println!("{:?}", grade);
    let grades = s.grades();
    println!("{:?}", grades);
}

pub struct School {
    grades: BTreeMap<u32, BTreeSet<String>>,
    students: BTreeSet<String>,
}

impl School {
    pub fn new() -> School {
        Self {
            grades: BTreeMap::new(),
            students: BTreeSet::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.students.contains(student) {
            self.students.insert(student.into());
            // self.grades
            //     .entry(grade)
            //     .and_modify(|list| {
            //         list.insert(String::from(student));
            //     })
            //     .or_insert(BTreeSet::from([String::from(student)]));
            //     NOTE: gives mut reference and then insert into it
            // self.grades.entry(grade).or_insert_with(BTreeSet::new).insert(student.into());
            //     NOTE: same as above
            // self.grades.entry(grade).or_insert(BTreeSet::new()).insert(student.into());
            self.grades.entry(grade).or_default().insert(student.into());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades
            .get(&grade)
            .map_or(vec![], |x| x.iter().cloned().collect())
    }
}

fn etl() {
    let a = BTreeMap::from([(1, vec!['A', 'E']), (2, vec!['D', 'G'])]);
    let res = transform(&a);
    println!("{:?}", res);
}

fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&point, vec)| {
            vec.iter()
                .map(|char| (char.to_ascii_lowercase(), point))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
struct BowlingGame {
    throw: Vec<u16>,
    second: bool,
}

impl BowlingGame {
    fn new() -> Self {
        Default::default()
    }

    fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || (self.second && pins + self.throw.last().unwrap() > 10) {
            Err(Error::NotEnoughPinsLeft)
        } else if self.score().is_some() {
            Err(Error::GameComplete)
        } else {
            self.throw.push(pins);
            self.second = if pins != 10 { !self.second } else { false };
            Ok(())
        }
    }

    fn score(&self) -> Option<u16> {
        let mut total = 0;
        let mut frames = 0;
        for _ in 0..10 {
            if let (Some(&first), Some(&second)) =
                (self.throw.get(frames), self.throw.get(frames + 1))
            {
                total += first + second;
                if first == 10 || first + second == 10 {
                    if let Some(thrid) = self.throw.get(frames + 2) {
                        total += thrid;
                    } else {
                        return None;
                    }
                }
                frames += if first == 10 { 1 } else { 2 };
            } else {
                return None;
            }
        }
        Some(total)
    }
}

fn binary_search() {
    let array = [1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377];
    let result = call_binary_search_1(array, 21);
    println!("{:?}", result);
}

fn call_binary_search(array: &[i32], key: i32) -> Option<usize> {
    let mut hi = array.len();
    let mut lo = 0;
    while lo < hi {
        let middle = (hi + lo) / 2;
        match key < array[middle] {
            true => {
                hi = middle;
            }
            false if key > array[middle] => {
                lo = middle + 1;
            }
            false => return Some(middle),
        }
    }
    None
}

fn call_binary_search_1<R: AsRef<[T]>, T: Ord>(array: R, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mid = array.len() / 2;
    match key.cmp(array.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Less => call_binary_search_1(&array[..mid], key),
        Ordering::Greater => {
            call_binary_search_1(&array[mid + 1..], key).map(|index| index + mid + 1)
        }
    }
}
