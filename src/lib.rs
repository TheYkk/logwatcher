use std::{
    fs::File,
    io::{
        BufReader,
        prelude::*,
        self,
        ErrorKind,
        SeekFrom
    },
    os::unix::fs::MetadataExt,
    path::Path,
    thread::sleep,
    time::Duration,
};

pub enum LogWatcherAction {
    None,
    SeekToEnd,
}

pub struct LogWatcher {
    filename: String,
    inode:    u64,
    pos:      u64,
    reader:   BufReader<File>,
    finish:   bool,
}

impl LogWatcher {
    pub fn register<P: AsRef<Path>>(
        filename: P
    ) -> Result<LogWatcher, io::Error> {
        let f = match File::open(&filename) {
            Ok(x) => x,
            Err(err) => return Err(err),
        };

        let metadata = match f.metadata() {
            Ok(x) => x,
            Err(err) => return Err(err),
        };

        let mut reader = BufReader::new(f);
        let pos = metadata.len();
        reader.seek(SeekFrom::Start(pos)).unwrap();
        Ok(LogWatcher {
            filename: filename.as_ref().to_string_lossy().to_string(),
            inode: metadata.ino(),
            pos,
            reader,
            finish: false,
        })
    }

    fn reopen_if_log_rotated<F: ?Sized>(
        &mut self,
        callback: &mut F,
    ) where
        F: FnMut(String) -> LogWatcherAction,
    {
        loop {
            match File::open(&self.filename) {
                Ok(x) => {
                    let f = x;
                    let metadata = match f.metadata() {
                        Ok(m) => m,
                        Err(_) => {
                            sleep(Duration::new(1, 0));
                            continue;
                        }
                    };
                    if metadata.ino() != self.inode {
                        self.finish = true;
                        self.watch(callback);
                        self.finish = false;
                        println!("reloading log file");
                        self.reader = BufReader::new(f);
                        self.pos = 0;
                        self.inode = metadata.ino();
                    } else {
                        sleep(Duration::new(1, 0));
                    }
                    break;
                }
                Err(err) => {
                    if err.kind() == ErrorKind::NotFound {
                        sleep(Duration::new(1, 0));
                        continue;
                    }
                }
            };
        }
    }

    pub fn watch<F: ?Sized>(
        &mut self,
        callback: &mut F,
    ) where
        F: FnMut(String) -> LogWatcherAction,
    {
        loop {
            let mut line = String::new();
            let resp = self.reader.read_line(&mut line);
            match resp {
                Ok(len) => {
                    if len > 0 {
                        self.pos += len as u64;
                        self.reader.seek(SeekFrom::Start(self.pos)).unwrap();
                        match callback(line.replace("\n", "")) {
                            LogWatcherAction::SeekToEnd => {
                                println!("SeekToEnd");
                                self.reader.seek(SeekFrom::End(0)).unwrap();
                            }
                            LogWatcherAction::None => {}
                        }
                        line.clear();
                    } else {
                        if self.finish {
                            break;
                        } else {
                            self.reopen_if_log_rotated(callback);
                            self.reader
                                .seek(SeekFrom::Start(self.pos))
                                .unwrap();
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }
}
