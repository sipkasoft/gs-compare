pub fn parse(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(|l| l.split_once("="))
        .map(|(k, v)| (k.trim().into(), v.trim().into()))
        .collect()
}
