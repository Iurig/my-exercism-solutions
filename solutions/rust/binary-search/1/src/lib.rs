pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let (mut l, mut r) = (0, array.len());
    dbg!(array, key);
    while l + 1 < r {
        let mid = (l + r) / 2;
        if array[mid] > key {
            r = mid;
        } else if array[mid] == key {
            return Some(mid);
        } else {
            l = mid;
        }
        dbg!(&l, &r);
    }
    match !array.is_empty() && array[l] == key {
        true => Some(l),
        false => None,
    }
}
