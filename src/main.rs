use rand::Rng;

fn main() {
    for _ in 0..100 {
        let mut input = rand_vec_gen(20);
        let sorted = sort::insertion_by(input.clone(), |a, b| a.partial_cmp(b).unwrap());
        assert_eq!(sorted, {
            input.sort();
            input
        });
    }
    println!("NO ERROR");
}

mod sort {
    use std::{
        cmp::Ordering,
        ops::{Add, Sub, SubAssign},
    };

    pub fn bubble<T>(mut vec: Vec<T>) -> Vec<T>
    where
        T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T> + SubAssign,
    {
        for i in 1..vec.len() {
            for j in 0..vec.len() - i {
                if vec[j] > vec[j + 1] {
                    vec.swap(j, j + 1);
                }
            }
        }
        vec
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
    pub fn insertion<T, F>(mut vec: Vec<T>) -> Vec<T>
    where
        T: Copy + PartialOrd,
    {
        'o: for i in 1..vec.len() {
            let key = vec[i];
            let mut j = i - 1;
            while key < vec[j] {
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
}

fn rand_vec_gen(limit: isize) -> Vec<isize> {
    let mut rng = rand::thread_rng();
    (0..limit)
        .into_iter()
        .map(|_| rng.gen_range(-20..20))
        .collect()
}
