use std::env::args;

fn main() {
    let name = args().nth(1).unwrap();

    match name.to_lowercase().trim() {
        "w1n4a" => println!("+1000000 AURA"),
        "linux" => println!("+1000000 AURA"),
        "arch" => println!("+10000000 AURA"),
        "windows" => println!("-10000 AURA"),
        _ => println!("{} AURA", count(&name)),
    }
}

fn count(name: &String) -> i16 {
    let mut res: i16 = 0;

    match name.trim().len() {
        0..=3 => res += 20,
        4..=6 => res += 15,
        _ => res -= 10,
    }

    for i in name.trim().chars() {
        match i {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => res += 15,
            x if x.is_uppercase() => res -= 2,
            x if x.is_lowercase() => res += 5,
            '_' | '/' | ')' | '(' | '-' | '!' | '?' | '*' | '@' | '`' | '~' => res += 20,
            ' ' => res -= 10,
            _ => {}
        }
    }

    return res;
}
