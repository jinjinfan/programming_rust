use std::sync::{Arc, Mutex};
use std::task::{Waker, Wake};

struct Shared<T> {
    value : Option<T>,
    waker : Option<Waker>,
}
pub struct SpawnBlocking<T>(Arc<Mutex<Shared<T>>>);

pub fn spawn_blocking<T,F>(closure : F) -> SpawnBlocking<T> 
    where F : FnOnce() -> T,
          F : Send + 'static,
          T : Send + 'static
{
    let inner = Arc::new(Mutex::new(Shared {
        value : None,
        waker : None,
    }));

    std::thread::spawn({
        let inner = inner.clone();
        move || {
            let value = closure();
            let maybe_waker = {
                let mut guard = inner.lock().unwrap();
                guard.value = Some(value);
                guard.waker.take()
            };
            if let Some(waker) = maybe_waker {
                waker.wake();
            }
        }
    });
    SpawnBlocking(inner)
}

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

impl<T:Send> Future for SpawnBlocking<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.0.lock().unwrap();
        if let Some(value) = guard.value.take() {
            return  Poll::Ready(value);
        }
        guard.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

use async_std::pin;
use waker_fn::waker_fn;
use futures_lite::pin;
use crossbeam::sync::Parker;

fn block_on<F:Future>(future : F) -> F::Output {
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = waker_fn(move || unparker.unpark());
    let mut context = Context::from_waker(&waker);
    pin!(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => parker.park(),
        }
    }
}

fn main() {
    let mut string = "Pinned?".to_string();
    let mut pinned : Pin<&mut String> = Pin::new(&mut string);

    pinned.push_str(" Not");
    Pin::into_inner(pinned).push_str(" so much.");

    let new_home = string;
    println!("{}", new_home);
}