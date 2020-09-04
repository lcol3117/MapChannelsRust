# MapChannelsRust
A Rust implementation of the Map Channels concurrency model. Built on top of mpsc. 

Creating a `MapChannel`:

To make things more organized and reusable, `MapChannel` is a `trait`.

Using it is more verbose, but the benefits include:
- Reusability: You can reuse the map channels you create
- Safety: Type safety guarentees are built into Rust's trait system
- Extensibility: Rust has existential types, which means that any map channel can be represented as `impl MapChannel<InType,OutType>`.
- Awesome: Rust's traits are the coolest thing ever, and it's impossible not to love them. 

```rust
#[derive(Copy,Clone)]
struct MyMapChannel {}
impl MapChannel<InType,OutType> for MyMapChannel {
  fn pass(&self, input_name: InType) -> OutType {
    // Do stuff
  }
}
```

Your new `MapChannel` can now be used:

Create an instance:

```rust
MyMapChannel {}
```

Give it stuff, get back your `IdentityToken<O>`:

```rust
MyInstance.connect(stuff_to_process)
```

Get back transformed stuff:

```
MyInstance.find(my_identity_token)
```

FAQ:
- Why the heck does `IdentityToken<O>` exist? Why not just use a `mpsc::Receiver<O>`? Why does it need a UUID?
  - Debugging would by a nightmare without the UUID, all instances of the same map channel would look exactly the same!
