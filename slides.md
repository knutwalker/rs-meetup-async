# Async & Rust

2024-01-16, Leipzig Meetup

[https://gh.knutwalker.io/rs-meetup-async](https://gh.knutwalker.io/rs-meetup-async/blob/main/slides.md#async--rust)

---

Note: This is a very big topic, we can only talk about the tip of the iceberg. May this would even be better as an Q&A session 🤷

---

# Agenda, possibly

- What is async
- Why would I want to use async?
- How do I use it?
- (How does it work?)

---

# What is async?

- `async`/`await` syntax
- async vs sync programming
- parallel vs concurrent execution
- not unique to Rust:
    - C# 5.0, Python 3.5, ES2017, C++20, Swift 5.5, …

---

# Why would I want to use async?

- Highly concurrent, IO bound applications
- Because you have to
    - e.g. 3rd party crates with async only APIs
- Some concurrency primitives can be composed
- Some environments or applications are simpler

---

# Code examples

---

# How do I use it?

---

```rust
async fn some_fn() -> i32 {
    42
}
```

---

```rust
async fn caller() -> i32 {
    let answer = some_fn().await;
    println!("The answer is {answer}");
}
```

---

- `await` can only be used in `async` functions
- [Function coloring](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/)
- going from async to sync requires an executor/a runtime
    - e.g. `#[tokio::main]` or `futures::executor::block_on`

---

# RPIT

Return Position `impl Trait`

```rust
fn show1() -> impl Display {
    42
}

fn show2() -> impl Display {
    "42"
}
```

---

# RPIT

```rust
fn show1() -> i32 {
    42
}

fn show2() -> &'static str {
    "42"
}
```

---

# RPIT

```rust
fn no_show() -> impl Display {
    if rand::bool() {
        42
    } else {
        "42"
    }
}
```

---

# RPIT

- hide implementation
- communicate intent
- hide complex types
- return "Voldemort" types

---

# Voldemort types

- cannot be named
- e.g. closures

---

# Voldemort types

```rust
let f = |x| x + 42;
```

---

# Voldemort types

```rust
let f: ??? = |x| x + 42;
```

---

# Voldemort types

```rust
let f: fn(i32) -> i32 = |x| x + 42;
```

---

# Voldemort types

```rust
let f: impl Fn(i32) -> i32 = |x| x + 42;
```

---

# Voldemort types

```rust
let f: &dyn Fn(i32) -> i32 = &|x| x + 42;
```

---

# RPIT + Voldemort types

```rust
fn return_closure() -> impl Fn(i32) -> i32 {
    |x| x + 42
}
```

---

# RPIT + Voldemort types

```rust
fn return_closure() -> impl Fn(i32) -> i32 {
    if rand::bool() {
        |x| x + 42
    } else {
        |x| x + 42
    }
}
```

---

# RPIT + Voldemort types + async

Rust 1.36

```rust
trait Future {
    type Output;

    fn poll(&mut self) -> Option<Self::Output>;
}
```

---

# RPIT + Voldemort types + async

Rust 1.39

```rust
async fn some_fn() -> i32 {
    42
}

fn some_fn() -> impl Future<Output = i32> + Send + Sync + 'static {
    async { 42 }
}
```

---

# RPITIT

Return Position `impl Trait` In Trait

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
```

---

# RPITIT

Rust 1.75

```rust
trait IntoIterator {
    type Item;

    fn into_iter(self) -> impl Iterator<Item = Self::Item>;
}
```

---

# RPITIT + async

```rust
trait AsyncIterator {
    type Item;

    fn next(&mut self) -> impl Future<Output = Option<Self::Item>>;
}
```


---

# RPITIT + async = AFIT

Rust 1.75, Async Functions In Traits

```rust
trait AsyncIterator {
    type Item;

    async fn next(&mut self) -> Option<Self::Item>;
}
```

---

# Executor / Runtime

- `async`/`await` is just syntax sugar
- `Future`s are a lazily evaluated state machine
- Something has to drive the state machine
- Something has to manage all the state machines

---

# Executor / Runtime

- `tokio` the most popular one, general purpose, big ecosystem
- `async-std` general purpose, mirrors std
- `smol` general purpose, small & simple
- `embassy` for embedded systems
- `glommio` for high performance IO over `io_uring`
- `futures` for testing/demo

---

# Running async code

```rust
#[tokio::main]
async fn main() {
    println!("Hello, async world!");
}
```

---

# Running async code

```rust
fn main() {
    let future = async {
        println!("Hello, async world!");
    };
    tokio::runtime::Runtime::new().unwrap().block_on(future);
}
```

---

# Testing async code

```rust
#[tokio::test]
async fn test() {
    assert_eq!(42, async { 42 }.await);
}
```

---

# Testing async code

```rust
#[test]
fn test() {
    let future = async {
        assert_eq!(42, async { 42 }.await);
    };
    futures::executor::block_on(future);
}
```

---

# Code examples

---

# Spawning futures

```rust
async fn spawner() {
    tokio::spawn(async {
        println!("Hello, async world!");
    }).await
}
```

---

# Code examples

---

# Three levels of async

- For applications:
    - `async`/`await` syntax
    - `futures` combinators
    - rarely have to implement the `Future` manually

---

# Three levels of async

- For libraries:
    - `async`/`await` syntax
    - possibly have to implement `Future` manually
    - think about runtime compatibility

---

# Three levels of async

- For runtime implementors:
    - `Future` trait
    - Best for learning how async works under the hood

---

# Things I haven't talked about (yet)

- Manually implementing `Future`
    - `Pin` / `Unpin`
    - `Pin` projection
    - `Context` and `Waker`
    - `Send` and non-`Send` futures
    - `pin!` and `ready!` macros

---

# Things I haven't talked about (yet)

- Related concepts
    - Cancel safety
    - Generators
    - Coroutines
    - Async drop


---

# Things I haven't talked about (yet)

- Runtime specifics
    - Async IO/Networking APIs
    - Timer related APIs
    - Tracing and debugging

---

# Things I haven't talked about (yet)

- `futures` combinators
    - select, join, race
    - `FutureExt` and`TryFuture` traits
    - `Stream`, and `TryStream` traits
    - Existing combinators in `std`

---

# Code examples

---

# Resources

- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [async-std book](https://book.async.rs/)
- [Amos' blog](https://fasterthanli.me/tags/async)
- [Comprehensive guide on async](https://google.github.io/comprehensive-rust/async.html)
- [Logrocket](https://blog.logrocket.com/a-practical-guide-to-async-in-rust/)
- The, ahem, _future_
    - https://without.boats/blog/a-four-year-plan/
    - Boats, generally: https://without.boats/

