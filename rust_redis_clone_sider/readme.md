## Sider - A Redis clone in Rust

This is one of the Redis clones that exists in the wild, as a learning experiment.

Of course, it doesn't implement the whole [RESP (Redis serialization protocol)](https://redis.io/docs/latest/develop/reference/protocol-spec/).

<br/>

### Usage

Use the standard `cargo run` to compile and start it.

Then connect to it using `redis-cli` and run commands (with capitals or small case).<br/>
The supported commands are:

-   `PING`
