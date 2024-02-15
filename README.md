# snake-pipe-rust

[![crates.io](https://img.shields.io/crates/v/snakepipe.svg)](https://crates.io/crates/snakepipe) [![Docs](https://docs.rs/snakepipe/badge.svg)](https://docs.rs/snakepipe/latest/snakepipe/) [![Build](https://github.com/topheman/snake-pipe-rust/actions/workflows/rust.yml/badge.svg?label=build)](https://github.com/topheman/snake-pipe-rust/actions/workflows/rust.yml)

Not just yet another snake game in the terminal 😉.

https://github.com/topheman/snake-pipe-rust/assets/985982/76161595-1c3a-4252-9cbd-25e144bf185c

This one follows the [unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy) as:

- `snakepipe gamestate` accepts user inputs, calculates the state of the game and writes it to `stdout`
- `snakepipe render` reads the state of the game from `stdin` and renders it on the terminal
- `snakepipe throttle` reads a pre-recorded game from `stdin` and writes to `stdout` each tick so that `snakepipe render` can pick it up

That way:

- you could write your own version of the `gamestate` or `render` command in any programming language and make it work with mine
- it's a great exercise to handle stream serialization/deserialization in rust

## Prerequisites

- Rust 1.75.0

## Install

```sh
cargo install snakepipe
```

## Usage

🎮 Play in terminal

```sh
# basic usage
snakepipe gamestate|snakepipe render

# change the defaults
snakepipe gamestate --frame-duration 80 --width 70 --height 20 --snakepipe-length 15|snakepipe render

# call help on any of the commands
snakepipe --help
```

📼 You can even record and replay using basic piping

```sh
# record a game into a file using the `tee` command utility
snakepipe gamestate|tee /tmp/snakepipe-output|snakepipe render

# replay the game you recorded
cat /tmp/snakepipe-output|snakepipe throttle|snakepipe render
```

## Manual of commands

<details>
  <summary><code>snakepipe --help</code></summary>
  <pre>
Usage: snakepipe <CMD_>

Commands:
  gamestate
  render
  throttle
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
  </pre>
</details>

<details>
  <summary><code>snakepipe gamestate --help</code></summary>
  <pre>
Usage: snakepipe gamestate [OPTIONS]

Options:
      --frame-duration <FRAME_DURATION>  in ms [default: 120]
      --width <WIDTH_>                    default 25
      --height <HEIGHT_>                  default 25
      --snakepipe-length <snakepipe_LENGTH>      [default: 2]
      --fit-terminal
  </pre>
</details>

<details>
  <summary><code>snakepipe render --help</code></summary>
  <pre>
Usage: snakepipe render
  </pre>
</details>

<details>
  <summary><code>snakepipe throttle --help</code></summary>
  <pre>
Usage: snakepipe throttle [OPTIONS]

Options:
      --frame-duration <FRAME_DURATION>  in ms [default: 120]
  </pre>
</details>

## Using as a library

This crate is a cli, but it also exports a lib from where you can import a few utilities, such as `snakepipe::stream::parse_gamestate` - [direct link to docs.rs](https://docs.rs/snakepipe/latest/snakepipe/stream/fn.parse_gamestate.html):

```rust
use snakepipe::stream::{parse_gamestate, Game};

fn main() -> () {
    match parse_gamestate() {
        Ok(stream) => {
            println!(
                "Frame duration {}, Snake length {}, Level {}x{}",
                stream.options.frame_duration,
                stream.options.snake_length,
                stream.options.size.width,
                stream.options.size.height
            );
            for parsed_line in stream.lines {
                do_something(parsed_line);
            }
        }
        Err(e) => {
            println!("Error occurred while parsing stdin: \"{}\"", e);
        }
    }
}

fn do_something(parsed_line: Game) {
    println!("Snake head position {:?}", parsed_line.snake.head)
}
```

## Next

- [ ] Make an implementation of the `render` command that starts a server so that the render happens in a browser
- [ ] Make an implementation of the actual `render` for the terminal in an other language than rust
