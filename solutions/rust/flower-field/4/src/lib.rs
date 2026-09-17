pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut anotated_garden, (i, row)| {
            let anotated_row =
                row.chars()
                    .enumerate()
                    .fold(Vec::new(), |mut anotating_row, (j, c)| {
                        anotating_row.push(if c == '*' {
                            b'*'
                        } else {
                            let flowers = garden[i.saturating_sub(1)..(i + 2).min(garden.len())]
                                .iter()
                                .fold(0u8, |mut count, line| {
                                    count += line[j.saturating_sub(1)..(j + 2).min(row.len())]
                                        .chars()
                                        .filter(|c| c == &'*')
                                        .count() as u8;
                                    count
                                });
                            if flowers == 0 { b' ' } else { flowers + b'0' }
                        });
                        anotating_row
                    });
            anotated_garden.push(String::from_utf8(anotated_row).unwrap());
            anotated_garden
        })
}
