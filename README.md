# flutter_rust_example

This is a counter app build in flutter the implementation coming from rust. It uses dart:ffi library to call functions in rust, and uses the [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) package.


## References: 
https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU64.html
http://cjycode.com/flutter_rust_bridge/tutorial_with_flutter.html
https://github.com/Desdaemon/flutter_rust_bridge_template
https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/frb_example/with_flutter


## Run the project
> flutter run

## Generate dart bindings
flutter_rust_bridge_codegen --rust-input rust/src/api.rs --dart-output lib/bridge_generated.dart

## Youtube walkthrough
[![Tutorial](https://img.youtube.com/vi/oRahosxToxA/0.jpg)](https://www.youtube.com/watch?v=oRahosxToxA)