use std::fs;
use std::path::PathBuf;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

pub(crate) struct SubstreamsYaml {
    substreams_yaml_dir: PathBuf, // This is parent directory for the substreams.yaml file
    yaml: Yaml,
}

impl SubstreamsYaml {
    pub(crate) fn new(project_name: &str, substreams_filepath: &PathBuf) -> Self {
        let substreams_yaml_dir = substreams_filepath.parent().unwrap().to_path_buf();

        let yaml_contents = format!(
            "specVersion: v0.1.0\n\
            package:\n    \
                name: \"{0}\"\n    \
                version: v0.1.0\n\
            \n\
            binaries:\n    \
                default:\n        \
                    type: wasm/rust-v1\n        \
                    file: ../target/wasm32-unknown-unknown/release/{0}.wasm",
            project_name
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
