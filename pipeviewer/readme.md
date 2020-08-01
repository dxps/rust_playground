## pipeviewer

This project is based on [Hands-On Systems Programming with Rust](https://www.packtpub.com/programming/hands-on-systems-programming-with-rust-video) video course.

### Testing

A manual test:
```shell
$ cat yes.txt | cargo run -- > yes_out.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/pipeviewer`
[total bytes: 67993600]%
$ 
$ ls -l yes*.txt
-rw-r--r--  1 user  staff  67993600 Jul 31 16:17 yes.txt
-rw-r--r--  1 user  staff  67993600 Jul 31 16:28 yes_out.txt
$ 
```

Run the unit tests using `cargo test`.
As a sample, unit test is included in the `stats_logic.rs`.

### Documentation

Use `cargo doc --no-deps --open` to see the generated documentation.
