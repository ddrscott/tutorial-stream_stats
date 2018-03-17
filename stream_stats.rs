use std::io::{self, BufRead, BufReader, BufWriter, Write};

static READ_BUF_SIZE: usize = 1024 * 1024;

fn main() {
    let mut reader = BufReader::with_capacity(READ_BUF_SIZE, io::stdin());
    let mut writer = BufWriter::new(io::stdout());
    let mut buffer = vec![];

    while reader.read_until(b'\n', &mut buffer).unwrap() > 0 {
        writer.write(&buffer).unwrap();
        buffer.clear();
    }
    writer.flush().unwrap();
}
