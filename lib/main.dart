import 'dart:ffi';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_rust_example/bridge_generated_api.dart';
import 'package:flutter_rust_example/bridge_generated_methods.dart';

const base = "rust";
final path = Platform.isWindows ? "$base.dll" : "lib$base.so";
late final dylib = Platform.isIOS
    ? DynamicLibrary.process()
    : Platform.isMacOS
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path);

late final api = CoreApiImpl(dylib);
late final methodsApi = MethodsApiImpl(dylib);

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);
  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  late Future<int> counter;
  late Stream<int> ticks;
  late Stream<int> controlledTicks = Stream.empty();

  @override
  void initState() {
    super.initState();
    counter = methodsApi.getCounter();
    ticks = api.tick();
    api
        .call(request: ActionRequest(action: "echo", payload: "hello there"))
        .then((r) => debugPrint("ActionResponse: ${r.success} ${r.response}"))
        .catchError((error) => debugPrint("ActionResponse.error: $error"));
    methodsApi
        .getCounter()
        .then((r) => debugPrint("GetCounter: $r"))
        .catchError((error) => debugPrint("GetCounter.error $error"));
  }

  void _incrementCounter() {
    setState(() {
      counter = methodsApi.increment();
    });
  }

  void _startStream() {
    setState(() {
      controlledTicks = api.startStream();
    });
  }

  void _stopStream() {
    setState(() {
      api.stopStream();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text(
              'You have pushed the button this many times:',
            ),
            FutureBuilder<List<dynamic>>(
                future: Future.wait([counter]),
                builder: (context, snap) {
                  final data = snap.data;
                  if (data == null) {
                    return const Text("Loading");
                  }
                  return Text(
                    '${data[0]}',
                    style: Theme.of(context).textTheme.headline4,
                  );
                }),
            StreamBuilder<int>(
                stream: ticks,
                builder: (context, snap) {
                  final data = snap.data;
                  if (data == null) {
                    return const Text("Loading");
                  }
                  return Text(
                    'tick: $data',
                    style: Theme.of(context).textTheme.headline4,
                  );
                }),
            TextButton(onPressed: _startStream, child: Text("Start Stream")),
            TextButton(onPressed: _stopStream, child: Text("Stop Stream")),
            StreamBuilder<int>(
                stream: controlledTicks,
                builder: (context, snap) {
                  final data = snap.data;
                  if (data == null) {
                    return const Text("Loading");
                  }
                  return Text(
                    'tick: $data',
                    style: Theme.of(context).textTheme.headline4,
                  );
                }),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
