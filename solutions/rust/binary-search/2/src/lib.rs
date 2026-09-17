use itertools::Itertools;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    (0..)
        .fold_while((Some(0), Some(array.len())), |(some_l, some_r), _| {
            let (l, r) = (some_l.unwrap(), some_r.unwrap());
            if l + 1 >= r {
                match array.get(l) == Some(&key) {
                    true => itertools::FoldWhile::Done((Some(l), Some(r))),
                    false => itertools::FoldWhile::Done((None, None)),
                }
            } else {
                let mid = (l + r) / 2;
                match array[mid].cmp(&key) {
                    std::cmp::Ordering::Greater => {
                        itertools::FoldWhile::Continue((Some(l), Some(mid)))
                    }
                    std::cmp::Ordering::Less => {
                        itertools::FoldWhile::Continue((Some(mid), Some(r)))
                    }
                    std::cmp::Ordering::Equal => {
                        itertools::FoldWhile::Done((Some(mid), Some(mid + 1)))
                    }
                }
            }
        })
        .into_inner()
        .0
}
