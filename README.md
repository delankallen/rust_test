# Requests
## GET
    /api/todo
Get all todo items

## POST
    /api/newtodo
POST new todo item by Title

## DELETE
    /api/removetodo
Remove todo item with matching ID

    /api/clearall
Remove all todo items

## Make sure you have the proper tools installed
To run this example, you'll need to have git, rustc/cargo ([install here](https://rustup.rs/)), as well as `cargo-make` ([instructions here](https://github.com/sagiegurari/cargo-make)) installed on your machine. 

## Run the example
Open a terminal, navigate to where you cloned the project, go into the `/app` directory, and run `cargo make watch`. Wait for it to finish building, then open a new terminal window in the root directory of the project and run `cargo run`. Once it finishes building, you should be able to go to [localhost:8000](http://localhost:8000)
