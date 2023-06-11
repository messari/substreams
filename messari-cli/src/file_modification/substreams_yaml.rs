use crate::utils::{get_relative_path, get_repo_root_folder};
use linked_hash_map::LinkedHashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use yaml_rust::yaml::Hash;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};
use clap::ValueEnum;

#[derive(Clone, ValueEnum, PartialEq)]
pub(crate) enum VersionType {
    Major,
    Minor,
    Patch
}

impl VersionType {
    pub(crate) fn get_version_char(&self) -> char {
        match self {
            VersionType::Major => 'M',
            VersionType::Minor => 'K',
            VersionType::Patch => 'P'
        }
    }

    pub(crate) fn from_char(character: char) -> Self {
        match character {
            'M' => VersionType::Major,
            'K' => VersionType::Minor,
            'P' => VersionType::Patch,
            _ => panic!("Can only use M, K or P characters to declare version increment type!! Letter given: {}", character)
        }
    }
}

impl Display for VersionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionType::Major => write!(f, "Major"),
            VersionType::Minor => write!(f, "Minor"),
            VersionType::Patch => write!(f, "Patch")
        }
    }
}

pub(crate) struct SubstreamsYaml {
    substreams_yaml_dir: PathBuf, // This is parent directory for the substreams.yaml file
    yaml: Yaml,
}

impl SubstreamsYaml {
    pub(crate) fn new(project_name: &str, substreams_filepath: &PathBuf) -> Self {
        let substreams_yaml_dir = substreams_filepath.parent().unwrap().to_path_buf();
        let wasm_filepath = get_repo_root_folder()
            .join("target")
            .join("wasm32-unknown-unknown")
            .join("release")
            .join(format!("{}.wasm", project_name));
        let relative_wasm_filepath = get_relative_path(&substreams_yaml_dir, &wasm_filepath);
        let yaml_contents = format!(
            "specVersion: v0.1.0\n\
            package:\n    \
                name: \"{}\"\n    \
                version: v0.1.0\n\
            \n\
            imports:\n    \
                eth: https://github.com/streamingfast/sf-ethereum/releases/download/v0.10.2/ethereum-v0.10.4.spkg\n\
            \n\
            binaries:\n    \
                default:\n        \
                    type: wasm/rust-v1\n        \
                    file: {}",
            project_name,
            relative_wasm_filepath
        );

        SubstreamsYaml {
            substreams_yaml_dir,
            yaml: YamlLoader::load_from_str(yaml_contents.as_str()).unwrap()[0].clone(),
        }
    }

    pub(crate) fn load_from_file(substreams_yaml_filepath: &PathBuf) -> Self {
        let substreams_yaml_dir = substreams_yaml_filepath.parent().unwrap().to_path_buf();

        let yaml_contents = fs::read_to_string(substreams_yaml_filepath).expect(&format!(
            "Unable to read substreams_yaml contents! Filepath: {}",
            substreams_yaml_filepath.to_string_lossy()
        ));
        let yaml = YamlLoader::load_from_str(yaml_contents.as_str()).expect(&format!(
            "Unable to read substreams_yaml contents! Filepath: {}\nFile contents: {}",
            substreams_yaml_filepath.to_string_lossy(),
            yaml_contents
        ))[0]
            .clone();

        SubstreamsYaml {
            substreams_yaml_dir,
            yaml,
        }
    }

    pub(crate) fn get_substream_name(&self) -> String {
        let contents_hashmap = self.get_contents_hashmap();

        let package = if let Some(package) = contents_hashmap.get(&Yaml::from_str("package")) {
            package
        } else {
            panic!("Error with yaml file - package section does not exist! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
        };
        if let Yaml::Hash(package_hashmap) = package {
            let version = if let Some(version) = package_hashmap.get(&Yaml::from_str("name")) {
                version
            } else {
                panic!("Error with yaml file - name section is not found in package section! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
            };
            if let Yaml::String(version_string) = version {
                version_string.clone()
            } else {
                panic!("Error with yaml file - name section is not a string! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
            }
        } else {
            panic!("Error with yaml file - package section is not a hashmap! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
        }
    }

    /// Returns spkg version in form X.Y.Z
    pub(crate) fn get_version(&self) -> String {
        let contents_hashmap = self.get_contents_hashmap();

        let package = if let Some(package) = contents_hashmap.get(&Yaml::from_str("package")) {
            package
        } else {
            panic!("Error with yaml file - package section does not exist! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
        };
        let version = if let Yaml::Hash(package_hashmap) = package {
            let version = if let Some(version) = package_hashmap.get(&Yaml::from_str("version")) {
                version
            } else {
                panic!("Error with yaml file - version section is not found in package section! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
            };
            if let Yaml::String(version_string) = version {
                version_string
            } else {
                panic!("Error with yaml file - version section is not a string! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
            }
        } else {
            panic!("Error with yaml file - package section is not a hashmap! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
        };

        // We will make sure to represent the version in form: X.Y.Z
        let v_semver = Regex::new(r"^v\d+.\d+.\d+$").unwrap();
        let semver = Regex::new(r"^\d+.\d+.\d+$").unwrap();
        if v_semver.is_match(version) {
            version[1..].to_string()
        } else if semver.is_match(version) {
            version.clone()
        } else {
            panic!("Couldn't extract proper versioning from spkg! Expecting version to be either in form: vX.Y.Z or X.Y.Z - actual version given: {}", version);
        }
    }

    pub(crate) fn modify_version(&mut self, increment_version: VersionType, decrement_version: Option<VersionType>) {
        let contents_hashmap = self.get_contents_hashmap_mut();

        let package = if let Some(package) = contents_hashmap.get_mut(&Yaml::from_str("package")) {
            package
        } else {
            panic!("Error with yaml file - package section does not exist! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
        };
        let version = if let Yaml::Hash(package_hashmap) = package {
            let version = if let Some(version) = package_hashmap.get_mut(&Yaml::from_str("version")) {
                version
            } else {
                panic!("Error with yaml file - version section is not found in package section! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
            };
            if let Yaml::String(version_string) = version {
                version_string
            } else {
                panic!("Error with yaml file - version section is not a string! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
            }
        } else {
            panic!("Error with yaml file - package section is not a hashmap! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
        };

        // Only expecting spkg_version to be in forms: either vX.Y.Z or X.Y.Z
        let v_semver = Regex::new(r"^v\d+.\d+.\d+$").unwrap();
        let semver = Regex::new(r"^\d+.\d+.\d+$").unwrap();
        let (semver_str, starts_with_v) = if v_semver.is_match(version) {
            (version[1..].to_string(), true)
        } else if semver.is_match(version) {
            (version.clone(), false)
        } else {
            panic!("Couldn't extract proper versioning from spkg! Expecting version to be either in form: vX.Y.Z or X.Y.Z - actual version given: {}", version);
        };

        let mut semver_iter = semver_str.split(".").into_iter();

        let new_version = if let Some(existing_version_increment) = decrement_version {
            if existing_version_increment == increment_version {
                return;
            }

            match (increment_version, existing_version_increment) {
                (VersionType::Major, VersionType::Minor) => {
                    let major_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    let mut new_version = (major_version + 1).to_string();
                    new_version.push('.');
                    let minor_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    new_version.push_str(&(minor_version - 1).to_string());
                    new_version.push('.');
                    new_version.push_str(semver_iter.next().unwrap());
                    new_version
                },
                (VersionType::Major, VersionType::Patch) => {
                    let major_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    let mut new_version = (major_version + 1).to_string();
                    new_version.push('.');
                    new_version.push_str(semver_iter.next().unwrap());
                    new_version.push('.');
                    let patch_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    new_version.push_str(&(patch_version - 1).to_string());
                    new_version
                },
                (VersionType::Minor, VersionType::Major) => {
                    let major_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    let mut new_version = (major_version - 1).to_string();
                    new_version.push('.');
                    let minor_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    new_version.push_str(&(minor_version + 1).to_string());
                    new_version.push('.');
                    new_version.push_str(semver_iter.next().unwrap());
                    new_version
                },
                (VersionType::Minor, VersionType::Patch) => {
                    let mut new_version = semver_iter.next().unwrap().to_string();
                    new_version.push('.');
                    let minor_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    new_version.push_str(&(minor_version + 1).to_string());
                    new_version.push('.');
                    let patch_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    new_version.push_str(&(patch_version - 1).to_string());
                    new_version
                },
                (VersionType::Patch, VersionType::Major) => {
                    let major_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    let mut new_version = (major_version - 1).to_string();
                    new_version.push('.');
                    new_version.push_str(semver_iter.next().unwrap());
                    new_version.push('.');
                    let patch_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    new_version.push_str(&(patch_version + 1).to_string());
                    new_version
                },
                (VersionType::Patch, VersionType::Minor) => {
                    let mut new_version = semver_iter.next().unwrap().to_string();
                    new_version.push('.');
                    let minor_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    new_version.push_str(&(minor_version - 1).to_string());
                    new_version.push('.');
                    let patch_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    new_version.push_str(&(patch_version + 1).to_string());
                    new_version
                },
                _ => unreachable!()
            }
        } else {
            match increment_version {
                VersionType::Major => {
                    let major_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    let mut new_version = (major_version + 1).to_string();
                    new_version.push('.');
                    new_version.push_str(semver_iter.next().unwrap());
                    new_version.push('.');
                    new_version.push_str(semver_iter.next().unwrap());
                    new_version
                }
                VersionType::Minor => {
                    let mut new_version = semver_iter.next().unwrap().to_string();
                    new_version.push('.');
                    let minor_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    new_version.push_str(&(minor_version + 1).to_string());
                    new_version.push('.');
                    new_version.push_str(semver_iter.next().unwrap());
                    new_version
                }
                VersionType::Patch => {
                    let mut new_version = semver_iter.next().unwrap().to_string();
                    new_version.push('.');
                    new_version.push_str(semver_iter.next().unwrap());
                    new_version.push('.');
                    let patch_version = semver_iter.next().unwrap().parse::<u8>().unwrap();
                    new_version.push_str(&(patch_version + 1).to_string());
                    new_version
                }
            }
        };

        if starts_with_v {
            *version = format!("v{}", new_version);
        } else {
            *version = new_version;
        }
    }

    /// Returns the project path of an local spkg dependency
    pub(crate) fn get_local_spkg_dependencies(&self) -> Vec<PathBuf> {
        let contents_hashmap = self.get_contents_hashmap();

        if let Some(imports) = contents_hashmap.get(&Yaml::from_str("imports")) {
            if let Yaml::Hash(imports_hashmap) = imports {
                return imports_hashmap.values().into_iter().filter_map(|x| {
                    let spkg_dependency = x.as_str().unwrap();
                    if spkg_dependency.starts_with("https://") {
                        None
                    } else {
                        let full_path = self.substreams_yaml_dir.join(spkg_dependency);
                        if full_path.exists() {
                            Some(full_path.canonicalize().unwrap())
                        } else {
                            panic!("Path for spkg dependency: {}, could not be found!", spkg_dependency)
                        }
                    }
                }).collect();
            } else {
                panic!("Error with yaml file - imports section is not a hashmap! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
            }
        }

        Vec::new()
    }

    /// Returns true if an edit to the substreams.yaml was made. (false if no changes made)
    pub(crate) fn add_protobuf_files(&mut self, protobuf_file_paths: Vec<PathBuf>) -> bool {
        if protobuf_file_paths.is_empty() {
            return false;
        }

        let substreams_yaml_dir = self.substreams_yaml_dir.clone();
        let contents_hashmap = self.get_contents_hashmap_mut();

        let protobuf_hashmap = if let Some(protobuf) =
            contents_hashmap.get_mut(&Yaml::from_str("protobuf"))
        {
            if let Yaml::Hash(protobuf_hashmap) = protobuf {
                protobuf_hashmap
            } else {
                panic!("Error with yaml file - protobuf section is not a hashmap! Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
            }
        } else {
            contents_hashmap.insert(Yaml::from_str("protobuf"), Yaml::Hash(LinkedHashMap::new()));
            let protobuf = contents_hashmap
                .get_mut(&Yaml::from_str("protobuf"))
                .unwrap();
            if let Yaml::Hash(protobuf_hashmap) = protobuf {
                protobuf_hashmap
            } else {
                unreachable!()
            }
        };

        let mut modified = false;

        {
            let files_array = if let Some(files) =
                protobuf_hashmap.get_mut(&Yaml::from_str("files"))
            {
                if let Yaml::Array(files_array) = files {
                    files_array
                } else {
                    panic!("Error with yaml file - protobuf:files section is not an array! Make sure each item in protobuf:files section is declared with a '-' \
                to make it be treated as an array item . Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
                }
            } else {
                protobuf_hashmap.insert(Yaml::from_str("files"), Yaml::Array(Vec::new()));
                let files = protobuf_hashmap.get_mut(&Yaml::from_str("files")).unwrap();
                if let Yaml::Array(files_array) = files {
                    files_array
                } else {
                    unreachable!()
                }
            };

            let existing_files = files_array
                .iter()
                .filter_map(|file| {
                    if let Some(file_str) = file.as_str() {
                        Some(file_str.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            for protobuf_file_path in &protobuf_file_paths {
                let filename = protobuf_file_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                if !existing_files.contains(&filename) {
                    modified = true;
                    files_array.push(Yaml::from_str(filename.as_str()));
                }
            }
        }

        let import_paths_array = if let Some(import_paths) =
            protobuf_hashmap.get_mut(&Yaml::from_str("importPaths"))
        {
            if let Yaml::Array(import_paths_array) = import_paths {
                import_paths_array
            } else {
                panic!("Error with yaml file - protobuf:importPaths section is not an array! Make sure each item in protobuf:importPaths section is declared with a '-' \
                to make it be treated as an array item . Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
            }
        } else {
            protobuf_hashmap.insert(Yaml::from_str("importPaths"), Yaml::Array(Vec::new()));
            let import_paths = protobuf_hashmap
                .get_mut(&Yaml::from_str("importPaths"))
                .unwrap();
            if let Yaml::Array(import_paths_array) = import_paths {
                import_paths_array
            } else {
                unreachable!()
            }
        };

        let existing_import_paths = import_paths_array
            .iter()
            .filter_map(|import_path| {
                if let Some(import_path_str) = import_path.as_str() {
                    Some(import_path_str.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for protobuf_file_path in &protobuf_file_paths {
            let relative_path = get_relative_path(
                &substreams_yaml_dir,
                &protobuf_file_path.parent().unwrap().to_path_buf(),
            );
            if !existing_import_paths.contains(&relative_path) {
                modified = true;
                import_paths_array.push(Yaml::from_str(relative_path.as_str()));
            }
        }

        modified
    }

    /// Returns true if an edit to the substreams.yaml was made. (false if no changes made)
    pub(crate) fn add_module(&mut self, module: Module) -> bool {
        let contents_hashmap = self.get_contents_hashmap_mut();

        if let Some(modules) = contents_hashmap.get_mut(&Yaml::from_str("modules")) {
            let modules_array = if let Yaml::Array(modules_array) = modules {
                modules_array
            } else {
                panic!("Error with yaml file - module section is not an array! Make sure each item in modules section is declared with a '-' \
                to make it be treated as an array item . Filepath: {}", self.substreams_yaml_dir.join("substreams.yaml").to_string_lossy());
            };

            let existing_module_names = modules_array
                .iter()
                .filter_map(|module| {
                    if let Some(module_hashmap) = module.as_hash() {
                        if let Some(module_name_yaml) = module_hashmap.get(&Yaml::from_str("name"))
                        {
                            if let Some(module_name) = module_name_yaml.as_str() {
                                return Some(module_name.to_string());
                            }
                        }
                    }

                    None
                })
                .collect::<Vec<_>>();

            if existing_module_names.contains(module.name()) {
                // For now, if the module already exists we will just not do anything. At some point we should add a force_update flag here.
                return false;
            }

            modules_array.push(module.to_yaml());
        } else {
            contents_hashmap.insert(Yaml::from_str("modules"), Yaml::Array(vec![module.to_yaml()]));
        }

        true
    }

    fn get_contents_hashmap(&self) -> &Hash {
        if let Yaml::Hash(contents_hashmap) = &self.yaml {
            contents_hashmap
        } else {
            panic!(
                "Error getting contents hashmap for yaml file: {}",
                self.substreams_yaml_dir
                    .join("substreams.yaml")
                    .to_string_lossy()
            )
        }
    }

    fn get_contents_hashmap_mut(&mut self) -> &mut Hash {
        if let Yaml::Hash(contents_hashmap) = &mut self.yaml {
            contents_hashmap
        } else {
            panic!(
                "Error getting contents hashmap for yaml file: {}",
                self.substreams_yaml_dir
                    .join("substreams.yaml")
                    .to_string_lossy()
            )
        }
    }

    pub(crate) fn get_file_contents(self) -> String {
        let mut file_contents = String::new();
        let mut emitter = YamlEmitter::new(&mut file_contents);
        emitter.dump(&self.yaml).unwrap();

        // Making the output presentation of the file look nicer..
        if file_contents.starts_with("---\n") {
            file_contents = file_contents[4..].to_string();
        }
        file_contents = file_contents.replace("\nimports:\n", "\n\nimports:\n");
        file_contents = file_contents.replace("\nprotobuf:\n", "\n\nprotobuf:\n");
        file_contents = file_contents.replace("\nbinaries:\n", "\n\nbinaries:\n");
        file_contents = file_contents.replace("\nmodules:\n", "\n\nmodules:\n");

        file_contents
    }
}

impl From<&str> for SubstreamsYaml {
    fn from(yaml_contents: &str) -> Self {
        let yaml = YamlLoader::load_from_str(yaml_contents).expect(&format!(
            "Unable to read substreams_yaml contents! File contents: {}",
            yaml_contents
        ))[0]
            .clone();

        SubstreamsYaml {
            substreams_yaml_dir: PathBuf::new(),
            yaml,
        }
    }
}

pub(crate) enum Module {
    Map {
        name: String,
        initial_block: Option<u64>,
        inputs: Vec<Input>,
        output_type: String,
    },
    Store {
        name: String,
        initial_block: Option<u64>,
        update_policy: UpdatePolicy,
        value_type: String,
        inputs: Vec<Input>,
    },
}

impl Module {
    pub(crate) fn map(
        name: String,
        initial_block: Option<u64>,
        inputs: Vec<Input>,
        output_type: String,
    ) -> Self {
        Module::Map {
            name,
            initial_block,
            inputs,
            output_type,
        }
    }

    pub(crate) fn store(
        name: String,
        initial_block: Option<u64>,
        update_policy: UpdatePolicy,
        value_type: String,
        inputs: Vec<Input>,
    ) -> Self {
        Module::Store {
            name,
            initial_block,
            update_policy,
            value_type,
            inputs,
        }
    }

    pub(crate) fn name(&self) -> &String {
        match self {
            Module::Map { name, .. } => name,
            Module::Store { name, .. } => name,
        }
    }

    pub(crate) fn to_yaml(self) -> Yaml {
        let yaml_str = match self {
            Module::Map {
                name,
                initial_block,
                inputs,
                output_type,
            } => {
                let mut yaml_str = format!(
                    "name: {}\n\
                    kind: map\n",
                    name
                );

                if let Some(initial_block) = initial_block {
                    yaml_str.push_str(&format!("initialBlock: {}\n", initial_block));
                }

                yaml_str.push_str(&format!(
                    "inputs:\n    \
                        - {}\n\
                    output:\n    \
                        type: {}\n",
                    inputs
                        .iter()
                        .map(|input| input.to_string())
                        .collect::<Vec<_>>()
                        .join("\n  - "),
                    output_type
                ));

                yaml_str
            }
            Module::Store {
                name,
                initial_block,
                update_policy,
                value_type,
                inputs,
            } => {
                let mut yaml_str = format!(
                    "name: {}\n\
                    kind: store\n",
                    name
                );

                if let Some(initial_block) = initial_block {
                    yaml_str.push_str(&format!("initialBlock: {}\n", initial_block));
                }

                yaml_str.push_str(&format!(
                    "updatePolicy: {}\n\
                    valueType: {}\n\
                    inputs:\n    \
                        - {}",
                    update_policy,
                    value_type,
                    inputs
                        .iter()
                        .map(|input| input.to_string())
                        .collect::<Vec<_>>()
                        .join("\n  - "),
                ));

                yaml_str
            }
        };

        YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0].clone()
    }
}

pub(crate) struct Input {
    pub(crate) input_type: InputType,
    pub(crate) input_value: String,
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.input_type, self.input_value)
    }
}

pub(crate) enum InputType {
    Source,
    Store,
    Map,
}

impl Display for InputType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Source => write!(f, "source"),
            InputType::Store => write!(f, "store"),
            InputType::Map => write!(f, "map"),
        }
    }
}

pub(crate) enum UpdatePolicy {
    Set,
    Add,
}

impl Display for UpdatePolicy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdatePolicy::Set => write!(f, "set"),
            UpdatePolicy::Add => write!(f, "add"),
        }
    }
}
