## pipeviewer

### Testing

A manual test:
```shell
$ cat yes.txt | cargo run -- > yes_out.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/pipeviewer`
[total bytes: 67993600]%
$ 
$ ls -l yes*.txt
-rw-r--r--  1 mariusi  staff  67993600 Jul 31 16:17 yes.txt
-rw-r--r--  1 mariusi  staff  67993600 Jul 31 16:28 yes_out.txt
$ 
```
