# Rust Hangman Game

## Overview

This is a simple implementation of the classic Hangman game in Rust. Players try to guess a hidden word by suggesting letters. The game provides feedback on correct and incorrect guesses, and players have a limited number of attempts to guess the entire word.

## Features

- Random word selection from a predefined list
- Interactive command-line interface
- Visual representation of the word progress
- Tracking of guessed letters and remaining attempts

## Prerequisites

To run this game, you need to have Rust installed on your system. If you haven't installed Rust yet, you can do so by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Setup

1. Clone this repository or download the source code.
2. Navigate to the project directory in your terminal.
3. Ensure you have the required dependencies by adding the following to your `Cargo.toml` file:

   ```toml
   [dependencies]
   rand = "0.8.5"
   ```

4. Build the project by running:

   ```
   cargo build
   ```

## How to Play

1. Start the game by running:

   ```
   cargo run
   ```

2. The game will select a random word and display underscores for each letter in the word.
3. Enter a letter as your guess when prompted.
4. If the letter is in the word, it will be revealed in its correct position(s).
5. If the letter is not in the word, you lose one attempt.
6. Keep guessing letters until you either:
   - Correctly guess the entire word (You win!)
   - Run out of attempts (Game over)
7. After the game ends, you can choose to play again or exit.

## Customization

You can easily customize the game by modifying the following in the source code:

- The list of words to choose from
- The number of allowed attempts
- The visual representation of the hangman (if you want to add ASCII art)

## Contributing

Contributions to improve the game are welcome! Here are some ideas for enhancements:

- Add difficulty levels (e.g., easy, medium, hard)
- Implement a scoring system
- Create a graphical user interface
- Add support for phrases or multiple words
- Introduce categories for words

Please feel free to submit pull requests or open issues for any bugs or feature requests.

## License

This project is open source and available under the [MIT License](LICENSE).
