pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut anotated_garden: Vec<Vec<u8>> = garden
        .iter()
        .map(|&line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let directions = [
        (0, 1),
        (1, 0),
        (0, -1isize),
        (-1isize, 0),
        (1, 1),
        (-1isize, 1),
        (1, -1isize),
        (-1isize, -1isize),
    ];

    for i in 0..anotated_garden.len() {
        for j in 0..anotated_garden[0].len() {
            let mut flowers: u8 = b'0';
            for d in directions {
                if i.checked_add_signed(d.0)
                    .map_or(false, |sum| sum < anotated_garden.len())
                    && j.checked_add_signed(d.1)
                        .map_or(false, |sum| sum < anotated_garden[0].len())
                {
                    if anotated_garden[i.strict_add_signed(d.0)][j.strict_add_signed(d.1)] == b'*' {
                        flowers += 1;
                    }
                }
                if flowers != b'0' && anotated_garden[i][j] != b'*' {
                    anotated_garden[i][j] = flowers;
                }
            }
        }
    }
    return anotated_garden
        .into_iter()
        .map(|line| String::from_utf8(line).unwrap())
        .collect::<Vec<String>>();
}
