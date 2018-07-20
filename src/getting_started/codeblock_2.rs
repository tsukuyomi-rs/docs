# //! ```cargo
# //! [dependencies]
# //! tsukuyomi = "0.2"
# //! futures-await = "0.1"
# //! ```
#![feature(use_extern_macros)]
#![feature(proc_macro_non_items, generators)]

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
