//! A minimal, educational "top-like" system monitor implemented in Rust.

use std::{fmt::Display, fs, io, thread, time::Duration};

// ------------------------------------------
// Memory information
// ------------------------------------------

/// Struct to store information from `/proc/meminfo`.
struct MemInfo {
    total: u64,
    available: u64,
}

impl MemInfo {
    /// Reads `/proc/meminfo` from a custom path.
    fn from_file(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::parse_from_str(&content)
    }

    /// Parses the content of `/proc/meminfo`.
    fn parse_from_str(content: &str) -> io::Result<Self> {
        let mut total = None;
        let mut available = None;

        fn parse_value(line: &str) -> io::Result<u64> {
            line.trim()
                .split_once(' ')
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "No whitespace found".to_owned())
                })?
                .0
                .parse::<u64>()
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))
        }

        for line in content.lines() {
            if let Some(value) = line.strip_prefix("MemTotal:") {
                total = Some(parse_value(value)?);
            } else if let Some(value) = line.strip_prefix("MemAvailable:") {
                available = Some(parse_value(value)?);
            }
        }

        if let (Some(total), Some(available)) = (total, available) {
            Ok(Self { total, available })
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Didn't find MemTotal and MemAvailable".to_owned(),
            ))
        }
    }

    /// Calculate the amount of used memory.
    fn used(&self) -> u64 {
        self.total.saturating_sub(self.available)
    }
}

// Make `MemInfo` printable
impl Display for MemInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Memory: total={}kB free={}kB used={}kB",
            self.total,
            self.available,
            self.used()
        )
    }
}

// ------------------------------------------
// Process parsing
// ------------------------------------------

/// Parses the content of `/proc/<pid>/status`
/// and returns the `VmRSS` field in kB if found.
fn parse_process_status(status: &str) -> Option<u64> {
    status
        .lines()
        .find(|l| l.starts_with("VmRSS:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|val| val.parse::<u64>().ok())
}

/// Reads only the `/proc/<pid>/status` file.
fn read_process_status(base: &str, pid: &str) -> Option<u64> {
    let path = format!("{}/{}/status", base, pid);
    let content = fs::read_to_string(path).ok()?;
    parse_process_status(&content)
}

/// Reads only the `/proc/<pid>/comm` file.
fn read_process_comm(base: &str, pid: &str) -> String {
    let path = format!("{}/{}/comm", base, pid);
    fs::read_to_string(path)
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// Combines status + comm info.
///
/// Returns `(comm, memory_kb)` or `None`.
fn read_process(base: &str, pid: &str) -> Option<(String, u64)> {
    let name = read_process_comm(base, pid);
    let mem = read_process_status(base, pid)?;
    Some((name, mem))
}

// ------------------------------------------
// Process listing
// ------------------------------------------

/// Scans a `/proc` directory and returns a list of `(pid, name, rss_kb)`.
fn list_processes_from(base: &str) -> io::Result<Vec<(String, String, u64)>> {
    let mut out = Vec::new();

    for entry in fs::read_dir(base)? {
        let entry = entry?;
        let pid = entry.file_name().to_string_lossy().to_string();

        if pid.chars().all(|c| c.is_ascii_digit())
            && let Some((name, mem)) = read_process(base, &pid)
        {
            out.push((pid, name, mem));
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
fn print_top_processes(procs: &[(String, String, u64)], n: usize) {
    println!("Top {} processes by memory:", n);

    for (pid, name, mem) in procs.iter().take(n) {
        println!("{:<6} {:<20} {} kB", pid, name, mem);
    }
}

// ------------------------------------------
// Main
// ------------------------------------------

fn main() -> io::Result<()> {
    loop {
        print!("\u{001b}c"); // Clear screen

        let meminfo = MemInfo::from_file("/proc/meminfo")?;
        println!("{meminfo}");

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
SomeOtherValue:  123567 kB
MemAvailable:    2345678 kB";

        let meminfo = MemInfo::parse_from_str(input).unwrap();
        assert_eq!(meminfo.total, 16384256);
        assert_eq!(meminfo.available, 2345678);
    }

    #[test]
    fn test_parse_process_status() {
        let input = "Name: myproc\nVmRSS:   1234 kB\n";
        let mem = parse_process_status(input);
        assert_eq!(mem, Some(1234));
    }
}
