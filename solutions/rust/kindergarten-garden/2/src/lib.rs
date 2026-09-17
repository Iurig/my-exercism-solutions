/// # Panics
///
/// Expects two lines following the problem format, panics if `diagram` isn't ASCII,
/// but may fail silently or panic if the format is not followed in another way
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    assert!(diagram.is_ascii(), "diagram must be ASCII");
    let pos = 2 * (student.as_bytes()[0] - b'A') as usize;
    let second_line = &diagram[diagram.find('\n').unwrap() + 1..];
    diagram.as_bytes()[(pos)..(pos + 2)]
        .iter()
        .chain(second_line.as_bytes()[(pos)..(pos + 2)].iter())
        .map(|c| match c {
            b'G' => "grass",
            b'C' => "clover",
            b'R' => "radishes",
            b'V' => "violets",
            _ => panic!(),
        })
        .collect()
}
