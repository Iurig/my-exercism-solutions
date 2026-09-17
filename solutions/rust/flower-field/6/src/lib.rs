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

    garden
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut annotated_garden, (i, row)| {
            let annotated_row = row.as_bytes().iter().enumerate().fold(
                String::new(),
                |mut annotating_row, (j, c)| {
                    annotating_row.push(if c == &b'*' {
                        '*'
                    } else {
                        let flowers = garden[i.saturating_sub(1)..(i + 2).min(garden.len())]
                            .iter()
                            .fold(0u8, |mut count, line| {
                                count += line.as_bytes()
                                    [j.saturating_sub(1)..(j + 2).min(line.len())]
                                    .iter()
                                    .map(|c| u8::from(c == &b'*'))
                                    .sum::<u8>();
                                count
                            });
                        if flowers == 0 {
                            ' '
                        } else {
                            char::from(b'0' + flowers)
                        }
                    });
                    annotating_row
                },
            );
            annotated_garden.push(annotated_row);
            annotated_garden
        })
}
