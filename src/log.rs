pub fn log(level: u8, line: &str) {
    if level > 0 {
        eprintln!("[{level}] {line}");
    }
}
