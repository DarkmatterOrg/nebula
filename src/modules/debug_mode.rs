pub fn running_in_debug() -> bool {
    if cfg!(debug_assertions) {
        true
    } else {
        false
    }
}
