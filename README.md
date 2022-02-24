# titik

[![Build Status](https://api.travis-ci.com/ivanceras/titik.svg?branch=master)](https://travis-ci.com/github/ivanceras/titik)
[![Latest Version](https://img.shields.io/crates/v/titik.svg)](https://crates.io/crates/titik)

Titik is a crossplatform TUI widget library with the goal of being able to interact
intuitively on these widgets.

![Screenshot](https://ivanceras.github.io/screenshots/titik-record2.gif)

It uses [`crossterm`](https://crates.io/crates/crossterm) as the underlying backend.

To run the demo use the following command:
```sh
cargo run --example demo 2>/dev/null
```
Note: `2>/dev/null` is sending the debugging log from `eprintln` into the `/dev/null` device

Without doing so, will result a flicker in your screen caused by debugging info and tui mixed
in one terminal output.

Alternatively, you can pipe the debugging log from `eprintln` into a file say `/tmp/debug.log`
by doing so:
```sh
cargo run --example demo 2>/tmp/debug.log
```

You can then open a new terminal and tail the `/tmp/debug.log` file
```sh
tail -f /tmp/debug.log
```

License: MIT
