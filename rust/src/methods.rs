#![allow(unused_variables)]

use crate::models::{ActionRequest, ActionResponse};
use anyhow::Result;
use flutter_rust_bridge::*;
use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};



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
