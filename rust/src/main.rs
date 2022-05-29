use flutter_rust_bridge::{StreamSink};
use flutter_rust_bridge::rust2dart::Rust2Dart;

mod api;

pub fn main(){
    let result = api::tick::tick(StreamSink::new(Rust2Dart::new(9000)));
}