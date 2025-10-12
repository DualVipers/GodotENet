pub fn clean_path(mut path: String) -> String {
    // Remove Trailing `0 if it exists
    if path.ends_with("\0") {
        path.pop();
    }

    path
}
