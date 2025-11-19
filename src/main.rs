//! A minimal, educational "top-like" system monitor implemented in Rust.
//!
//! # Overview
//!
//! This program demonstrates how to read basic system information from the
//! Linux `/proc` filesystem and present it in a simple, continuously updating
//! terminal view.
//!
//! It performs three main tasks:
//!
//! 1. Reads system memory usage from `/proc/meminfo`
//! 2. Reads running processes from numeric directories in `/proc/<pid>/`
//! 3. Extracts process name and resident memory size (RSS) from:
//!    - `/proc/<pid>/comm`
//!    - `/proc/<pid>/status`
//!
//! The printing logic displays the top N processes by memory usage.
//!
//! This file contains only TODO markers that correspond to the teaching
//! worksheet Parts 1–8 (e.g., TODO-Part3).

use std::{fs, io, thread, time::Duration};

// ============================================================================
// Part 2 – Memory parsing
// ============================================================================

/// Parses the content of `/proc/meminfo`.
///
/// Expected output: `(total_kb, available_kb)`.
///
/// TODO-Part2:
/// - Parse the MemTotal line
/// - Parse the MemAvailable line
/// - Use iterators and split_whitespace
pub fn parse_meminfo(_content: &str) -> io::Result<(u64, u64)> {
    // TODO-Part2
    unimplemented!()
}

/// Reads `/proc/meminfo` from a path and delegates to parse_meminfo.
///
/// TODO-Part2:
/// - Read file with fs::read_to_string
/// - Call parse_meminfo
pub fn read_meminfo_from(_path: &str) -> io::Result<(u64, u64)> {
    // TODO-Part2
    unimplemented!()
}

// ============================================================================
// Part 3 + Part 4 + Part 5 – Process parsing
// ============================================================================

/// Parses `VmRSS` from `/proc/<pid>/status`.
///
/// Example line: `VmRSS:   1234 kB`
///
/// TODO-Part3:
/// - Find the line beginning with "VmRSS:". Use .lines().find(...)
/// - Extract the number from the split fields
/// - Convert to u64
pub fn parse_process_status(_status: &str) -> Option<u64> {
    // TODO-Part3
    unimplemented!()
}

/// Reads the `/proc/<pid>/status` file.
///
/// TODO-Part3:
/// - Read file
/// - Call parse_process_status
pub fn read_process_status(_base: &str, _pid: &str) -> Option<u64> {
    // TODO-Part3
    unimplemented!()
}

/// Reads the `/proc/<pid>/comm` file to get the process name.
///
/// TODO-Part4:
/// - Read file using fs::read_to_string
/// - Trim newline
pub fn read_process_comm(_base: &str, _pid: &str) -> String {
    // TODO-Part4
    unimplemented!()
}

/// Combines the process name and memory usage.
///
/// TODO-Part5:
/// - Call read_process_comm
/// - Call read_process_status
/// - Return Some((name, rss_kb)) or None
pub fn read_process(_base: &str, _pid: &str) -> Option<(String, u64)> {
    // TODO-Part5
    unimplemented!()
}

// ============================================================================
// Part 6 – Process listing
// ============================================================================

/// Scans `/proc`, filters numeric directories, and collects process info.
///
/// TODO-Part6:
/// - Iterate through fs::read_dir
/// - Filter entries where the directory name is numeric
/// - Call read_process for each PID
/// - Collect into a Vec<(pid, name, rss_kb)>
pub fn list_processes_from(_base: &str) -> io::Result<Vec<(String, String, u64)>> {
    // TODO-Part6
    unimplemented!()
}

// ============================================================================
// Part 7 – Printing
// ============================================================================

/// Prints the top N processes using iter().take(n).
///
/// TODO-Part7:
/// - Use procs.iter().take(n)
/// - Print formatted rows
pub fn print_top_processes(_procs: &[(String, String, u64)], _n: usize) {
    // TODO-Part7
    unimplemented!()
}

// ============================================================================
// Part 8 – Main loop
// ============================================================================

fn main() -> io::Result<()> {
    loop {
        // TODO-Part8: Call read_meminfo_from("/proc/meminfo")
        // let (total, available) = read_meminfo_from("/proc/meminfo")?;

        // TODO-Part8: Print memory summary

        // TODO-Part8: Call list_processes_from("/proc")
        // let mut procs = list_processes_from("/proc")?;

        // TODO-Part8: Sort descending by memory usage

        // TODO-Part8: print_top_processes(&procs, 5);

        thread::sleep(Duration::from_secs(1));
    }
}

// ============================================================================
// Tests – Part 2 and Part 3 starting point
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_meminfo() {
        let input = "\
MemTotal:       16384256 kB
MemAvailable:    2345678 kB";

        let (total, free) = parse_meminfo(input).unwrap();
        assert_eq!(total, 16384256);
        assert_eq!(free, 2345678);
    }

    #[test]
    fn test_parse_process_status() {
        let input = "Name: myproc\nVmRSS:   1234 kB\n";
        let mem = parse_process_status(input);
        assert_eq!(mem, Some(1234));
    }
}
