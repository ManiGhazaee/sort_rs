#[cfg(test)]
#[test]
fn test_all() {
    use sort::rand_vec_gen;

    let i = rand_vec_gen(256);
    let mut e = i.clone();
    e.sort();

    let mut i1 = i.clone();
    let mut i2 = i.clone();
    let mut i3 = i.clone();
    let mut i4 = i.clone();
    let mut i5 = i.clone();
    let mut i6 = i.clone();

    sort::merge(&mut i1);
    sort::heapsort(&mut i2);
    sort::quicksort(&mut i3);
    sort::insertion(&mut i4);
    sort::selection(&mut i5);
    sort::bubble(&mut i6);

    assert_eq!(i1, e);
    assert_eq!(i2, e);
    assert_eq!(i3, e);
    assert_eq!(i4, e);
    assert_eq!(i5, e);
    assert_eq!(i6, e);
}
