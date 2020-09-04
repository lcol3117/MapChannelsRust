# MapChannelsRust
A Rust implementation of the Map Channels concurrency model. Built on top of mpsc. 

Creating a `MapChannel`:

To make things more organized and reusable, `MapChannel` is a `trait`.

Using it is more verbose, but the benefits include:
- Reusability: Re

```
#[derive(Copy,Clone)]
struct MyMapChannel {}
impl MapChannel<InType,OutType> for MyMapChannel {
  fn pass(&self, input_name: InType) -> OutType {
    // Do stuff
  }
}
```

Your new `MapChannel`
