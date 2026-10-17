fn main() {
    let x = 250u8;
    let y = 8;
    println!("{}", add(x, y));
}

fn add(a: u8, b: u8) -> u8 {
    if let Some(result) = a.checked_add(b) {
        return result;
    } else {
        return 0;
    }
}
