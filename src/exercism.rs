#![allow(dead_code, clippy::new_without_default)]
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

pub fn call() {
    println!("namah prabhu /\\ /\\");
    // binary_search();
    // etl();
    grade_school();
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
