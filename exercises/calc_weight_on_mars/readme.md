# Weight on Mars Calculator

A simple calculator app that showcases:
- How to get the user input
  - An external crate name `dialoguer` is used, since `print!` macro does not flush the text and currently flushing to stdout turns out to be not that simple.
- Functions
- `String` manipulation
  - A trimming and conversion to float are being done.
- `Result` (error) handling using `.unwrap()`

## Usage

A usage example:

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/calc_weight_on_mars`
Enter your weight (kg): 85
Your weight on Mars is 32.154434 kg.
$
```
