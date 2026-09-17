pub fn find(array: &[i32], key: i32) -> Option<usize> {
    (0..(array.len().checked_ilog2().unwrap_or_default() + 2))
        .try_fold((0, array.len()), |(l, r), _| {
            println!("({l}, {r})");
            if l + 1 >= r {
                match array.get(l) == Some(&key) {
                    true => Some((l, r)),
                    false => None,
                }
            } else {
                let mid = (l + r) / 2;
                match array[mid].cmp(&key) {
                    std::cmp::Ordering::Greater => Some((l, mid)),
                    std::cmp::Ordering::Less => Some((mid, r)),
                    std::cmp::Ordering::Equal => Some((mid, mid + 1)),
                }
            }
        })
        .map(|(i, _)| i)
}
