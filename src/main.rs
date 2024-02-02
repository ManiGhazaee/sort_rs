use std::{collections::HashMap, time::Instant};

use sort::{empty_test, rand_vec_gen, sort_time};

const ITERATIONS: usize = 100;
const VEC_LENS: [usize; 4] = [4, 16, 256, 2_048];

fn main() {
    let t = [0.0; ITERATIONS];
    let mut map = HashMap::from([
        ("rust", t),
        ("merge", t),
        ("quicksort", t),
        ("insertion", t),
        ("bubble", t),
        ("selection", t),
        ("heapsort", t),
    ]);

    empty_test::<isize>(&[
        sort::merge,
        sort::quicksort,
        sort::insertion,
        sort::bubble,
        sort::selection,
        sort::heapsort,
    ]);

    for i in 0..ITERATIONS {
        let vec_len = VEC_LENS[i % VEC_LENS.len()];
        let input = rand_vec_gen(vec_len);
        let mut sorted = input.clone();

        let inst = Instant::now();
        sorted.sort();
        map.get_mut("rust").unwrap()[i] = inst.elapsed().as_secs_f64();

        sort_time(&mut map, "merge", &input, &sorted, i, sort::merge);
        sort_time(&mut map, "quicksort", &input, &sorted, i, sort::quicksort);
        sort_time(&mut map, "insertion", &input, &sorted, i, sort::insertion);
        sort_time(&mut map, "bubble", &input, &sorted, i, sort::bubble);
        sort_time(&mut map, "selection", &input, &sorted, i, sort::selection);
        sort_time(&mut map, "heapsort", &input, &sorted, i, sort::heapsort);
    }

    for (_, val) in map.iter_mut() {
        val[0] = val.iter().map(|i| *i).sum::<f64>() / ITERATIONS as f64;
    }

    let mut times: Vec<(&str, f64)> = map.iter().map(|(key, val)| (*key, val[0])).collect();
    times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for (name, time) in times {
        println!("{:<20} {:0<26} {}", name, time, "s");
    }
}
