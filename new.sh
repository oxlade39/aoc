#!/bin/bash

# Get the year and day from the arguments
YEAR=$1
DAY=$2

# Determine the max year and day if arguments are not provided
if [ -z "$YEAR" ] || [ -z "$DAY" ]; then
    MAX_YEAR=$(find . -mindepth 1 -maxdepth 1 -type d -name "[0-9]*" | sort -nr | head -n 1 | sed 's|./||')
    if [ -z "$MAX_YEAR" ]; then
        echo "No existing year folders found. Please specify <year> and <day>."
        exit 1
    fi
    YEAR=${YEAR:-$MAX_YEAR}
    if [ -z "$DAY" ]; then
        MAX_DAY=$(find ./$YEAR -mindepth 1 -maxdepth 1 -type d -name "d[0-9]*" | sort -V | tail -n 1 | sed 's|.*/d||')
        DAY=$((MAX_DAY + 1))
    fi
fi

# Create the target directory
TARGET_DIR="./$YEAR/d$DAY"

# Ensure the directory exists
mkdir -p "$TARGET_DIR"

# Path to the new main.rs file
MAIN_RS_PATH="$TARGET_DIR/main.rs"

# Template for the main.rs content
read -r -d '' MAIN_RS_TEMPLATE << EOM
use core::str;
use std::{i64, str::FromStr, time::Instant, usize};

use aoclib::{input, timing};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    0
}

fn part2(txt: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {    
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(0, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(0, part1(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part2(test_input));
    }
}

EOM

# Write the template to the main.rs file
if echo "$MAIN_RS_TEMPLATE" > "$MAIN_RS_PATH"; then
    touch "$TARGET_DIR/input.txt"
    touch "$TARGET_DIR/input.test.txt"
    echo "main.rs created successfully at $MAIN_RS_PATH"
else
    echo "Failed to create main.rs at $MAIN_RS_PATH"
    exit 1
fi

# Path to the Cargo.toml file
CARGO_TOML_PATH="./$YEAR/Cargo.toml"

# Append a new bin section to Cargo.toml if it exists
if [ -f "$CARGO_TOML_PATH" ]; then
    BIN_NAME="d$DAY"
    BIN_SECTION="\n\n[[bin]]\nname = \"$BIN_NAME\"\npath = \"d$DAY/main.rs\""
    if echo -e "$BIN_SECTION" >> "$CARGO_TOML_PATH"; then
        echo "Updated Cargo.toml with a new bin section for $BIN_NAME"
    else
        echo "Failed to update Cargo.toml at $CARGO_TOML_PATH"
        exit 1
    fi
else
    echo "Cargo.toml not found at $CARGO_TOML_PATH"
fi
