# uh
The uh library provides functions and mathematical input operations

to use this library you have to add following line in dependency section of cargo.toml

`uh = "0.1.0"`

your cargo.toml file should look like this:
```
[package]
name = "uh"
version = "0.1.0"
authors = ["badar343 <66517064+badar343@users.noreply.github.com>"]
edition = "2018"

[dependencies]
uh = "0.1.0"
```

In `src/main.rs` you can use like this:

```
//for input
use uh;
use uh::get_input;
fn main() {
	let input = get_input::take_input();
}
```
following will also work:
```
//for calculatios. x , y should integer
use uh::calculate;
fn main() {
	calculate::add(x , y);        //for addition of two numbers
	calculate::subtract(x , y);   //for sutraction of two numbers
	calculate::divide(x , y);     //for division of two numbers
	calculate::multiply(x , y);   //for multiplication of two numbers
	calculate::modulus(x , y);    //for reminder 
    }
```

now `cargo run` for results
