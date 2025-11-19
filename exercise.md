# Proc Info in Rust -- Training Exercise

## Exercise Description

In this exercise, you will implement a small, simplified version of the
Linux `top` tool using Rust.\
The goal is to gain experience with:

-   Reading system information from the `/proc` filesystem
-   Parsing unstructured text into typed Rust data
-   Building small, testable components
-   Separating I/O from logic
-   Printing structured terminal output

The completed program will display: - Total and available memory - The
top five processes by memory usage - A refresh every second

The project is intentionally small to focus on Rust fundamentals.

## Part 1: Understand `/proc`

Explore these files manually: - `/proc/meminfo` - `/proc/<pid>/comm` -
`/proc/<pid>/status`

Resources: -
https://www.kernel.org/doc/Documentation/filesystems/proc.txt -
`man proc`

## Part 2: Parsing Memory Information

In this step, you will parse the two key fields from `/proc/meminfo`:

- `MemTotal`
- `MemAvailable`

Before writing any code, start by running the unit tests to see the expected behavior:

```sh
cargo test
```

You will see the test for `parse_meminfo` failing.
Your goal is to implement the function so that the test passes.


Rust references: -
https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_whitespace -
https://doc.rust-lang.org/stable/std/primitive.str.html#method.strip_prefix -
https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

## Part 3: Parsing Process Memory

Extract `VmRSS` from `/proc/<pid>/status`.

References: -
https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find -
https://doc.rust-lang.org/std/primitive.str.html#method.parse

## Part 4: Process Names

Read and trim the process name from `/proc/<pid>/comm`.

Reference: - https://doc.rust-lang.org/std/fs/fn.read_to_string.html -
https://doc.rust-lang.org/std/primitive.str.html#method.trim

## Part 5: Combining Information

Return `Some((name, rss_kb))` or `None`.

Reference: - https://doc.rust-lang.org/std/option/enum.Option.html

## Part 6: Listing Processes

Scan `/proc`, filter numeric PIDs, gather process data.

References: - https://doc.rust-lang.org/std/fs/struct.ReadDir.html -
https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by

## Part 7: Sorting and Display

Sort descending by memory usage and print the top 5.

Reference: -
https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take

## Part 8: Continuous Operation

Loop with a 1-second refresh.

References: - https://doc.rust-lang.org/std/thread/fn.sleep.html -
https://doc.rust-lang.org/std/time/struct.Duration.html

## Additional Parts

Solve the exercise using the `procinfo` crate. This solutions also works on Windows and macOs.
You have to adjust your tests.

References: https://docs.rs/crate/procinfo/latest
