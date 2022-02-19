use std::{env::args, fs::File, io::prelude::*, process::exit, sync::atomic::{Ordering, AtomicBool}, sync::Arc, thread};
use unbytify::unbytify;

use std::sync::Mutex;

use ctrlc;

use flate2::Compression;
use flate2::write::GzEncoder;

use log_archiver::{
    LogWatcher,
    LogWatcherAction,
};


fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("Shutting down...");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let filename = match args().nth(1) {
        Some(x) => x,
        None => {
            println!("File name required");
            exit(1);
        }
    };

    let file_size = match args().nth(2) {
        Some(x) => x,
        None => {
            println!("File size required");
            exit(1);
        }
    };

    let mut log_watcher = LogWatcher::register(filename)?;

    let buffer= Arc::new(Mutex::new(String::new()));
    let chunk_id= Arc::new(Mutex::new(0));

    let log_buffer = Arc::clone(&buffer);
    let chunk_id_log = Arc::clone(&chunk_id);
    thread::spawn(move || {
        log_watcher.watch(&mut |line: String| {
            let mut in_buffer = log_buffer.lock().unwrap();
            in_buffer.push_str(line.as_str());

            println!("Line size: {}", in_buffer.len());

            // ? Buffer 100mb of log file to RAM
            // if buffer.len() > 1024 * 1024 * 10 {
            if in_buffer.len() > unbytify(file_size.as_str()).unwrap() as usize {
                if let Err(e) = write_to_file(&in_buffer, &chunk_id_log.lock().unwrap()) {
                    println!("Error verdim ben {}", e)
                }

                in_buffer.clear();
                let mut ch = chunk_id_log.lock().unwrap();
                *ch += 1;
            }

            LogWatcherAction::None
        });
    });

    println!("Waiting for ctrl-c");

    while running.load(Ordering::SeqCst) { }

    println!("Exiting app");

    let a = buffer.lock().unwrap();
    write_to_file(&a, &chunk_id.lock().unwrap()).unwrap();
    return Ok(())
}

fn write_to_file(buffer: &String, chunk_id: &i32) -> Result<(), String> {
    if buffer.len() == 0 {
        return Ok(())
    }

    let output = File::create(format!("logs/log_archive_{}.gz", chunk_id)).unwrap();
    println!("Yaziyorum");
    let mut gzip = GzEncoder::new(output, Compression::fast());

    gzip.write_all(buffer.as_bytes()).unwrap();

    println!("Flush {} bytes", buffer.len());

    gzip.finish().unwrap();
    Ok(())
}
