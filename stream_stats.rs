use std::io;

fn main() {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).unwrap() > 0 {
        print!("{}", buffer);
        buffer.clear();
    }
}
