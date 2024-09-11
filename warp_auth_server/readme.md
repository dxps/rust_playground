## Authentication Server using WARP

### Run

You can use the standard `cargo run`.

### Usage

The following operations (use cases) are implemented:

-   User Registration:
    ```shell script
    curl -v -X POST localhost:3000/register          \
           -H "content-type: application/json"       \
           -d '{ "username":"me", "password":"so" }'
    ```
-   User Login
    ```shell script
    curl -v -X POST localhost:3000/login               \
             -H "content-type: application/json"       \
             -d '{ "username":"me", "password":"so" }'
    ```
