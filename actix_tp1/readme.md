## Actix - Touch point 1

Playing with Actix Web and evaluating the framework.

### Run

Use the standard `cargo run` to start it.

### Usage

Currently, it implements two "service" endpoints.<br/>
Accessing them using cURL is as simply as this:
- `curl localhost:8080` that returns the text(word) "Index"
- `curl localhost:8080/index.html` that returns the text "Index page"
