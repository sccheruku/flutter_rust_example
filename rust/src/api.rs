#![allow(unused_variables)]

use crate::models::{ActionRequest, ActionResponse};
use anyhow::Result;
use flutter_rust_bridge::*;
use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};

mod controlled_stream;
mod tick;
mod person;

pub fn call(request: ActionRequest) -> ActionResponse {
    match request.action.as_str() {
        "echo" => echo(request),
        "get_person" => get_person(),
        // starts_with("person.") => Person::call(request) // for things like person.get person.save person.list etc
        _ => panic!("Unsupported action: {}", request.action),
    }
}

fn get_person() -> ActionResponse {
    let person = person::get_person();
    let j = json!(person);
    let person_json = j.to_string();
    ActionResponse { 
        success: true,
        response: person_json
    }
}

fn echo(request: ActionRequest) -> ActionResponse {
    ActionResponse {
        response: request.payload,
        success: true,
    }
}

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

pub fn start_stream(sink: StreamSink<i32>) -> Result<()> {
    controlled_stream::start();
    controlled_stream::tick(sink)
}
pub fn stop_stream() {
    controlled_stream::stop();
}
