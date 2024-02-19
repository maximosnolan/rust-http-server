use std::{env, fs, io};

const INTEGRATION_DIR : &str = "tests/integration/sample_requests";

pub fn read_sample_request(request_name: &str) -> Result<String, io::Error> {
    let mut test_dir = env::current_dir()?;

    test_dir.push(INTEGRATION_DIR);
    test_dir.push(request_name);

    // Convert the path to a string
    let dir_str = test_dir.to_string_lossy().to_string();
    print!("path: {}\n", dir_str);
    let contents = fs::read_to_string(dir_str)?;

    let trimmed_contents: String = contents.trim().to_string();

    Ok(trimmed_contents)
}
