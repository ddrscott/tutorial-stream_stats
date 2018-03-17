use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::time::Instant;

static READ_BUF_SIZE: usize = 1024 * 1024;
static CLEAR_LINE: &str = "\x1B[1G\x1B[2K";

struct Stats {
    started: Instant,
    lines: usize,
    bytes: usize,
    tty: File,
}

impl Stats {
    fn new(tty: &str) -> Stats {
        Stats {
            started: Instant::now(),
            lines: 0,
            bytes: 0,
            tty: OpenOptions::new()
                .write(true)
                .append(true)
                .open(tty)
                .expect("Cannot open tty for writing!"),
        }
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let elapsed = self.started.elapsed();
        let seconds: f64 = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
        if seconds == 0.0 {
            return write!(f, "");
        }
        let kb = self.bytes as f64 / 1024 as f64;
        let kb_per_sec = kb / seconds;
        let lines_per_sec = self.lines as f64 / seconds;
        write!(
            f,
            "{}{:.1} sec | {:.0} kb [ {:.1}/s ] | {} lines [ {:.0}/s ]",
            CLEAR_LINE,
            seconds,
            kb,
            kb_per_sec,
            self.lines,
            lines_per_sec
        )
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
        stats.bytes += &buffer.len();
        buffer.clear();
    }
    writer.flush().unwrap();
    writeln!(&stats.tty, "{}", &stats).expect("Could not write to tty!");
}
