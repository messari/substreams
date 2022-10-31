use crate::commands::init::ProjectType;
use crate::utils::{get_relative_path, get_relative_path_from_root_folder, get_repo_root_folder, StaticStrExt};
use cargo_edit::{get_compatible_dependency, get_latest_dependency, LocalManifest, Manifest};
use semver::{Comparator, Op};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use url::Url;

pub(crate) struct CargoToml {
    cargo_dir: PathBuf, // This is parent directory for the Cargo.toml
    manifest: Manifest
}

impl CargoToml {
    pub(crate) fn new(project_name: String, project_description: Option<String>, project_type: ProjectType, cargo_filepath: &PathBuf) -> Self {
        let cargo_dir = cargo_filepath.parent().unwrap().to_path_buf();

        let mut document_string = format!(
            "[package]\n\
            name = \"{}\"\n\
            version = \"0.1.0\"\n",
            project_name
        );

        if let Some(mut description) = project_description {
            description = description.replace("\"", "\\\""); // Make sure to the description is escaped
            document_string.push_str(&format!("description = \"{}\"\n", description))
        }

        document_string.push_str(&format!(
            "edition = \"2021\"\n\
            repository = \"https://github.com/messari/substreams/{}\"\n",
                                          get_relative_path_from_root_folder(&cargo_dir)));

        if project_type == ProjectType::SubstreamsProject {
            document_string.push_str(
                "\n\
                [lib]\n\
                crate-type = [\"cdylib\"]\n"
            );
        }

        let mut cargo_toml = CargoToml {
            manifest: Manifest::from_str(&document_string).unwrap(),
            cargo_dir
        };

        if project_type == ProjectType::SubstreamsProject {
            cargo_toml.add_wasm_dependencies(vec![
                "substreams-helper".dep_with_local_path("substreams-helper"),
                "substreams-ethereum".dep_from_workspace(),
                "substreams".dep_from_workspace(),
                "ethabi".dep_with_major_version(17),
                "hex-literal".into_dep(),
                "prost".dep_with_major_version(0),
            ]);
        }

        cargo_toml
    }

    pub(crate) fn load_from_file(cargo_filepath: &PathBuf) -> Self {
        let cargo_dir = cargo_filepath.parent().unwrap().to_path_buf();
        CargoToml {
            manifest: LocalManifest::try_new(cargo_filepath)
                .expect(&format!(
                    "Unable to read Cargo.toml! Filepath: {}",
                    cargo_filepath.to_string_lossy()
                ))
                .manifest,
            cargo_dir
        }
    }

    /// Returns true if an edit to the cargo.toml was made. (false if no changes made)
    pub(crate) fn add_project_to_workspace(&mut self, project_dir: &PathBuf) -> bool {
        let member = get_relative_path_from_root_folder(project_dir);
        let workspace = self.manifest.data.get_mut("workspace").unwrap();
        let members = workspace.get_mut("members").unwrap();
        let members_array = members.as_array_mut().unwrap();

        let members = members_array
            .iter()
            .map(|member| member.as_str().unwrap())
            .collect::<Vec<_>>();
        if members.contains(&member.as_str()) {
            return false;
        }

        members_array.push(member);

        // Here we make sure that the new member has the same style as the other members
        let members_decor = {
            members_array
                .get(members_array.len() - 2)
                .unwrap()
                .decor()
                .clone()
        };
        let new_member = members_array.get_mut(members_array.len() - 1).unwrap();
        *new_member.decor_mut() = members_decor;

        true
    }

    /// Returns true if an edit to the cargo.toml was made. (false if no changes made)
    pub(crate) fn add_build_dependencies(&mut self, build_dependencies: Vec<Dependency>) -> bool {
        if build_dependencies.is_empty() {
            return false;
        }

        self.add_dependencies(build_dependencies, "build-dependencies")
    }

    /// Returns true if an edit to the cargo.toml was made. (false if no changes made)
    pub(crate) fn add_wasm_dependencies(&mut self, wasm_dependencies: Vec<Dependency>) -> bool {
        if wasm_dependencies.is_empty() {
            return false;
        }

        self.add_dependencies(wasm_dependencies, "target.wasm32-unknown-unknown.dependencies")
    }

    pub(crate) fn get_file_contents(self) -> String {
        self.manifest.to_string()
    }

    fn add_dependencies(&mut self, dependencies: Vec<Dependency>, section: &str) -> bool {
        let section_parts = section.split(".").into_iter().collect::<Vec<_>>();
        self.ensure_section_created(section, &section_parts);
        let mut section_parts_iter = section_parts.into_iter();
        let mut item = self.manifest.data.get_mut(section_parts_iter.next().unwrap()).unwrap();
        for section_part in section_parts_iter {
            item = item.get_mut(section_part).unwrap();
        }
        let dependency_table = item.as_table_mut().unwrap();
        let current_dependencies = dependency_table.iter().map(|(dependency, _)| dependency.to_string()).collect::<Vec<_>>();
        let mut dependencies_added = false;
        for dependency in dependencies.into_iter() {
            if current_dependencies.contains(&dependency.crate_name.to_string()) {
                continue;
            }
            match dependency.location {
                Location::Local { local_path } => {
                    let relative_path = get_relative_path(&self.cargo_dir, &get_repo_root_folder().join(local_path));
                    let workspace_dependency = Manifest::from_str(&format!("{} = {{ path = \"{}\" }}", dependency.crate_name, relative_path)).unwrap().data.get(dependency.crate_name).unwrap().clone();
                    dependency_table.insert(dependency.crate_name, workspace_dependency);
                }
                Location::Remote { major_version_requirement } => {
                    let dependency_item = get_dependency(dependency.crate_name, major_version_requirement).to_toml(&self.cargo_dir);
                    dependency_table.insert(dependency.crate_name, dependency_item);
                }
                Location::Workspace => {
                    let workspace_dependency = Manifest::from_str(&format!("{} = {{ workspace = true }}", dependency.crate_name)).unwrap().data.get(dependency.crate_name).unwrap().clone();
                    dependency_table.insert(dependency.crate_name, workspace_dependency);
                }
            }
            dependencies_added = true;
        }
        dependencies_added
    }

    fn ensure_section_created(&mut self, section: &str, section_parts: &Vec<&str>) {
        let mut section_parts_iter = section_parts.iter();
        let mut item = if let Some(section_part) = section_parts_iter.next() {
            if let Some(item) = self.manifest.data.get_mut(section_part) {
                item
            } else {
                self.create_section(section, section_parts.first().unwrap());
                return;
            }
        } else {
            self.create_section(section, section_parts.first().unwrap());
            return;
        };

        for section_part in section_parts_iter {
            item = if let Some(item_unwrapped) = item.get_mut(section_part) {
                item_unwrapped
            } else {
                self.create_section(section, section_parts.first().unwrap());
                return;
            };
        }
    }

    fn create_section(&mut self, section: &str, first_section_part: &str) {
        let manifest_section = Manifest::from_str(&format!("\n[{}]\n", section)).unwrap().data.get(first_section_part).unwrap().clone();
        self.manifest.data.insert(first_section_part, manifest_section);
    }
}

fn get_dependency(crate_name: &str, major_version_requirement: Option<u64>) -> cargo_edit::Dependency {
    let registry_url = Url::parse("https://github.com/rust-lang/crates.io-index").unwrap();
    if let Some(major_version_requirement) = major_version_requirement {
        get_compatible_dependency(crate_name,
                                  &semver::VersionReq{ comparators: vec![get_version_requirement(major_version_requirement)] },
                                  &Path::new("PathNeverUsed"),
                                  Some(&registry_url))
            .expect(&format!("Unable to fetch latest dependency for crate: {}, with major version: {}, from crates.io!", crate_name, major_version_requirement))
    } else {
        get_latest_dependency(
            crate_name,
            false,
            &Path::new("PathNeverUsed"),
            Some(&registry_url),
        )
            .expect(&format!(
                "Unable to fetch latest dependency for crate: {}, from crates.io!",
                crate_name
            ))
    }
}

fn get_version_requirement(major_version_requirement: u64) -> Comparator {
    Comparator {
        op: Op::Exact,
        major: major_version_requirement,
        minor: None,
        patch: None,
        pre: Default::default(),
    }
}

pub(crate) struct Dependency {
    pub(crate) crate_name: &'static str,
    pub(crate) location: Location,
}

#[derive(PartialEq)]
pub(crate) enum Location {
    Local {
        local_path: PathBuf // This path needs to be specified relative to the repo root
    },
    Remote {
        major_version_requirement: Option<u64>
    },
    Workspace
}
