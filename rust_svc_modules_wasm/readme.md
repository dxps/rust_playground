# A Rust (Axum-based) Service with a minimal (WIT Component Model based) Plugin System

This is a small Axum service that manages WebAssembly Component Model plugins at runtime.

The host can:

- register a plugin component from a `.wasm` path
- update a plugin's JSON configuration
- remove a plugin
- load/unload a plugin without restarting the service
- dispatch a request into a loaded plugin through a typed WIT ABI

## Plugin ABI

The contract lives in [wit/plugin.wit](wit/plugin.wit). A plugin exports `dxps:service-plugins/plugin-api`:

- `metadata() -> metadata`
- `configure(config-json: string) -> result<_, string>`
- `handle(req: request) -> result<response, string>`

## Run

Use the classic `cargo run` to start the service.

It listens on `127.0.0.1:3000`.

## HTTP API

```sh
curl http://127.0.0.1:3000/healthz
curl http://127.0.0.1:3000/plugins
```

Register a plugin:

```sh
curl -X POST http://127.0.0.1:3000/plugins \
  -H 'content-type: application/json' \
  -d '{
    "id": "echo",
    "wasm_path": "examples/echo-plugin/target/wasm32-wasip2/debug/echo_plugin.wasm",
    "config": {},
    "load": true
  }'
```

Load and unload:

```sh
curl -X POST http://127.0.0.1:3000/plugins/echo/load
curl -X POST http://127.0.0.1:3000/plugins/echo/unload
```

Update config:

```sh
curl -X PUT http://127.0.0.1:3000/plugins/echo \
  -H 'content-type: application/json' \
  -d '{"config":{"greeting":"hello"},"reload":false}'
```

Dispatch:

```sh
curl -X POST http://127.0.0.1:3000/plugins/echo/dispatch \
  -H 'content-type: application/json' \
  -d '{"method":"GET","path":"/demo","body":"hello"}'
```

Remove:

```sh
curl -X DELETE http://127.0.0.1:3000/plugins/echo
```

## Build the Example Plugin

The example plugin is intentionally outside the host package so the service can build even when you are not compiling guest components.

```sh
rustup target add wasm32-wasip2
cd examples/echo-plugin
cargo build --target wasm32-wasip2
```

The resulting component can be registered from `examples/echo-plugin/target/wasm32-wasip2/debug/echo_plugin.wasm`.

## Design Notes

Loaded plugins are kept in memory as Wasmtime component instances. Unloading drops the store and bindings, while keeping the registration and config. Updating config calls the plugin's `configure` function in place unless `reload` is set, in which case the component is reinstantiated and configured from scratch.
