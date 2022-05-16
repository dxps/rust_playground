# Getting Started

A template for kick starting a Cloudflare worker project using [`workers-rs`](https://github.com/cloudflare/workers-rs).

This template is designed for compiling Rust to WebAssembly and publishing the resulting worker to 
Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/).

## Usage 

It starts you off with a `src/lib.rs` file, acting as an entrypoint for requests hitting
your Worker. Feel free to add more code in this file, or create Rust modules anywhere else for this
project to use. 

With `wrangler`, you can build, test, and deploy your Worker with the following commands: 

```bash
# compiles your project to WebAssembly and will warn of any issues
wrangler build 

# run your Worker in an ideal development workflow (with a local server, file watcher & more)
wrangler dev

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
wrangler publish
```

### Interacting with it

Examples below are using the locally available worker (started using `wrangler dev`).<br/>
If you publish it, then you'd be able to access it through `cloudflare_workers_rustwasm_helloworld.{your-subdomain}.workers.dev`.

```shell
curl --location --request GET 'http://localhost:8787'
```

```shell
curl --location --request GET 'http://localhost:8787/worker-version'
```

```shell
curl --location --request POST 'http://localhost:8787/form/myField' \
--form 'myField="myValue"'
```


Read the latest `worker` crate documentation here: https://docs.rs/worker

<br/>

<hr/>

## WebAssembly

`workers-rs` (the Rust SDK for Cloudflare Workers used in this template) is meant to be executed as 
compiled WebAssembly, and as such so **must** all the code you write and depend upon. All crates and
modules used in Rust-based Workers projects have to compile to the `wasm32-unknown-unknown` triple. 

Read more about this on the [`workers-rs` project README](https://github.com/cloudflare/workers-rs).

## Issues

If you have any problems with the `worker` crate, please open an issue on the upstream project 
issue tracker on the [`workers-rs` repository](https://github.com/cloudflare/workers-rs).

