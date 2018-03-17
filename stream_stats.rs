use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::{self, sleep};
use std::time::{Duration, Instant};

static READ_BUF_SIZE: usize = 1024 * 1024;
static CLEAR_LINE: &str = "\x1B[1G\x1B[2K";
static UPDATE_INTERVAL_MS: u64 = 100;

struct Stats {
    started: Instant,
    lines: AtomicUsize,
    bytes: AtomicUsize,
    tty: File,
}

impl Stats {
    fn new(tty: &str) -> Stats {
        Stats {
            started: Instant::now(),
            lines: AtomicUsize::new(0),
            bytes: AtomicUsize::new(0),
            tty: OpenOptions::new()
                .write(true)
                .append(true)
                .open(tty)
                .expect("Cannot open tty for writing!"),
        }
    }

    fn add(&self, buffer: &Vec<u8>) {
        self.lines.fetch_add(1, Ordering::Relaxed);
        self.bytes.fetch_add(buffer.len(), Ordering::Relaxed);
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let elapsed = self.started.elapsed();
        let seconds: f64 = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
        if seconds == 0.0 {
            return write!(f, "");
        }
        let bytes = self.bytes.load(Ordering::Relaxed) as f64;
        let lines = self.lines.load(Ordering::Relaxed) as f64;
        let kb = bytes / 1024 as f64;
        let kb_per_sec = kb / seconds;
        let lines_per_sec = lines / seconds;
        write!(
            f,
            "{}{:.1} sec | {:.0} kb [ {:.1}/s ] | {:.0} lines [ {:.0}/s ]",
            CLEAR_LINE,
            seconds,
            kb,
            kb_per_sec,
            lines,
            lines_per_sec
        )
    }
}

fn main() {
    let mut reader = BufReader::with_capacity(READ_BUF_SIZE, io::stdin());
    let mut writer = BufWriter::new(io::stdout());
    let mut buffer = vec![];
    let stats = Arc::new(Stats::new("/dev/tty"));

    let stats_clone = stats.clone();
    thread::spawn(move || loop {
        sleep(Duration::from_millis(UPDATE_INTERVAL_MS));
        write!(&stats_clone.tty, "{}", &stats_clone).expect("Could not write to tty!");
    });

    while reader.read_until(b'\n', &mut buffer).unwrap() > 0 {
        writer.write(&buffer).unwrap();
        stats.add(&buffer);
        buffer.clear();
    }
    writer.flush().unwrap();
    writeln!(&stats.tty, "{}", &stats).expect("Could not write to tty!");
}
