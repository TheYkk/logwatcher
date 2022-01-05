use std::{
    env::args,
    fs::File,
    io::prelude::*,
    process::exit,
    sync::mpsc::{
        Receiver,
        Sender,
    },
    sync::mpsc::channel,
};

use flate2::Compression;
use flate2::write::GzEncoder;

use log_archiver::{
    LogWatcher,
    LogWatcherAction,
};
use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;



fn main() -> std::io::Result<()> {
    // let argsm: Vec<String> = args().collect();
    // ? ➜ ./target/release/logwatcher -s a -b c -d "sa" 123 dbc
    // ? ["./target/release/logwatcher", "-s", "a", "-b", "c", "-d", "sa",
    // "123", "dbc"]
    // println!("{:?}", argsm);
    // let (tx, rx) = std::sync::mpsc::channel();

    let running = Arc::new(AtomicBool::new(false));
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            println!("Ctrl-C detected");
            r.store(true, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");

        while running.load(Ordering::SeqCst) {
            println!("Exiting app");
            // write_to_file(&buffer.clone(), &i);
            exit(0)
        }
    }
    // let filename = match args().nth(1) {
    //     Some(x) => x,
    //     None => {
    //         println!("File name required");
    //         exit(1);
    //     }
    // };

    // // let mut log_watcher = LogWatcher::register(filename)?;
    // //
    // // let mut buffer = String::from("");
    // //
    // // let mut i = 0;
    // //
    // // log_watcher.watch(&mut  |line: String| {
    // //     buffer.push_str(line.as_str());
    // //     println!("{}", line.as_str());
    // //     println!("{}", buffer.len());
    // //
    // //     // ? Buffer 100mb of log file to RAM
    // //     // if buffer.len() > 1024 * 1024 * 10 {
    // //     if buffer.len() > 10 {
    // //         if let Err(e) = write_to_file(&buffer.clone(), &i) {
    // //             println!("Error verdim ben {}", e)
    // //         }
    // //         buffer.clear();
    // //         i += 1;
    // //     }
    // //
    // //     LogWatcherAction::None
    // // });

    // while SHUTDOWN.load(Ordering::Relaxed) {
    //     println!("Exiting app");
    //     // write_to_file(&buffer.clone(), &i);
    //     exit(0)
    // }
    sleep(Duration::from_secs(200));
    Ok(())
}

// fn write_to_file(buffer: &String, i: &i32) -> Result<(), String> {
//     let output = File::create(format!("logs/log_archive_{}.gz", i)).unwrap();
//     println!("Yaziyorum");
//     let mut gzip = GzEncoder::new(output, Compression::fast());
//
//     // match gzip.write_all(buffer.as_bytes()) { Err(e) => { return
//     // e.to_string() } }
//     gzip.write_all(buffer.as_bytes())
//         .map_err(|e| e.to_string())?;
//
//     println!("Flush {} bytes", buffer.len());
//
//     gzip.finish().unwrap();
//     Ok(())
// }
