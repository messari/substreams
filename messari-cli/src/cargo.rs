use std::fs;
use std::path::PathBuf;
use regex::Regex;

pub(crate) fn add_member_to_workspace(member: &String, cargo_filepath: &PathBuf) -> String {
    let cargo_file_contents = match fs::read_to_string(cargo_filepath) {
        Ok(cargo_file_contents) => cargo_file_contents,
        Err(error) => panic!("Error reading cargo file: {}\nError: {}", cargo_filepath.to_string_lossy(), error)
    };

    let mut lines = cargo_file_contents.lines().into_iter().collect::<Vec<_>>();
    let mut members_line_number = 0;
    let re_members_line = Regex::new(r"^members[\s\t]+=[\s\t]+\[[^\]]*$").unwrap();
    for (line_number, line) in lines.iter().enumerate() {
        if re_members_line.is_match(*line) {
            members_line_number = line_number;
        }
    }
    if members_line_number == 0 {
        // members line can't on the first line so it means that it has not been found
        panic!("Was unable to find workspace members section in cargo.toml file: {}, cargo.toml file contents:\n{}", cargo_filepath.to_string_lossy(), cargo_file_contents);
    }
    lines.insert(members_line_number+1, &format!("    {},", member));

    lines.join("\n")
}

pub(crate) fn add_build_dependencies(build_dependencies: Vec<String>, cargo_filepath: &PathBuf) -> String {
    let mut cargo_file_contents = match fs::read_to_string(cargo_filepath) {
        Ok(cargo_file_contents) => cargo_file_contents,
        Err(error) => panic!("Error reading cargo file: {}\nError: {}", cargo_filepath.to_string_lossy(), error)
    };

    // For now assuming that build dependencies just haven't been included yet
    cargo_file_contents.push_str(&format!("\n[build-dependencies]\n{}", build_dependencies.join("\n")));
    cargo_file_contents
}