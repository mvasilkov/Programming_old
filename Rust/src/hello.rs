use std::io;

fn main() {
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
