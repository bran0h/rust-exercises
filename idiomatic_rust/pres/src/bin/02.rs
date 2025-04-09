use std::io::Write;
use std::path::PathBuf;

// impl Write for PathBuf {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         todo!()
//     }

//     fn flush(&mut self) -> std::io::Result<()> {
//         todo!()
//     }
// }

struct FileLogger {
    log_path: PathBuf,
}
impl Write for FileLogger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
    /* - snip -*/
}

struct StdOutLogger;
impl Write for StdOutLogger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
    /* - snip -*/
}

fn log<L: Write + ?Sized>(entry: &str, logger: &mut L) -> Result<(), std::io::Error> {
    write!(logger, "{}", entry)
}

fn main() -> Result<(), std::io::Error> {
    let log_file: Option<PathBuf> = None;
    let logger: &mut dyn Write = match log_file {
        Some(log_path) => &mut FileLogger { log_path },
        None => &mut StdOutLogger,
    };

    log("Hello, world!🦀", logger)
}
