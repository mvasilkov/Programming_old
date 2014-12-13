use std::io;

const SIZE: uint = 4;

fn pew(level: uint, placed: Vec<bool>, result: Vec<uint>) {
    if level == SIZE { return println!("{}", result) }

    for a in range(0, SIZE) {
        if placed[a] { continue }

        let new_level = level + 1;
        let new_placed = Vec::from_fn(SIZE, |i| i == a || placed[i]);
        let new_result = Vec::from_fn(new_level, |i| {
            if i == level { a } else { result[i] }
        });

        pew(new_level, new_placed, new_result);
    }
}

fn main() {
    let placed = Vec::from_elem(SIZE, false);

    pew(0, placed, Vec::<uint>::new());
}

#[allow(dead_code)]
fn print_binary() {
    println!("Rust implements println! as a macro rather than a function");

    let input = io::stdin()
        .read_line()
        .ok()
        .expect("What to do?");

    let input_num: Option<uint> = from_str::<uint>(input.trim());

    let num = match input_num {
        Some(x) => x,
        None => {
            println!("Uh, what?");
            return;
        },
    };

    println!("{:b}", num);
}
