# First Welcome
this is a demo rust library published on crates.io

to use this library you have to add following line in dependency section of cargo.toml

`firstwelcome = "0.1.0"`

your cargo.toml file should look like this:
```
[package]
name = "most_used_functions"
version = "0.1.0"
authors = ["badar343 <66517064+badar343@users.noreply.github.com>"]
edition = "2018"

[dependencies]
most_used_functions = "0.1.0"
```

In `src/main.rs` you can use like this:

```
//for input
use most_used_functions;
use most_used_functions::get_input;
fn main() {
	let input = get_input::take_input();
}
```
following will also work:
```
//for calculatios. x , y should integer
use most_used_functions::calculate;
fn main() {
	calculate::add(x , y);        //for addition of two numbers
	calculate::subtract(x , y);   //for sutraction of two numbers
	calculate::divide(x , y);     //for division of two numbers
	calculate::multiply(x , y);   //for multiplication of two numbers
	calculate::modulus(x , y);    //for reminder 
    }
```

now `cargo run` for results
