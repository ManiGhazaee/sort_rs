use rand::Rng;

fn main() {
    for _ in 0..1000 {
        let mut input_1 = rand_vec_gen(rand::random());
        let mut input_2 = input_1.clone();
        sort::quicksort(&mut input_1);
        input_2.sort();
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
}

fn rand_vec_gen(limit: u8) -> Vec<isize> {
    let mut rng = rand::thread_rng();
    (0..limit)
        .into_iter()
        .map(|_| rng.gen_range(-100..100))
        .collect()
}
