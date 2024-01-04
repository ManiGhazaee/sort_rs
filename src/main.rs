use sort::rand_vec_gen;

fn main() {
    for _ in 0..1000 {
        let mut i1 = rand_vec_gen(rand::random());
        let mut i2 = i1.clone();
        let mut i3 = i1.clone();
        let mut i4 = i1.clone();
        let mut i5 = i1.clone();

        i1.sort();

        sort::merge(&mut i2);
        sort::quicksort(&mut i3);
        sort::insertion(&mut i4);
        sort::bubble(&mut i5);

        assert_eq!(i1, i2);
        assert_eq!(i1, i3);
        assert_eq!(i1, i4);
        assert_eq!(i1, i5);
    }
    println!("NO ERROR");
}
