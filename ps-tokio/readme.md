## ps-tokio

A minimal chat server implementation, done with Tokio library.<br/>
It is based on a PluralSight course, but improved a bit the graceful shutdown.

<br/>

### Usage

1. Run the server using `cargo run`.
2. Open two clients using `telnet localhost 8080`.
3. Type any message on any of the clients and you should see the message on the other chat terminal.
4. Stop the server using `Ctrl +C` to have them all gracefully shutted down.5.
