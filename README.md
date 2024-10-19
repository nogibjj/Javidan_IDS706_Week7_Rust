[![CI/CD](https://github.com/nogibjj/Javidan_IDS706_Week7_Rust/actions/workflows/main.yaml/badge.svg)](https://github.com/nogibjj/Javidan_IDS706_Week7_Rust/actions/workflows/main.yaml)


# Javidan_IDS706_Week7_Rust

This repository contains Rust implementations related to the **Levenshtein Distance Algorithm**, broken down into manageable, modular functions. It reflects the basic structure including CI/CD, Testing, Makefile and reproduceble environments. 

## Project Structure

- **src/**
  - **lib.rs**: Contains the core functions to compute Levenshtein Distance.
  - **main.rs**: The entry point to the program, calls functions from `lib.rs`.

- **data/**: Non-script object to use in project and readme

## Functions

### 1. `initialize_distances(token1_len, token2_len)`
   - Initializes a 2D vector to store distances between tokens. 
   - The first row and first column are filled with incremental values representing the distance at each stage.

### 2. `calculate_minimum(a, b, c)`
   - Calculates the minimum of three values, used to compute the Levenshtein distance in `fill_distances`.

### 3. `fill_distances(token1, token2, distances)`
   - Fills the distance matrix with calculated distances based on character comparisons between two tokens.

### 4. `levenshtein_distance_dp(token1, token2)`
   - Main function that calculates and returns the Levenshtein distance between two strings using dynamic programming.

## Running the Code

### Prerequisites

Ensure that you have **Rust** installed. If not, you can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Build and Run

To build the project, navigate to the project root and run:

```bash
cargo build
```

To execute the main program:

```bash
cargo run
```

### Testing

The project includes unit tests for the main functions. To run the tests:

```bash
make test
```

## Linting and Formatting

To check the code for linting issues using **Clippy**, and to format the code using **rustfmt**, run the following commands:

```bash
make lint
```

To auto-format the code:

```bash
make format
```

## Related Images
![Run Example Console Output](https://github.com/nogibjj/Javidan_IDS706_Week7_Rust/blob/ed1b2464843227c18ad819eaa6cd887f8917ae37/data/run.png)

![Test Example Console Output](https://github.com/nogibjj/Javidan_IDS706_Week7_Rust/blob/ed1b2464843227c18ad819eaa6cd887f8917ae37/data/testing.png)

## License

This project is licensed under the [MIT License](https://github.com/nogibjj/Javidan_IDS706_Week7_Rust/blob/232d3bb984b144f90a2425f53b2d900379e67cf5/LICENSE.txt).