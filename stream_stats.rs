use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::time::Instant;

static READ_BUF_SIZE: usize = 1024 * 1024;

struct Stats {
    started: Instant,
    lines: usize,
}

fn main() {
    let mut reader = BufReader::with_capacity(READ_BUF_SIZE, io::stdin());
    let mut writer = BufWriter::new(io::stdout());
    let mut buffer = vec![];

    let mut stats = Stats {
        started: Instant::now(),
        lines: 0,
    };

    while reader.read_until(b'\n', &mut buffer).unwrap() > 0 {
        writer.write(&buffer).unwrap();
        stats.lines += 1;
        buffer.clear();
    }
    writer.flush().unwrap();
    eprintln!("lines: {}, {:?}", stats.lines, stats.started.elapsed());
}
