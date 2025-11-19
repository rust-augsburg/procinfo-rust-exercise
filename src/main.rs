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
//! 1. **Reads system memory usage** from `/proc/meminfo`  
//!    (`MemTotal` and `MemAvailable` fields).
//! 2. **Reads running processes** by scanning the numeric directories inside
//!    `/proc/<pid>/`.
//! 3. **Extracts process name and resident memory size (RSS)** from:
//!    - `/proc/<pid>/comm` — process name  
//!    - `/proc/<pid>/status` — `VmRSS` in kB  
//!
//! The program then prints the **top N processes by memory usage** to the
//! terminal every second.
//!
//! # Structure
//!
//! The code is split into small, test-friendly units:
//!
//! - `parse_meminfo` – pure function for parsing `/proc/meminfo` content  
//! - `parse_process_status` – pure parser for `VmRSS` extraction  
//! - `read_process_status` – loads and parses `/proc/<pid>/status`  
//! - `read_process_comm` – loads `/proc/<pid>/comm`  
//! - `read_process` – combines name + memory  
//! - `list_processes_from` – scans `/proc` and collects `(pid, name, rss_kb)`  
//! - `print_top_processes` – prints process information using
//!   `procs.iter().take(N)`  
//!
//! Pure parsing code is fully covered by unit tests.  
//! The `/proc`-dependent parts are small and well-isolated.
//!
//! # Platform Requirements
//!
//! This program requires a **Linux system** with a **procfs** mounted at
//! `/proc`.  
//! It will not run on:
//!
//! - macOS
//! - Windows
//! - WSL without `/proc`
//! - Containers where `/proc` is restricted
//!
//! # Permissions
//!
//! The program does **not** require root privileges.  
//! All accessed files are normally world-readable, including for processes
//! owned by `root`:
//!
//! - `/proc/<pid>/comm`
//! - `/proc/<pid>/status` (except `io` or `cmdline` fields)
//!
//! It does **not** access restricted files like:
//!
//! - `/proc/<pid>/mem`
//! - `/proc/<pid>/io`
//!
//! # Usage
//!
//! ```bash
//! cargo run
//! ```
//!
//! The screen updates once per second.  
//! Press `Ctrl+C` to exit.
//!
//! # Educational Purpose
//!
//! This program is intentionally kept small and readable to serve as:
//!
//! - a training exercise for Rust beginners  
//! - an introduction to systems-level programming in Rust  
//! - an example for structured parsing and `/proc` exploration  
//!
//! It can be easily extended with:
//!
//! - CPU usage calculation from `/proc/stat`
//! - sorting modes
//! - terminal UI libraries
//! - htop-like process interaction
//! - async / tokio support
//!
//! ---

use std::{fs, io, thread, time::Duration};

// ------------------------------------------
// Memory parsing
// ------------------------------------------

/// Parses the content of `/proc/meminfo`.
///
/// Returns `(total_kb, available_kb)`.
pub fn parse_meminfo(content: &str) -> io::Result<(u64, u64)> {
    let mut total = 0;
    let mut available = 0;

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("MemTotal:") {
            total = value.split_whitespace().next().unwrap().parse().unwrap();
        } else if let Some(value) = line.strip_prefix("MemAvailable:") {
            available = value.split_whitespace().next().unwrap().parse().unwrap();
        }
    }

    Ok((total, available))
}

/// Reads `/proc/meminfo` from a custom path (testable).
pub fn read_meminfo_from(path: &str) -> io::Result<(u64, u64)> {
    let content = fs::read_to_string(path)?;
    parse_meminfo(&content)
}

// ------------------------------------------
// Process parsing
// ------------------------------------------

/// Parses the content of `/proc/<pid>/status`
/// and returns the `VmRSS` field in kB if found.
pub fn parse_process_status(status: &str) -> Option<u64> {
    status
        .lines()
        .find(|l| l.starts_with("VmRSS:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|val| val.parse::<u64>().ok())
}

/// Reads only the `/proc/<pid>/status` file.
pub fn read_process_status(base: &str, pid: &str) -> Option<u64> {
    let path = format!("{}/{}/status", base, pid);
    let content = fs::read_to_string(path).ok()?;
    parse_process_status(&content)
}

/// Reads only the `/proc/<pid>/comm` file.
pub fn read_process_comm(base: &str, pid: &str) -> String {
    let path = format!("{}/{}/comm", base, pid);
    fs::read_to_string(path).unwrap_or_default().trim().to_string()
}

/// Combines status + comm info.
///
/// Returns `(comm, memory_kb)` or `None`.
pub fn read_process(base: &str, pid: &str) -> Option<(String, u64)> {
    let name = read_process_comm(base, pid);
    let mem = read_process_status(base, pid)?;
    Some((name, mem))
}

// ------------------------------------------
// Process listing
// ------------------------------------------

/// Scans a `/proc` directory and returns a list of `(pid, name, rss_kb)`.
pub fn list_processes_from(base: &str) -> io::Result<Vec<(String, String, u64)>> {
    let mut out = Vec::new();

    for entry in fs::read_dir(base)? {
        let entry = entry?;
        let pid = entry.file_name().to_string_lossy().to_string();

        if pid.chars().all(|c| c.is_ascii_digit()) {
            if let Some((name, mem)) = read_process(base, &pid) {
                out.push((pid, name, mem));
            }
        }
    }

    Ok(out)
}

// ------------------------------------------
// Printing logic
// ------------------------------------------

/// Prints the top N processes by memory usage.
///
/// `procs` is a list of tuples `(pid, name, mem_kb)`.
///
/// Uses: `procs.iter().take(5)`
pub fn print_top_processes(procs: &[(String, String, u64)], n: usize) {
    println!("\nTop {} processes by memory:", n);

    for (pid, name, mem) in procs.iter().take(n) {
        println!("{:<6} {:<20} {} kB", pid, name, mem);
    }
}

// ------------------------------------------
// Main
// ------------------------------------------

fn main() -> io::Result<()> {
    loop {
        let (total, free) = read_meminfo_from("/proc/meminfo")?;
        println!(
            "Memory: total={}kB free={}kB used={}kB",
            total,
            free,
            total.saturating_sub(free)
        );

        let mut procs = list_processes_from("/proc")?;
        procs.sort_by(|a, b| b.2.cmp(&a.2)); // sort descending

        print_top_processes(&procs, 5);

        thread::sleep(Duration::from_secs(1));
    }
}

// ------------------------------------------
// Tests
// ------------------------------------------

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
