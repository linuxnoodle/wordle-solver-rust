# wordle-solver-rust
An approach to wordle which is definitely not cheating, and now written in rust.
## Prerequisites
- cargo (rust manager)
## Building
```
git clone https://github.com/linuxnoodle/wordle-solver-rust
cd wordle-solver
cargo build --release
./target/release/wordle-solver-rust
```
## TODO
- [ ] Use huffman encoding (think that's the right term) to choose the best word out of a list
- [ ] Allow for entering in a board
- [X] Fix bug with (somewhat?) random segfaults (done by moving to rust)
- [ ] Implement CLI version of game
