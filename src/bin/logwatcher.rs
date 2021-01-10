use log_archiver::{
    LogWatcher,
    LogWatcherAction,
};

use flate2::write::GzEncoder;
use std::{
    env::args,
    fs::File,
    io::prelude::*,
    process::exit,
    sync::mpsc::channel,
    sync::mpsc::{
        Receiver,
        Sender,
    },
};

use flate2::Compression;

fn main() -> std::io::Result<()> {
    // let argsm: Vec<String> = args().collect();
    // ? ➜ ./target/release/logwatcher -s a -b c -d "sa" 123 dbc
    // ? ["./target/release/logwatcher", "-s", "a", "-b", "c", "-d", "sa",
    // "123", "dbc"]
    // println!("{:?}", argsm);
    let (tx, rx): (Sender<bool>, Receiver<bool>) = channel();

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        tx.send(true).unwrap();
    })
    .expect("Error setting Ctrl-C handler");

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

    while rx.recv().unwrap() {
        println!("Exiting app");
        write_to_file(&mut buffer, i).unwrap();

        i += 1;
        exit(0)
    }

    log_watcher.watch(&mut move |line: String| {
        buffer.push_str(line.as_str());
        // ? Buffer 100mb of log file to RAM
        if buffer.len() > 1024 * 1024 * 100 {
            if let Err(e) = write_to_file(&mut buffer, i) {
                println!("Error verdim ben {}", e)
            }

            i += 1;
        }

        LogWatcherAction::None
    });
    Ok(())
}

fn write_to_file(
    buffer: &mut String,
    i: i32,
) -> Result<(), String> {
    let output = File::create(format!("logs/log_archive_{}.gz", i)).unwrap();

    let mut gzip = GzEncoder::new(output, Compression::fast());

    // match gzip.write_all(buffer.as_bytes()) { Err(e) => { return
    // e.to_string() } }
    gzip.write_all(buffer.as_bytes())
        .map_err(|e| e.to_string())?;

    println!("Flush {} bytes", buffer.len());

    buffer.clear();
    gzip.finish().unwrap();
    Ok(())
}
