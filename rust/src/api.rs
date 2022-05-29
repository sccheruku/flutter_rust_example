use std::sync::atomic::{AtomicU64, Ordering, AtomicBool};
use anyhow::Result;
use flutter_rust_bridge::StreamSink;

pub mod tick;
pub mod controlled_stream;

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn get_counter() -> u64 {
    COUNTER.load(Ordering::SeqCst)
}

pub fn increment() -> u64 {
    COUNTER.fetch_add(1, Ordering::SeqCst);
    COUNTER.load(Ordering::SeqCst)
}

pub fn decrement() -> u64 {
    COUNTER.fetch_sub(1, Ordering::SeqCst);
    COUNTER.load(Ordering::SeqCst)
}

pub fn tick(sink: StreamSink<i32>) -> Result<()> {
    tick::tick(sink);
    Ok(())
}

pub fn start_stream(sink: StreamSink<i32>) -> Result<()>{
    controlled_stream::start();
    controlled_stream::tick(sink)
}
pub fn stop_stream(){
    controlled_stream::stop();
}
