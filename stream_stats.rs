use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::time::Instant;

static READ_BUF_SIZE: usize = 1024 * 1024;

struct Stats {
    started: Instant,
    lines: usize,
    tty: File,
}

impl Stats {
    fn new(tty: &str) -> Stats {
        Stats {
            started: Instant::now(),
            lines: 0,
            tty: OpenOptions::new()
                .write(true)
                .append(true)
                .open(tty)
                .expect("Cannot open tty for writing!"),
        }
    }
}

fn main() {
    let mut reader = BufReader::with_capacity(READ_BUF_SIZE, io::stdin());
    let mut writer = BufWriter::new(io::stdout());
    let mut buffer = vec![];
    let mut stats = Stats::new("/dev/tty");

    while reader.read_until(b'\n', &mut buffer).unwrap() > 0 {
        writer.write(&buffer).unwrap();
        stats.lines += 1;
        buffer.clear();
    }
    writer.flush().unwrap();
    writeln!(
        stats.tty,
        "lines: {}, {:?}",
        stats.lines,
        stats.started.elapsed()
    ).expect("Could not write to tty!");
}
