

pub fn remove_first_slash(method: &str) -> &str {
    // Attempt to remove the first '/' character from the string
    if let Some(rest) = method.strip_prefix('/') {
        rest
    } else {
        // If the string doesn't start with '/', return the original string
        method
    }
}