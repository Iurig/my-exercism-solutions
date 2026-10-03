/// # Panics
///
/// Expects two lines following the problem format, panics if `diagram` isn't ASCII,
/// but may fail silently or panic if the format is not followed in another way
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    assert!(diagram.is_ascii(), "diagram must be ASCII");
    let pos = 2 * (student.as_bytes()[0] - b'A') as usize;
    let diagram = diagram.as_bytes();
    let second_line = &diagram[diagram.len() / 2 + 1..];
    [
        diagram[pos],
        diagram[pos + 1],
        second_line[pos],
        second_line[pos + 1],
    ]
    .iter()
    .map(|c| match c {
        b'G' => "grass",
        b'C' => "clover",
        b'R' => "radishes",
        b'V' => "violets",
        _ => panic!(),
    })
    .collect()
}
