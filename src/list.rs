pub fn items() -> Vec<u32> {
    Vec::new()
}

pub fn first(list: &[u32]) -> Option<u32> {
    list.first().copied()
}
