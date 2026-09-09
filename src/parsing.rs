use std::fs::read_to_string;
use std::path::Path;

const PREFIX_LEN: usize = 15; // the length of "internal_port: "
const PORT_LEN: usize = 5; // the length of "internal_port: "

pub struct Match {
    pub port: u16,
    pub offset: usize,
}

// Searches for "Forwarded Port Updated" in the logs file, from end to start. If finds it, parses
// the port from the previous line.
//
// Example logs:
//
// 2026-09-04T10:48:54.268839+00:00 | proton.vpn.platform/port_forwarding.rs:240 | INFO | Receiving Response { version: 0, operation: 130, response_code: 0, gateway_epoch_seconds: 160514, internal_port: 55755, external_port: 55755, lifetime_seconds: 60 }
// 2026-09-04T10:48:54.268929+00:00 | proton.vpn.platform/listener.rs:118 | INFO | Forwarded port updated.
pub fn extract_last_port(path: &Path, offset: usize) -> Option<Match> {
    // TODO: This can be heavily optimized by using iterators to not load the entire file contents into memory.
    let lines: Vec<String> = read_to_string(path)
        .ok()?
        .lines()
        .map(|l| l.to_string())
        .collect();

    let mut offset = offset;

    // The logfile got trimmed or re-created
    if lines.len() < offset {
        offset = 0
    }

    for (i, line) in lines.iter().enumerate().rev() {
        if i < offset {
            return None;
        }

        if !line.contains("Forwarded port updated") {
            continue;
        }

        let Some(prev_line) = lines.get(i - 1) else {
            continue;
        };

        let Some(idx) = prev_line.find("internal_port") else {
            continue;
        };

        return match prev_line[idx + PREFIX_LEN..idx + PREFIX_LEN + PORT_LEN].parse::<u16>() {
            Ok(p) => Some(Match { port: p, offset: i }),
            Err(_) => None,
        };
    }

    None
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn fixture_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join("test.log")
    }

    #[test]
    fn test_extract_last_port() {
        let path = fixture_path();

        let match_ = extract_last_port(&path, 0).unwrap();
        assert_eq!(match_.port, 61005);
        assert_eq!(match_.offset, 19);
    }

    #[test]
    fn test_extract_last_port_offset_too_high() {
        let path = fixture_path();

        let match_ = extract_last_port(&path, 999).unwrap();
        assert_eq!(match_.port, 61005);
        assert_eq!(match_.offset, 19);
    }
}
