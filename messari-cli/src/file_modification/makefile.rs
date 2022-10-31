use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use crate::utils::get_relative_path;

#[derive(Debug)]
pub(crate) struct MakeFile {
    makefile_dir: PathBuf, // This is parent directory for the Makefile
    commands: Vec<MakeCommand>
}

impl MakeFile {
    pub(crate) fn new(makefile_filepath: &PathBuf) -> Self {
        let makefile_dir = makefile_filepath.parent().unwrap().to_path_buf();

        MakeFile {
            makefile_dir,
            commands: vec![]
        }
    }

    pub(crate) fn load_from_file(makefile_filepath: &PathBuf) -> Self {
        let makefile_dir = makefile_filepath.parent().unwrap().to_path_buf();

        let re_newline_spaces = Regex::new(r"[ \t]*\n *").unwrap();
        let re_start_new_lines = Regex::new(r"^\n*").unwrap();
        let re_end_new_lines = Regex::new(r"\n*$").unwrap();
        let re_command_block_gaps = Regex::new(r"\n{2,}").unwrap();

        let mut makefile_contents = fs::read_to_string(makefile_filepath).expect(&format!("Unable to read Makefile contents! Filepath: {}", makefile_filepath.to_string_lossy()));
        makefile_contents = makefile_contents.replace("\r\n", "\n").to_string();
        makefile_contents = re_newline_spaces.replace_all(&makefile_contents, "\n").to_string();
        makefile_contents = re_start_new_lines.replace(&makefile_contents, "").to_string();
        makefile_contents = re_end_new_lines.replace(&makefile_contents, "").to_string();
        makefile_contents = re_command_block_gaps.replace_all(&makefile_contents, "\n\n").to_string();

        let commands = makefile_contents.split("\n\n").map(|block_str| parse_block(block_str, &makefile_filepath)).collect();

        MakeFile {
            makefile_dir,
            commands
        }
    }

    /// Returns true if an edit to the Makefile was made. (false if no changes made)
    pub(crate) fn add_project_to_build_all_command(&mut self, project_dir: &PathBuf) -> bool {
        let project = get_relative_path(&self.makefile_dir, project_dir);

        for command in self.commands.iter_mut() {
            if command.command_name == "build-all".to_string() {
                let build_all_projects = command.get_build_all_projects();
                return if build_all_projects.contains(&project) {
                    false
                } else {
                    command.operations.push(format!("$(MAKE) -C {} build", project));
                    true
                }
            }
        }

        self.commands.push(MakeCommand {
            command_name: "build-all".to_string(),
            operations: vec![format!("$(MAKE) -C {} build", project)]
        });

        true
    }

    /// Returns true if an edit to the Makefile was made. (false if no changes made - due to build command already existing)
    pub(crate) fn add_build_operation(&mut self) -> bool {
        for command in self.commands.iter() {
            if command.command_name == "build".to_string() {
                return false;
            }
        }

        self.commands.push(MakeCommand {
            command_name: "build".to_string(),
            operations: vec!["cargo build --target wasm32-unknown-unknown --release".to_string()]
        });

        true
    }

    pub(crate) fn get_file_contents(self) -> String {
        let command_block_strings = self.commands.iter().map(|command| command.to_string()).collect::<Vec<_>>();
        command_block_strings.join("\n")
    }
}

fn parse_block(block_str: &str, makefile_filepath: &PathBuf) -> MakeCommand {
    let mut lines = block_str.lines().into_iter();

    let first_line = lines.next().unwrap();
    if !first_line.starts_with(".PHONY: ") {
        panic!("First line of each command block should always start with '.PHONY: ' and here it did not!\nFirst line: {}\nCommand block: {}\n\
        Makefile filepath: {}", first_line, block_str, makefile_filepath.to_string_lossy())
    }

    let command_name = first_line[8..].to_string();

    let second_line = lines.next().expect(&format!("There was no second line for command block!\nCommand block: {}\nMakefile filepath: {}",
                                                   block_str, makefile_filepath.to_string_lossy()));
    if second_line != &format!("{}:", command_name) {
        panic!("Command used on the second line of command block is not equal to the command name on the first line of command block!\nCommand block: {}\n\
        Makefile filepath: {}\nBlock should start in the format:\n.PHONY: COMMAND_NAME\nCOMMAND_NAME:\nOperation lines..", block_str, makefile_filepath.to_string_lossy());
    }

    let operations = lines.map(|line_str| parse_operation(line_str, block_str, makefile_filepath)).collect();

    MakeCommand {
        command_name,
        operations
    }
}

fn parse_operation(line_str: &str, command_block: &str, makefile_filepath: &PathBuf) -> String {
    if !line_str.starts_with("\t") {
        panic!("All operations have to start with a tab and the following line did not!\nLine: {}\nCommand block: {}\nMakefile filepath: {}",
                       line_str, command_block, makefile_filepath.to_string_lossy());
    }

    line_str[1..].to_string()
}

#[derive(Debug)]
pub(crate) struct MakeCommand {
    command_name: String,
    operations: Vec<String>
}

impl MakeCommand {
    fn get_build_all_projects(&self) -> Vec<String> {
        self.operations.iter().filter_map(|operation| {
            if operation.starts_with("$(MAKE) -C ") && operation.ends_with(" build") {
                let operation_args = operation.split(" ").collect::<Vec<_>>();
                if operation_args.len() == 4 {
                    return Some(operation_args[2].to_string());
                }
            }
            None
        }).collect()
    }
}

impl Display for MakeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let operations_lines = format!("{}", self.operations.join("\n\t"));
        write!(f, ".PHONY: {0}\n{0}:\n\t{1}\n", self.command_name, operations_lines)
    }
}