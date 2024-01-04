use std::cmp::Ordering;

use rand::Rng;

pub fn bubble<T>(input: &mut [T])
where
    T: Copy + PartialOrd,
{
    bubble_by(input, |a, b| a.partial_cmp(b).unwrap());
}
pub fn bubble_by<T, F>(input: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in 1..input.len() {
        for j in 0..input.len() - i {
            if compare(&input[j], &input[j + 1]) == Ordering::Greater {
                input.swap(j, j + 1);
            }
        }
    }
}

//////////////////////////////////////////////////
/// Insertion
//////////////////////////////////////////////////
pub fn insertion_by<T, F>(input: &mut [T], compare: F)
where
    T: Copy,
    F: Fn(&T, &T) -> Ordering,
{
    if input.len() == 0 {
        return;
    }
    'o: for i in 1..input.len() {
        let key = input[i];
        let mut j = i - 1;
        while compare(&key, &input[j]) == Ordering::Less {
            input[j + 1] = input[j];
            if j == 0 {
                input[j] = key;
                continue 'o;
            }
            j -= 1;
        }
        input[j + 1] = key;
    }
}
pub fn insertion<T>(vec: &mut [T])
where
    T: Copy + PartialOrd,
{
    insertion_by(vec, |a, b| a.partial_cmp(b).unwrap());
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
    _quicksort_by(vec, &compare, 0, (vec.len() - 1) as isize);
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
fn _quicksort_by<T, F>(vec: &mut Vec<T>, compare: &F, start: isize, end: isize)
where
    T: Copy,
    F: Fn(&T, &T) -> Ordering,
{
    if start >= end {
        return;
    }
    let new_p = partition(vec, compare, start, end);
    _quicksort_by(vec, compare, start, new_p - 1);
    _quicksort_by(vec, compare, new_p + 1, end);
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
    _merge_by(input, &compare);
}
fn _merge_by<T, F>(input: &mut [T], compare: &F)
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
        _merge_by(&mut input[..mid], compare);
        _merge_by(&mut input[mid..], compare);
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

pub fn rand_vec_gen(limit: u8) -> Vec<isize> {
    let mut rng = rand::thread_rng();
    (0..limit)
        .into_iter()
        .map(|_| rng.gen_range(-100..100))
        .collect()
}
