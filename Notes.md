
Debugging, redirect the output of eprintln or dbg! to other terminal

```
cargo run --example demo 2> /dev/pts/1
```

where `/dev/pts/1` is the path of the terminal you are outputing into.
