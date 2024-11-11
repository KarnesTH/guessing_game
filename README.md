# Rust Guessing Game

Welcome to the Rust Guessing Game! This is a simple command-line game where the player tries to guess a randomly generated number between 1 and 100.

## Features

- Generates a random number between 1 and 100.
- Prompts the user to guess the number.
- Provides feedback if the guessed number is too low, too high, or correct.
- Uses colored output for better readability.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Make sure you have Rust installed on your system)

### Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/KarnesTH/guessing_game.git
    cd rust-guessing-game
    ```

2. Run the game:

    ```sh
    cargo run
    ```

## Usage

When you run the game, you will see the following prompt:

```
Welcome to Rust Guessing Game!
Please enter a number:
```

Enter your guess and press Enter. The game will provide feedback on whether your guess is too low, too high, or correct. The game continues until you guess the correct number.

## Example

```
Welcome to Rust Guessing Game!

Please enter a number:
50
Your number: 50
Your given number is too high!

Please enter a number:
25
Your number: 25
Your given number is too low!

Please enter a number:
37
Your number: 37
Yeah, you're right!
```

## Dependencies

This project uses the following Rust crates:

- [colored](https://crates.io/crates/colored): For colored terminal output.
- [rand](https://crates.io/crates/rand): For generating random numbers.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the Rust programming language documentation and tutorials.
