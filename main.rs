#![forbid(unsafe_code)]
#![allow(non_snake_case)]

extern crate uuid;

use uuid::Uuid;

use std::sync::mpsc;
use std::thread;

trait MapChannel<A,R>
where
A: Send + Sync + 'static,
R: Send + Sync + 'static,
Self: Send + Sync + std::marker::Sized + Copy + 'static
{
  fn pass(&self, _: A) -> R;
  fn connect(&self, input: A) -> IdentityToken<R> {
    let (tx, rx) = mpsc::channel();
    let thread_self = self.clone();
    thread::spawn(move || {
      tx.send(thread_self.pass(input)).unwrap();
    });
    IdentityToken::<R>::new(rx)
  }
  fn find(&self, identity_token: IdentityToken<R>) -> R {
    identity_token
      .channel_rx
      .recv()
      .unwrap()
  }
}

#[derive(Debug)]
struct IdentityToken<O> {
  channel_rx: mpsc::Receiver<O>,
  identity: Uuid
}

impl<O> IdentityToken<O> {
  fn new(new_channel_rx: mpsc::Receiver<O>) -> Self {
    IdentityToken::<O> {
      channel_rx: new_channel_rx,
      identity: Uuid::new_v4()
    }
  }
}

macro_rules! mc {
  {$n:ident($ni:ident : $ti:ty) -> $to:ty {$b:expr}} => {
    #[derive(Copy,Clone)]
    struct $n {}
    impl MapChannel<$ti,$to> for $n {
      fn pass(&self, $ni : $ti) -> $to {
        $b
      }
    }
  }
}

// DEMO EXAMPLE TEST THINGY

mc! {
  Greet(name: String) -> String {
    "Hello "
      .to_owned()
      + &name
  }
}

fn main() {
  let greeter = Greet {};
  let landon_id = greeter.connect("landon".to_owned());
  println!("{}", greeter.find(landon_id));
}
