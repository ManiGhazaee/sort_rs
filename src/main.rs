use rand::Rng;

use crate::sort::insertion;

fn main() {
    for _ in 0..1000 {
        let mut input = rand_vec_gen(rand::random());
        let input_clone = input.clone();
        assert_eq!(insertion(input_clone), {
            input.sort();
            input
        });
    }
    println!("NO ERROR");
}

#[allow(dead_code)]
mod sort {
    use std::cmp::Ordering;

    //////////////////////////////////////////////////
    /// Bubble
    //////////////////////////////////////////////////
    pub fn bubble<T>(vec: Vec<T>) -> Vec<T>
    where
        T: Copy + PartialOrd,
    {
        bubble_by(vec, |a, b| a.partial_cmp(b).unwrap())
    }
    pub fn bubble_by<T, F>(mut vec: Vec<T>, compare: F) -> Vec<T>
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering,
    {
        for i in 1..vec.len() {
            for j in 0..vec.len() - i {
                if compare(&vec[j], &vec[j + 1]) == Ordering::Greater {
                    vec.swap(j, j + 1);
                }
            }
        }
        vec
    }

    //////////////////////////////////////////////////
    /// Insertion
    //////////////////////////////////////////////////
    pub fn insertion_by<T, F>(mut vec: Vec<T>, compare: F) -> Vec<T>
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering,
    {
        'o: for i in 1..vec.len() {
            let key = vec[i];
            let mut j = i - 1;
            while compare(&key, &vec[j]) == Ordering::Less {
                vec[j + 1] = vec[j];
                if j == 0 {
                    vec[j] = key;
                    continue 'o;
                }
                j -= 1;
            }
            vec[j + 1] = key;
        }
        vec
    }
    pub fn insertion<T>(vec: Vec<T>) -> Vec<T>
    where
        T: Copy + PartialOrd,
    {
        insertion_by(vec, |a, b| a.partial_cmp(b).unwrap())
    }

    //////////////////////////////////////////////////
    /// Quicksort
    //////////////////////////////////////////////////
    pub fn quick_by<T, F>(vec: &mut Vec<T>, compare: F)
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering,
    {
        if vec.len() == 0 {
            return;
        }
        quick_by_(vec, &compare, 0, vec.len() - 1);
    }
    fn partition<T, F>(vec: &mut Vec<T>, compare: &F, start: usize, end: usize) -> usize
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering,
    {
        let p = vec[end];
        let mut i: isize = start as isize - 1;
        for j in start..end {
            if let Ordering::Less | Ordering::Equal = compare(&vec[j], &p) {
                i += 1;
                vec.swap(i as usize, j);
            };
        }
        vec.swap((i + 1) as usize, end);
        return (i + 1) as usize;
    }
    fn quick_by_<T, F>(vec: &mut Vec<T>, compare: &F, start: usize, end: usize)
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering,
    {
        if start >= end {
            return;
        }
        let new_p = partition(vec, compare, start, end);
        if new_p > 0 {
            quick_by_(vec, compare, start, new_p - 1);
        }
        quick_by_(vec, compare, new_p + 1, end);
    }
    pub fn quick<T>(vec: &mut Vec<T>)
    where
        T: Copy + PartialOrd,
    {
        quick_by(vec, |a, b| a.partial_cmp(b).unwrap());
    }
}

fn rand_vec_gen(limit: u8) -> Vec<isize> {
    let mut rng = rand::thread_rng();
    (0..limit)
        .into_iter()
        .map(|_| rng.gen_range(-100..100))
        .collect()
}
