# Number Guessing Game

This is a simple number guessing game written in Rust. The game generates a random number between 1 and 100 (inclusive) and the player has to guess this number.

## How to Run

You need to have Rust installed on your machine. If you don't have it installed, you can download it from [here](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can run the game by using the following command in the terminal:

```bash
cargo run
```

## How to Play

When the game starts, it will display a message "Number Guessing Game!!" and generate a secret number between 1 and 100.
The game will then prompt you to enter a number.
After you enter a number, the game will compare your guess with the secret number.
If your guess is too high, it will display a message "Too big!" in red.
If your guess is too low, it will display a message "Too less!" in blue.
If your guess is correct, it will display a message "Correct Guess!!" in green and the game will end.
If your guess is incorrect, the game will prompt you to enter a new number and the process repeats until you guess the correct number.

## Dependencies

This game uses the following Rust crates:

std::io for standard input/output.
rand::Rng for generating random numbers.
std::cmp::Ordering for comparing numbers.
colored::* for colored output.
