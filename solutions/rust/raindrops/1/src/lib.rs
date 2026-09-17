pub fn raindrops(n: u32) -> String {
    let sounds = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let mut resp = String::new();
    for (i, s) in sounds {
        if n.is_multiple_of(i) {
            resp += s;
        }
    }
    if resp.is_empty() {
        resp = n.to_string();
    }
    resp
}
