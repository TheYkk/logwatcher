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
    thread::sleep,
    time,
};

use flate2::Compression;

fn main() -> std::io::Result<()> {
    let argsm: Vec<String> = args().collect();
    // ? ➜ ./target/release/logwatcher -s a -b c -d "sa" 123 dbc
    // ? ["./target/release/logwatcher", "-s", "a", "-b", "c", "-d", "sa",
    // "123", "dbc"]
    println!("{:?}", argsm);
    let (tx, mut rx): (Sender<bool>, Receiver<bool>) = channel();

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        tx.send(true).unwrap();

        // sleep(time::Duration::from_secs(2));

        exit(0)
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
    log_watcher.watch(&mut move |line: String| {
        buffer.push_str(line.as_str());

        // ? Buffer 100mb of log file to RAM
        if buffer.len() > 1024 * 1024 * 100 {
            let output =
                File::create(format!("logs/log_archive_{}.gz", i)).unwrap();

            let mut gzip = GzEncoder::new(output, Compression::fast());

            gzip.write_all(buffer.as_bytes()).unwrap();

            println!("Flush {} bytes", buffer.len());

            buffer.clear();
            i += 1;
            gzip.finish().unwrap();
        }

        // ? Handle ctrl+c action and save buffer to file
        while rx.recv().unwrap() {}
        LogWatcherAction::None
    });
    Ok(())
}
