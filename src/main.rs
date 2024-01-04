use std::time::Instant;

use rand::Rng;

fn main() {
    for _ in 0..1000 {
        let mut input_1 = rand_vec_gen(rand::random());
        let mut input_2 = input_1.clone();

        let inst = Instant::now();
        sort::merge(&mut input_1);
        let _1 = inst.elapsed().as_micros();

        let inst = Instant::now();
        input_2.sort();
        let _2 = inst.elapsed().as_micros();

        assert_eq!(input_1, input_2);
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
    pub fn quicksort_by<T, F>(vec: &mut Vec<T>, compare: F)
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering,
    {
        if vec.len() == 0 {
            return;
        }
        quicksort_by_(vec, &compare, 0, (vec.len() - 1) as isize);
    }
    fn partition<T, F>(vec: &mut Vec<T>, compare: &F, start: isize, end: isize) -> isize
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering,
    {
        let p = vec[end as usize];
        let mut i: isize = start - 1;
        for j in start..end {
            if let Ordering::Less | Ordering::Equal = compare(&vec[j as usize], &p) {
                i += 1;
                vec.swap(i as usize, j as usize);
            };
        }
        vec.swap((i + 1) as usize, end as usize);
        return i + 1;
    }
    fn quicksort_by_<T, F>(vec: &mut Vec<T>, compare: &F, start: isize, end: isize)
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering,
    {
        if start >= end {
            return;
        }
        let new_p = partition(vec, compare, start, end);
        quicksort_by_(vec, compare, start, new_p - 1);
        quicksort_by_(vec, compare, new_p + 1, end);
    }
    pub fn quicksort<T>(vec: &mut Vec<T>)
    where
        T: Copy + PartialOrd,
    {
        quicksort_by(vec, |a, b| a.partial_cmp(b).unwrap());
    }

    //////////////////////////////////////////////////
    /// Merge
    //////////////////////////////////////////////////
    pub fn merge_by<T, F>(input: &mut [T], compare: F)
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering,
    {
        merge_by_(input, &compare);
    }
    pub fn merge_by_<T, F>(input: &mut [T], compare: &F)
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering,
    {
        let input_len = input.len();
        if input_len <= 1 {
            return;
        } else if input_len == 2 {
            if let Ordering::Less = compare(&input[1], &input[0]) {
                input.swap(0, 1);
            }
        } else {
            let mid = input_len / 2;
            merge_by_(&mut input[..mid], compare);
            merge_by_(&mut input[mid..], compare);
            let mut res = Vec::with_capacity(input_len);
            let mut il = 0;
            let mut ir = mid;
            while il < mid && ir < input_len {
                if let Ordering::Less = compare(&input[ir], &input[il]) {
                    res.push(input[ir]);
                    ir += 1;
                } else {
                    res.push(input[il]);
                    il += 1;
                }
            }
            if il < mid {
                res.extend_from_slice(&input[il..mid]);
            } else if ir < input_len {
                res.extend_from_slice(&input[ir..input_len]);
            }

            input.copy_from_slice(&res[..]);
        }
    }
    pub fn merge<T>(vec: &mut [T])
    where
        T: Copy + PartialOrd,
    {
        merge_by(vec, |a, b| a.partial_cmp(b).unwrap());
    }
}

fn rand_vec_gen(limit: u8) -> Vec<isize> {
    let mut rng = rand::thread_rng();
    (0..limit)
        .into_iter()
        .map(|_| rng.gen_range(-100..100))
        .collect()
}
