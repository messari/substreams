use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;

/// This command should be used whenever modifying file contents as it provides a safe rollback operation encase the modification gets interrupted by an error mid-way through processing
pub(crate) fn safely_modify_file_contents(
    file_contents_modifications: Vec<FileContentsModification>,
) {
    // TODO: Get this working for ctrl-c interrupts also

    let mut reverse_operations = Vec::new();
    for file_contents_modification in file_contents_modifications {
        let reverse_operation = match file_contents_modification.get_reverse_operation() {
            Ok(reverse_operation) => reverse_operation,
            Err(error) => {
                if !reverse_operations.is_empty() {
                    panic!("Failed to reverse operation: {}!\nError: {}\n\nNo changes were required to be rolled back.", file_contents_modification, error);
                } else {
                    println!(
                        "Failed to reverse operation: {}!\nError: {}\n",
                        file_contents_modification, error
                    );
                    rollback_file_contents_changes(reverse_operations);
                    return;
                }
            }
        };

        let operation_display_string = file_contents_modification.to_string();
        if let Err(error) = file_contents_modification.apply_operation() {
            if !reverse_operations.is_empty() {
                panic!("Failed to apply operation: {}!\nError: {}\n\nNo changes were required to be rolled back.", operation_display_string, error);
            } else {
                println!(
                    "Failed to apply operation: {}!\nError: {}\n",
                    operation_display_string, error
                );
                rollback_file_contents_changes(reverse_operations);
                return;
            }
        }

        reverse_operations.push(reverse_operation);
    }
}

fn rollback_file_contents_changes(rollback_operations: Vec<FileContentsModification>) {
    let rollback_operations_iter = rollback_operations.into_iter();
    for rollback_operation in rollback_operations_iter {
        let operation_display_string = rollback_operation.to_string();
        if let Err(error) = rollback_operation.apply_operation() {
            // TODO: Print a command like "messari rollback {..info..}" in the terminal which a user can
            // TODO: copy and paste to try to manually rollback changes even after program has crashed
            panic!("Failed to rollback operation: {}\nError:{}\n\nManual rollback command:\nmessari rollback TODO..", operation_display_string, error);
        }
    }

    panic!("Rollback complete - exiting program");
}

pub(crate) enum FileContentsModification {
    CreateFile(File),
    UpdateFile(File),
    DeleteFile(PathBuf),
    CreateFolder(PathBuf),
    DeleteFolder(PathBuf),
}

impl Display for FileContentsModification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileContentsModification::CreateFile(file) => {
                write!(f, "CreateFile({})", file.filepath.to_string_lossy())
            }
            FileContentsModification::UpdateFile(file) => {
                write!(f, "UpdateFile({})", file.filepath.to_string_lossy())
            }
            FileContentsModification::DeleteFile(filepath) => {
                write!(f, "DeleteFile({})", filepath.to_string_lossy())
            }
            FileContentsModification::CreateFolder(folder_path) => {
                write!(f, "CreateFolder({})", folder_path.to_string_lossy())
            }
            FileContentsModification::DeleteFolder(folder_path) => {
                write!(f, "DeleteFolder({})", folder_path.to_string_lossy())
            }
        }
    }
}

impl FileContentsModification {
    fn get_reverse_operation(&self) -> Result<FileContentsModification, std::io::Error> {
        match self {
            FileContentsModification::CreateFile(file) => {
                Ok(FileContentsModification::DeleteFile(file.filepath.clone()))
            }
            FileContentsModification::UpdateFile(file) => {
                let current_file_contents = fs::read_to_string(&file.filepath)?;
                Ok(FileContentsModification::UpdateFile(File {
                    filepath: file.filepath.clone(),
                    file_contents: current_file_contents,
                }))
            }
            FileContentsModification::DeleteFile(filepath) => {
                let current_file_contents = fs::read_to_string(filepath)?;
                Ok(FileContentsModification::CreateFile(File {
                    filepath: filepath.clone(),
                    file_contents: current_file_contents,
                }))
            }
            FileContentsModification::CreateFolder(folder_path) => {
                Ok(FileContentsModification::DeleteFolder(folder_path.clone()))
            }
            FileContentsModification::DeleteFolder(folder_path) => {
                Ok(FileContentsModification::CreateFolder(folder_path.clone()))
            }
        }
    }

    // Destroys obj to make sure that you don't accidentally call the operation twice..
    fn apply_operation(self) -> std::io::Result<()> {
        match self {
            FileContentsModification::CreateFile(file) => {
                fs::write(file.filepath, file.file_contents)
            }
            FileContentsModification::UpdateFile(file) => {
                fs::write(file.filepath, file.file_contents)
            }
            FileContentsModification::DeleteFile(filepath) => fs::remove_file(filepath),
            FileContentsModification::CreateFolder(folder_path) => fs::create_dir_all(folder_path),
            FileContentsModification::DeleteFolder(folder_path) => fs::remove_dir_all(folder_path),
        }
    }
}

pub(crate) struct File {
    pub(crate) filepath: PathBuf,
    pub(crate) file_contents: String,
}
