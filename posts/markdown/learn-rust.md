---
title: learn rust
date: 11-23-2023
description: its hot new and so easy (i promise)
slug: rust
---

## rust is the new (not so) sexy language you have to learn!

if you are on programming youtube, you have seen nerds yelling at you to learn rust.
look im not a nerd so i'm not gonna try to convince you, buuuut if you wanna learn here's the basics as fast as possible.

## getting started

open up a terminal and run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
this will install the rust toolchain. this adds the compiler, `rustc`, and `cargo` which is the package manager (among other things).
now run `cargo new hello`. this will create a rust project with a hello world.

## woo that was so easy. this is gonna be easy.

`cd` into hello and open `src/main.rs` in a text editor. you will see:

```rust
fn main() { // main function
    println!("Hello, world!"); // print is a macro (not a function) in rust. more on that later
}
```

i added a couple comments. i'll do that to point out useful info.

now run `cargo run` in the terminal and it should print:

```
   Compiling hello v0.1.0 (/Users/matthewreed/projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.92s
     Running `target/debug/hello`
Hello, world!
```

cool it works. now let's do something

## let's build something trivial

under the main, add a new function:

```rust
fn add(x: i32, y: i32) -> i32 { // i32 is a 32 bit signed int. they are all named like that which is 🤌🏾
    return x + y;
}
```

this demonstrates how to make a new function, take in parameters and return a value.

now change main to: 

```rust
fn main() {
    let added = add(1, 2);
    println!("{}", added) // you can format a string in a print macro
}
```

this is how you call a function and print a value

run it and you should see: 

```
/Users/matthewreed/.cargo/bin/cargo run --color=always --package hello --bin hello
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello`
3
```