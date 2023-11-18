## Gracefully Shutting Down using Tokio

This is a minimal implementation - using [Tokio](https://tokio.rs) - of a graceful (or at least controlled) shutdown of an app in case CTRL+C signal is received.

<br/>

### On Windows

Using Command Prompt, start the app using `cargo run` and then type Ctrl+C and the application output confirms that the signal is received and the shutdown is initiated by the implementation.

```shell
> cargo run
   Compiling rust_tokio_graceful_shutdown v0.1.0 (C:\Users\...\rust_tokio_graceful_shutdown)
    Finished dev [unoptimized + debuginfo] target(s) in 0.97s
     Running `target\debug\rust_tokio_graceful_shutdown.exe`
Server started. The process id is 22112.
Waiting for the shutdown signal ...
Shutdown signal received.
Exit now.

> 
```

The alternative to typing Ctrl+c is to run a script that would send this signal to the process (as we're used to do using `kill` on *nix systems). In this case, you can use the included `ctrl_c.py` script and run it using `python tools\ctrl_c.py 22112`.
