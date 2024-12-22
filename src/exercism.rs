#![allow(unused_variables, dead_code, clippy::new_without_default)]
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::iter::{self, repeat};
pub mod dna_rna;
pub mod paas_io;
pub mod palindrome;
pub mod pascals_triangle;
pub mod queen_attack;

pub fn call() {
    println!("namah prabhu /\\ /\\");
    // binary_search();
    // etl();
    // grade_school();
    // valid_isbn();
    // isogram();
    // dna_nucleotide();
    // passio();
    // palindrome();
    // pangram();
    // alphametics();
    // pascals_triangle::call();
    // latin_kids();
    // run_length_encoding();
    saddle_points();
}

fn saddle_points() {
    let input = &[vec![9, 8, 7], vec![5, 3, 2], vec![6, 6, 7]];
    let res = find_saddle_points(input);
    println!("{:?}", res);
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(move |(c, v)| (r, c, v))
                .filter_map(|(r, c, v)| {
                    if input.iter().all(|row| row[c] >= *v) && input[r].iter().all(|x| v >= x) {
                        Some((r, c))
                    } else {
                        None
                    }
                })
        })
        .collect()
}

pub fn find_saddle_points_2(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    (0..input.len())
        .flat_map(|x| repeat(x).zip(0..input[x].len()))
        .filter(|&(r, c)| is_saddle(c, r, input))
        .collect()
}

fn is_saddle(r: usize, c: usize, input: &[Vec<u64>]) -> bool {
    let val = input[r][c];
    input[r].iter().all(|&x| x <= val) && input.iter().all(|row| row[c] >= val)
}

pub fn find_saddle_points_1(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut perfect_points = Vec::new();
    for (i, row) in input.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            let column = input.iter().all(|row| row[j] >= *value);
            if row.iter().all(|x| x <= value) && column {
                perfect_points.push((i, j));
            }
        }
    }
    perfect_points
}

fn run_length_encoding() {
    assert_eq!(rle_encode("aabbbcccc"), "2a3b4c".to_string());
    assert_eq!(rle_encode_1("aabbbcccc"), "2a3b4c".to_string());
    assert_eq!(rle_decode("2 hs2q q2w2 "), "  hsqq qww  ".to_string());
    assert_eq!(rle_decode_1("2 hs2q q2w2 "), "  hsqq qww  ".to_string());
    assert_eq!(rle_decode_2("2 hs2q q2w2 "), "  hsqq qww  ".to_string());
}

fn rle_encode_1(str: &str) -> String {
    let mut result = String::new();
    let mut peekable = str.chars().peekable();

    while let Some(c) = peekable.next() {
        match iter::from_fn(|| peekable.next_if_eq(&c)).count() {
            0 => result.push(c),
            n => result.push_str(&format!("{}{}", n + 1, c)),
        }
    }
    result
}
fn rle_encode(str: &str) -> String {
    let mut res = String::new();
    let mut count = 0;
    let mut peekable = str.chars().peekable();

    while let Some(c) = peekable.next() {
        count += 1;
        if Some(&c) != peekable.peek() {
            if count > 1 {
                res.push_str(&format!("{}", count));
            }
            res.push_str(&format!("{}", c));
            count = 0;
        }
    }
    res
}

fn rle_decode_2(str: &str) -> String {
    str.chars()
        .filter(|&c: &char| !c.is_numeric())
        .zip(
            str.split(|c: char| !c.is_numeric())
                .map(|x| x.parse::<usize>().unwrap_or(1)),
        )
        // NOTE: flat_map to avoid use of collect() for iter::repeat::take
        .flat_map(|(c, count)| iter::repeat(c).take(count))
        .collect()
}

fn rle_decode_1(str: &str) -> String {
    str.split(|c: char| !c.is_numeric())
        .map(|x| x.parse::<usize>().unwrap_or(1))
        .zip(str.matches(|c: char| !c.is_numeric()))
        .map(|(count, c)| c.repeat(count))
        .collect()
}

fn rle_decode(str: &str) -> String {
    let mut res = String::new();
    let mut count = String::new();
    for c in str.chars() {
        if c.is_numeric() {
            count.push(c);
        } else {
            res.push_str(&c.to_string().repeat(count.parse::<usize>().unwrap_or(1)));
            count.clear();
        }
    }
    res
}

fn latin_kids() {
    let n = "square";
    let res = latin_kids_string(n);
    let res = latin_kids_string_1(n);
    let res = translate_word(n);
    println!("{:?}", res);
}

use regex::Regex;

pub fn translate_word(word: &str) -> String {
    let vowel: Regex = Regex::new(r"^([aeiou]|y[^aeiou]|xr)[a-z]*").unwrap();
    let consonants: Regex = Regex::new(r"^([^aeiou]?qu|[^aeiou][^aeiouy]*)([a-z]*)").unwrap();

    if vowel.is_match(word) {
        String::from(word) + "ay"
    } else {
        let caps = consonants.captures(word).unwrap();
        String::from(&caps[2]) + &caps[1] + "ay"
    }
}

pub fn translate(input: &str) -> String {
    input
        .split(" ")
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}

const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
fn latin_kids_string_1(input: &str) -> String {
    input
        .split_whitespace()
        .map(anstray_latin)
        .collect::<Vec<_>>()
        .join(" ")
}

fn anstray_latin(word: &str) -> String {
    let mut i = if word.starts_with('y') {
        word.find(|c| "aeiou".contains(c)).unwrap()
    } else {
        word.find(|c| "aeiouy".contains(c)).unwrap()
    };

    if i >= 1 && &word[i - 1..=i] == "qu" {
        i += 1
    }

    if &word[..2] == "yt" || "ay" == &word[i..] {
        i = 0
    }

    format!("{}{}ay", &word[i..], &word[..i])
}

fn latin_kids_string(input: &str) -> String {
    match input {
        str if str.contains(char::is_whitespace) => {
            str.split_whitespace().map(latin_kids_string).join(" ")
        }
        str if str.starts_with(VOWEL) => push_ay(str),
        str if str.starts_with("xr") || str.starts_with("yt") => push_ay(str),
        str if !str.starts_with(VOWEL) && str.contains("qu") => latin_kids_rule_3(str, 2),
        str if !str.starts_with(VOWEL) && str.contains(VOWEL) => latin_kids_rule_2(str),
        str if !str.starts_with(VOWEL) && str.contains('y') => latin_kids_rule_3(str, 0),
        _ => todo!(),
    }
}

fn latin_kids_rule_3(str: &str, index_addition: usize) -> String {
    let index = str.find("qu").or(str.find('y')).unwrap();
    let s = str.split_at(index + index_addition);
    s.1.to_owned() + s.0 + "ay"
}

fn latin_kids_rule_2(str: &str) -> String {
    let index = str.chars().position(|c| VOWEL.contains(&c)).unwrap();
    let (consonant, other) = str.split_at(index);
    format!("{other}{consonant}ay")
}

fn push_ay(str: &str) -> String {
    str.to_owned() + "ay"
}

fn perfect_number() {
    let n = 1;
    let res = is_perfect_number(n);
    println!("{:?}", res);
}

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn is_perfect_number(number: u64) -> Option<Classification> {
    if number < 1 {
        return None;
    }
    let sum: u64 = (1..=number / 2).filter(|x| number % x == 0).sum();
    match sum.cmp(&number) {
        Ordering::Less => Some(Classification::Deficient),
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Abundant),
    }
}

fn alphametics() {
    let s = "I + BB == ILL";
    let res = solve_alphametics(s);
    println!("{:?}", res);
}

fn solve_alphametics(input: &str) -> Option<HashMap<char, u8>> {
    let firsts = input
        .split(&['+', '='])
        .filter_map(|s| s.trim().chars().next())
        .collect::<HashSet<_>>();
    let (letters, factors) = calc_factors(input);
    for perm in (0..=9).permutations(letters.len()) {
        let sum = perm
            .iter()
            .enumerate()
            .map(|(i, v)| v * factors.get(i).unwrap())
            .sum::<i64>();
        if sum == 0
            && !perm
                .iter()
                .enumerate()
                .any(|(i, v)| *v == 0 && firsts.contains(letters.get(i).unwrap()))
        {
            return Some(HashMap::from_iter(
                perm.iter()
                    .enumerate()
                    .map(|(i, v)| (*letters.get(i).unwrap(), *v as u8)),
            ));
        }
    }
    None
}

fn calc_factors(input: &str) -> (Vec<char>, Vec<i64>) {
    let mut factors = HashMap::new();
    let mut sign = -1;
    let mut pos = 0;
    for c in input.chars().filter(|c| !c.is_whitespace()).rev() {
        match c {
            '=' => {
                sign = 1;
                pos = 0
            }
            '+' => pos = 0,
            _ => {
                *factors.entry(c).or_insert(0) += sign * 10_i64.pow(pos);
                pos += 1;
            }
        }
    }
    factors.into_iter().sorted_by_key(|(_, v)| -v.abs()).unzip()
}

fn pangram() {
    let s = "\"Five quacking Zephyrs jolt my wax bed.\"";
    // let s = "the quick brown fox jumps over the lazy dog";
    let res = is_pangram(s);
    println!("{:?}", res);
}

fn is_pangram(sentence: &str) -> bool {
    let mut set = ('a'..='z').collect::<HashSet<_>>();
    sentence
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .for_each(|c| {
            set.remove(&c);
        });
    let _ = set.is_empty();
    let s = sentence.to_lowercase();
    ('a'..='z').all(|c| s.contains(c))
}

fn palindrome() {
    palindrome::call();
}
fn passio() {
    paas_io::call();
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
