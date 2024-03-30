## A HackerNews clone using Dioxus 0.5, the Web template

This project uses:

-   Tailwind CSS
    -   Install [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm).
    -   Install the Tailwind CSS [CLI](https://tailwindcss.com/docs/installation).
    -   Run the following command in the root of the project to start the Tailwind CSS compiler:\
        `npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch` \
        **Note:** This needs to run while development, if you add specific Tailwind CSS classes to the components. Otherwise, the classes won't be available in the generated CSS (`tailwind.css`) file, and thus the addition won't be available in the UI.
-   Dioxus [CLI](https://dioxuslabs.com/learn/0.5/getting_started)
    -   Install it (to have `dx` command available) using `cargo install dioxus-cli@0.5.0`

<br/>

### Run

Run `dx serve --hot-reload` in the root of the project to start the Dioxus dev server. A `run.sh` script is included for this purpose.

Open the browser to http://localhost:8080
