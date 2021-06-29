# LiveView Technologies WebDev Test
Thank you for your interest in working with us in the LiveView Technologies Web Development team!

In this repo, you'll find the scaffolding for a very basic full-stack web app written in [Rust](https://www.rust-lang.org/) using the [Seed](https://seed-rs.org/) and [Actix-web](https://actix.rs) libraries.

Please follow these instructions to get started

## Make sure you have the proper tools installed
To complete this test, you'll need to have git, rustc/cargo ([install here](https://rustup.rs/)), as well as `cargo-make` ([instructions here](https://github.com/sagiegurari/cargo-make)) installed on your machine. 

## Clone this repository
Please <b>do not</b> fork this repo.

## Run the example
Open a terminal, navigate to where you cloned the project, go into the `/app` directory, and run `cargo make watch`. Wait for it to finish building, then open a new terminal window in the root directory of the project and run `cargo run`. Once it finishes building, you should be able to go to [localhost:8000](http://localhost:8000) and view a very simple app that looks like this:

![Rust test 1](https://lvt-dev.s3-us-west-2.amazonaws.com/rust-test/rust-test-3.png)

## Build the App!
To complete this test, your app needs to have a the following features:
* A sweet logo at the top (maybe something like [this](https://cameras.liveviewtech.com/img/LVLogo_small.png)?)
* An input to add new items to the list
* A save button that will make an HTTP request to add the value in the input to the list.
* A clear button that will make an HTTP request to remove all items in the list.
* A delete button next to each item that will make an HTTP request to remove it individually.
* Any other enhancements that you think would improve the user experience.

If you get stuck, be sure to check the Seed [tutorials](https://seed-rs.org/) and/or [API documentation](https://docs.rs/seed/0.7.0/seed/) for any Seed-related issues, the Actix [tutorials](https://actix.rs/docs/) and/or [API docs](https://docs.rs/actix-web/3.0.2/actix_web/) for server-side problems, or [The Rust Book](https://doc.rust-lang.org/book/) for general help with Rust.

The finished product should look something like this (but hopefully prettier):

![LVT](https://cameras.liveviewtech.com/img/LVLogo_small.png)

![Rust test 2](https://lvt-dev.s3-us-west-2.amazonaws.com/rust-test/rust-test-2.gif)

## Send us your code
Please create a zip file with all your code, email it to the Web Development Manager, and we'll take a look at your amazing finished product!
