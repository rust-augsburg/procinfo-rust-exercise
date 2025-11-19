# Mini Top in Rust

This project implements a minimal "top-like" system monitor using the
Linux `/proc` filesystem. It is designed for learning Rust, practicing
structured parsing, and exploring Linux internals.

## Features

-   Reads memory statistics from `/proc/meminfo`
-   Lists running processes by scanning `/proc/<pid>`
-   Extracts process name and RSS memory usage from:
    -   `/proc/<pid>/comm`
    -   `/proc/<pid>/status`
-   Displays the top N memory-consuming processes
-   Updates every second

## Structure

The code is split into clear, testable units:

-   `parse_meminfo` parses the content of `/proc/meminfo`
-   `parse_process_status` extracts `VmRSS` from `/proc/<pid>/status`
-   `read_process_comm` loads process names
-   `read_process_status` loads process memory usage
-   `read_process` combines both fields
-   `list_processes_from` scans `/proc` and collects process information
-   `print_top_processes` handles output formatting

All pure parsing functions have unit tests.

## Requirements

-   Linux operating system
-   `/proc` filesystem mounted
-   Rust toolchain (stable recommended)

## Running

``` bash
cargo run
```

The program updates output once per second.

## Tests

``` bash
cargo test
```

## Goals

This project is designed for teaching how to:

-   Read from the Linux procfs
-   Structure code for testability
-   Parse structured but untyped text files
-   Separate logic, I/O, and presentation

## Extensions

Possible improvements include:

-   CPU usage from `/proc/stat`
-   Sorting by CPU or memory
-   Terminal UI using Crossterm or Ratatui
-   Process filtering
-   Async refresh using Tokio



## Exercise Instructions

This repository includes a complete coding exercise.  
You can find the full step-by-step guide here:

[exercise.md](./exercise.md)
