use anyhow::Result;
use std::{thread::sleep, time::Duration};
use flutter_rust_bridge::StreamSink;

const ONE_SECOND: Duration = Duration::from_secs(1);

pub fn tick(sink: StreamSink<i32>) -> Result<()> {
    let mut ticks = 0;
    loop {
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