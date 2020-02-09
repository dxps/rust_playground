## minigrep

This is the CLI example from "The Book" (TRPL), chapter 12 "An I/O Project: Building a Command Line Program".
<br/>

### Usage

Arguments: <query> <path-to-file>

For example, run a regular (case sensitive) search using:
```bash
$ cargo run to test/poem.txt
```

Run a case insesitive search using:
```bash
$ CASE_INSENSITIVE=1 cargo run to test/poem.
```
<br/>

### Tests

Two unit tests are included in `lib.rs`. <br/>
`cargo test` result is green.
