use std::fs::read_to_string;
use std::path::Path;

// Searches for "Forwarded Port Updated" in the logs file, from end to start. If finds it, parses
// the port from the previous line.
//
// Example logs:
//
// 2026-09-04T10:48:54.268839+00:00 | proton.vpn.platform/port_forwarding.rs:240 | INFO | Receiving Response { version: 0, operation: 130, response_code: 0, gateway_epoch_seconds: 160514, internal_port: 55755, external_port: 55755, lifetime_seconds: 60 }
// 2026-09-04T10:48:54.268929+00:00 | proton.vpn.platform/listener.rs:118 | INFO | Forwarded port updated.
pub fn extract_last_port(path: &Path) -> Option<u16> {
    // TODO: This can be heavily optimized by reading lines of the file end to start, and by keeping track of
    // the index of last last line checked to avoid checking already parsed lines in future calls.
    let lines: Vec<String> = read_to_string(path)
        .ok()?
        .lines()
        .map(|l| l.to_string())
        .collect();

    for (i, line) in lines.iter().rev().enumerate() {
        if !line.contains("Forwarded port updated") {
            continue;
        }

        let Some(prev_line) = lines.get(i - 1) else {
            continue;
        };

        let Some(idx) = prev_line.find("internal_port") else {
            continue;
        };

        return prev_line[idx + 15..idx + 20].parse::<u16>().ok();
    }

    None
}
