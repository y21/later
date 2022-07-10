# `later`
Defer code to be executed at the end of the scope, much like `defer` in Go.

```rs
fn main() {
    later::defer! {
        println!("World!");
    }
    println!("Hello");
}
```

## Features
- Access to local variables (see below)
- Drop guards are unnameable and cannot be passed to `mem::forget`
- Async support
- `#![no_std]` compatible

## Limitations
- Only one defer per scope

## Accessing local variables
The code snippet above shows how the defer macro can be used for running "freestanding" code (no access to locals), however sometimes access to local variables is needed.

In that case you can pass a closure to the macro. Note that the parameter names must match the variable names you would like to capture from the surrounding environment. The closure body can also be an async block, in which case it will be awaited.
```rs
fn main() {
    let fut = std::future::ready(123);

    // This runs when main exits
    later::defer!(|fut: std::future::Ready<i32>| async {
        println!("World = {}", fut.await);
    });

    println!("Hello");
}
```
> Note: Async blocks requires the `async` feature. Enabling it in turn disables `no_std` support.

## Capture semantics
When capturing variables from the enclosing environment, they are **cloned** by default.
You can specify the `move` keyword in front of the closure to move them rather than cloning, like so:
```rs
fn main() {
    let foo = String::from("bar");

    //            vvvv
    later::defer!(move |foo: String| {
        println!("{foo}");
    });

    print!("-> ");
}
```
If you now try to use `foo` after it's been moved into the `defer`, compilation will fail.
