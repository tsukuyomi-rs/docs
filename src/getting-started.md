# Getting Started

Tsukuyomi requires the latest version of Rust compiler.
The required Rust toolchain is 1.27 or higher.

```toml
[dependencies]
tsukuyomi = "0.2"
futures = "0.1.22"
```

```rust,no_run
extern crate tsukuyomi;
extern crate futures;

use tsukuyomi::{App, Input, Error, Handler};
use futures::Future;

// A *synchronous* handler function.
// It will return a `Responder` which immediately convert into an HTTP response,
// and does not need any asynchronous computation.
fn handler(_input: &mut Input) -> &'static str {
    "Hello, Tsukuyomi.\n"
}

// An *asynchronous* handler function.
// It will return a `Future` representing the remaining computation in the handler.
fn async_handler(input: &mut Input)
    -> impl Future<Item = String, Error = Error> + Send + 'static
{
    input.body_mut().read_all().convert_to::<String>()
        .and_then(|body| {
            Ok(format!("Received: {}", body))
        })
        .inspect(|_| {
            // You can access a mutable reference to `Input` stored in
            // the task-local storage by using Input::with_current():
            Input::with_current(|input| {
                println!("[debug] path = {}", input.uri().path());
            })
        })
}

fn main() -> tsukuyomi::AppResult<()> {
    let app = App::builder()
        .mount("/", |m| {
            m.get("/")
             .handle(Handler::new_ready(handler));

            m.post("/async")
             .handle(Handler::new_async(async_handler));
        })
        .finish()?;

    tsukuyomi::run(app)
}
```

## Using `futures-await` (requires Nightly compiler)

```toml
[dependencies]
tsukuyomi = "0.2"
futures-await = "0.1"
```

```rust,no_run
#![feature(proc_macro, proc_macro_non_items, generators)]

extern crate tsukuyomi;
extern crate futures_await as futures;

use futures::prelude::{async, await, Future};
use tsukuyomi::{App, Input, Handler};

#[async]
fn handler() -> tsukuyomi::Result<String> {
    let read_all = Input::with_current(|input| input.body_mut().read_all());
    let message: String = await!(read_all.convert_to())?;
    Ok(format!("Received: {}", message))
}

fn main() -> tsukuyomi::AppResult<()> {
    let app = App::builder()
        .mount("/", |m| {
            m.post("/")
             .handle(Handler::new_fully_async(handler));
        })
        .finish()?;

    tsukuyomi::run(app)
}
```

More examples are located at [`examples`](https://github.com/tsukuyomi-rs/examples).

