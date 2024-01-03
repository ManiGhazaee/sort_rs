use std::time::Instant;

use rand::Rng;

fn main() {
    for _ in 0..1000 {
        let input_1 = rand_vec_gen(rand::random());
        let mut input_2 = input_1.clone();
        let inst = Instant::now();
        let input_1 = sort::merge(input_1);
        let el1 = inst.elapsed().as_micros();
        let inst = Instant::now();
        input_2.sort();
        let el2 = inst.elapsed().as_micros();
        assert_eq!(input_1, input_2);
        println!("me: {}\nrust: {}", el1, el2);
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
    pub fn merge_by<T, F>(vec: Vec<T>, compare: F) -> Vec<T>
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering,
    {
        if vec.len() <= 1 {
            return vec;
        }
        merge_by_(vec, &compare)
    }
    pub fn merge_by_<T, F>(mut vec: Vec<T>, compare: &F) -> Vec<T>
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering,
    {
        let vec_len = vec.len();
        if vec_len == 1 {
            return vec;
        } else if vec_len == 2 {
            if let Ordering::Less = compare(&vec[1], &vec[0]) {
                vec.swap(0, 1);
            }
            return vec;
        } else {
            let mid = vec_len / 2;
            let left = merge_by_(vec[0..mid].to_owned(), compare);
            let right = merge_by_(vec[mid..].to_owned(), compare);
            let mut res = Vec::with_capacity(vec_len);
            let mut il = 0;
            let mut ir = 0;
            while il < left.len() && ir < right.len() {
                if let Ordering::Less = compare(&right[ir], &left[il]) {
                    res.push(right[ir]);
                    ir += 1;
                } else {
                    res.push(left[il]);
                    il += 1;
                }
            }
            res.extend_from_slice(&left[il..]);
            res.extend_from_slice(&right[ir..]);
            return res;
        }
    }
    pub fn merge<T>(vec: Vec<T>) -> Vec<T>
    where
        T: Copy + PartialOrd,
    {
        merge_by(vec, |a, b| a.partial_cmp(b).unwrap())
    }
    ///////////////////////
    pub fn merge_sort<T: PartialOrd + Copy>(input: &mut [T]) {
        if input.len() < 2 {
            return;
        }

        let len = input.len();
        let mid = len / 2;
        merge_sort(&mut input[..mid]);
        merge_sort(&mut input[mid..]);

        let mut tmp = Vec::with_capacity(len);
        let mut i = 0;
        let mut j = mid;

        while i < mid && j < len {
            if input[i] < input[j] {
                tmp.push(input[i]);
                i += 1;
            } else {
                tmp.push(input[j]);
                j += 1;
            }
        }
        if i < mid {
            tmp.extend_from_slice(&input[i..mid]);
        } else if j < len {
            tmp.extend_from_slice(&input[j..len]);
        }

        input.copy_from_slice(&tmp[..]);
    }

    fn merge_x<T: PartialOrd + Copy>(in1: &[T], in2: &[T], tmp: &mut [T]) {
        let mut left = 0;
        let mut right = 0;
        let mut index = 0;

        while left < in1.len() && right < in2.len() {
            if in1[left] <= in2[right] {
                tmp[index] = in1[left];
                index += 1;
                left += 1;
            } else {
                tmp[index] = in2[right];
                index += 1;
                right += 1;
            }
        }

        if left < in1.len() {
            tmp[index..].copy_from_slice(&in1[left..]);
        }
        if right < in2.len() {
            tmp[index..].copy_from_slice(&in2[right..]);
        }
    }
}

fn rand_vec_gen(limit: u8) -> Vec<isize> {
    let mut rng = rand::thread_rng();
    (0..limit)
        .into_iter()
        .map(|_| rng.gen_range(-100..100))
        .collect()
}
