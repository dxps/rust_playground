## Rocket Hello sample

A sample on using Rocket 0.5 release with Rust stable toolchain.

<br/>

### Run

The classic `cargo run` starts the app in debug mode, logging enough details on startup, and request and reply summary.

If you're on a *nix system, you can use the provided `run...` scripts.

<br/>

### Usage

`curl -v localhost:8000` to access the `/` path.

For a small stress test, you can use `stress.sh` script. As an example, when running the build done in release mode (see `run-release.sh`), here are some figures (on a Dell Latitude laptop, having 8 cores of Intel Core i7-10610U):
```shell
$ ./stress.sh
18:05:07.224 [INFO] benchrs:0.1.8
18:05:07.224 [INFO] Spawning 8 threads
18:05:07.365 [INFO] Ran in 0.11211325s 50 connections, 10000 requests with avg request time: 0.0128ms, median: 0ms, 95th percentile: 0ms and 99th percentile: 1ms
$ 
```
<br/>
