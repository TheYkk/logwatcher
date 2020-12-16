use logwatcher::{
    LogWatcher,
    LogWatcherAction,
};

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

fn main() -> std::io::Result<()> {
    let filename = match args().nth(1) {
        Some(x) => x,
        None => {
            println!("File name required");
            exit(1);
        }
    };

    let mut log_watcher = LogWatcher::register(filename)?;

    let mut buffer = String::from("");

    let mut i = 0;
    log_watcher.watch(&mut move |line: String| {
        buffer.push_str(line.as_str());
        // ?
        if buffer.len() > 1024 {
            let output = File::create(format!("as_{}.gz", i)).unwrap();

            let mut gzip = GzEncoder::new(output, Compression::default());

            let _ = gzip
                .write_all(buffer.as_bytes())
                .map(|_| {
                    println!("OK");
                })
                .map_err(|e| {
                    println!("Hata: {}", e.to_string());
                });

            println!("Flush {} bytes", buffer.len());

            buffer.clear();
            i += 1;
            gzip.finish().unwrap();
        }
        LogWatcherAction::SeekToEnd
    });
    Ok(())
}
