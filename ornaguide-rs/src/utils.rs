use futures::Future;

pub(crate) mod html;

/// Build a tokio runtime for the current thread and await the future on it.
pub fn block_on_this_thread<F: Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future)
}
