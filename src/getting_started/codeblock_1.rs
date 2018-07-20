# //! ```cargo
# //! [dependencies]
# //! tsukuyomi = "0.2"
# //! futures = "0.1.22"
# //! ```
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
