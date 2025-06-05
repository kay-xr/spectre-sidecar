use regex::Regex;

/// This is where you can set regex patterns to return in the logs, other entries not in this list
/// will be ignored. For example, `Regex::new(r"(?i)\bError\b").unwrap()` will return all messages
/// containing the "Error" string.
pub fn get_patterns() -> Vec<Regex> {
    let patterns: Vec<Regex> = vec![
        Regex::new(r"\[Video Playback\] ERROR:").unwrap(),
        Regex::new(r"\[Behaviour\] OnPlayerJoined").unwrap(),
        Regex::new(r"\[Behaviour\] OnPlayerLeft").unwrap()
    ];
    
    patterns
}