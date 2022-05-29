use std::{sync::atomic::{AtomicBool, Ordering}, thread::sleep, time::Duration};
use anyhow::Result;
use flutter_rust_bridge::StreamSink;

static state: AtomicBool = AtomicBool::new(false);
const ONE_SECOND: Duration = Duration::from_secs(1);

pub fn start() {
    state.store(true, Ordering::SeqCst);
}

pub fn stop() {
    state.store(false, Ordering::SeqCst);
}


pub fn tick(sink: StreamSink<i32>) -> Result<()> {
    let mut ticks = 0;
    if !state.load(Ordering::SeqCst) {
        panic!("Cannot start timer")
    }

    loop {
        
        // allow stopping the stream
        if !state.load(Ordering::SeqCst) {
            sink.close();
            return Ok(());
        }

        sink.add(ticks);
        sleep(ONE_SECOND);
        if ticks == i32::MAX {
            break;
        }
        ticks += 1;
        println!("tick");
    }
    Ok(())
}