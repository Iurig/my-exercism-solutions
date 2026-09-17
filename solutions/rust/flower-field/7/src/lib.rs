/// # Panics
///
/// Panics if the input is not ASCII or not rectangular
#[must_use]
pub fn annotate(garden: &[&str]) -> Vec<String> {
    assert!(garden.iter().all(|r| r.is_ascii()), "garden must be ASCII");
    assert!(
        garden
            .windows(2)
            .all(|line_pair| line_pair[0].len() == line_pair[1].len()),
        "garden must be rectangular"
    );

    garden.iter().enumerate().fold(
        Vec::with_capacity(garden.len()),
        |mut annotated_garden, (i, row)| {
            let annotated_row =
                row.bytes()
                    .enumerate()
                    .fold(String::new(), |mut annotating_row, (j, c)| {
                        annotating_row.push(if c == b'*' {
                            '*'
                        } else {
                            let flowers = garden[i.saturating_sub(1)..(i + 2).min(garden.len())]
                                .iter()
                                .flat_map(|line| {
                                    line.as_bytes()[j.saturating_sub(1)..(j + 2).min(line.len())]
                                        .iter()
                                })
                                .filter(|&c| c == &b'*')
                                .count();
                            if flowers == 0 {
                                ' '
                            } else {
                                char::from(b'0' + u8::try_from(flowers).unwrap())
                            }
                        });
                        annotating_row
                    });
            annotated_garden.push(annotated_row);
            annotated_garden
        },
    )
}
