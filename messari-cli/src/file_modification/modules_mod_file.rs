use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use regex::Regex;

pub(crate) struct ModulesModFile {
    // It's not advised but someone might want to break the current pattern by not prefixing their
    // modules with a module number so we have accounted for this by supporting "un-numbered modules"
    numbered_modules: Vec<NumberedModule>,
    un_numbered_modules: Vec<UnNumberedModule>,
}

impl ModulesModFile {
    pub(crate) fn new() -> Self {
        ModulesModFile {
            numbered_modules: vec![],
            un_numbered_modules: vec![],
        }
    }

    // If no module_fn_name is specified it defaults to using the same name for the fn and what was used for the module
    pub(crate) fn add_module(&mut self, module_filename: String, module_fn_name: Option<String>) {
        assert!(module_filename.ends_with(".rs"), "If adding a numbered module to the modules folder you need to make sure the filename is in the form: <number>_restOfFileName.rs!\nFilename given: {}", module_filename);

        if module_filename.chars().next().unwrap().is_ascii_digit() {
            if module_filename.contains("_") {
                let mut module_filename_iter = module_filename.split("_");
                if let Ok(module_number) = module_filename_iter.next().unwrap().parse::<u8>() {
                    let filename_no_number = module_filename_iter.collect::<Vec<_>>().join("_");
                    let mod_name =  &filename_no_number[filename_no_number.len()-4..filename_no_number.len()];
                    self.numbered_modules.push(NumberedModule {
                        filename: filename_no_number.to_string(),
                        mod_name: mod_name.to_string(),
                        fn_name: module_fn_name.unwrap_or(mod_name.to_string()),
                        module_number,
                    });

                    return;
                }
            }
        } else {
            let mod_name =  &module_filename[module_filename.len()-4..module_filename.len()];
            self.un_numbered_modules.push(UnNumberedModule {
                mod_name: mod_name.to_string(),
                fn_name: module_fn_name.unwrap_or(mod_name.to_string()),
            });

            return;
        }

        panic!("If adding a numbered module to the modules folder you need to make sure the filename is in the form: <number>_restOfFileName.rs!\nFilename given: {}", module_filename);
    }

    pub(crate) fn get_new_module_number(&self) -> u8 {
        if let Some(latest_module) = self.numbered_modules.last() {
            latest_module.module_number + 1
        } else {
            1
        }
    }

    pub(crate) fn load_from_file(modules_mod_filepath: &PathBuf) -> Self {
        let re_newline_cleaner = Regex::new(r"[ \t\n\r]*\n[ \t\n\r]*").unwrap();
        let re_path_extractor = Regex::new("#[path *= *\"(?P<filename>[^\"]+)\" *]").unwrap();

        let mut modules_mod_file_contents = fs::read_to_string(modules_mod_filepath).expect(&format!(
            "Unable to read modules mod file contents! Filepath: {}",
            modules_mod_filepath.to_string_lossy()
        ));
        modules_mod_file_contents = re_newline_cleaner
            .replace_all(&modules_mod_file_contents, "\n")
            .to_string();

        let mut new_lines_iter = modules_mod_file_contents.split("\n").into_iter();
        let mut numbered_modules = Vec::new();
        let mut un_numbered_modules = Vec::new();
        let mut using_declarations = Vec::new();
        while let Some(line) = new_lines_iter.next() {
            if line.starts_with("#[path = ") {
                let caps = re_path_extractor.captures(line).unwrap();
                let filename = caps.name("filename").unwrap().as_str();
                let module_number = filename.split("_").next().unwrap().parse::<u8>().expect(&format!("Unable to parse module number from path macro attribute!\nLine: {}", line));
                let next_line = new_lines_iter.next();
                if let Some(next_line) = next_line {
                    if next_line.starts_with("mod ") && next_line.ends_with(";") {
                        let mod_name = next_line[4..next_line.len()-1].to_string();
                        numbered_modules.push(NumberedModule {
                            filename: filename.to_string(),
                            mod_name,
                            fn_name: "".to_string(), // Will be overwritten further down
                            module_number,
                        });
                    } else {
                        panic!("Path macro attribute should always be followed by a mod line!\nLine instead: {}", next_line);
                    }
                }
            } else if line.starts_with("mod ") {
                assert!(line.ends_with(";"), "Module not declared in modules mod file correctly! Module declaration line:\n{}", line);
                un_numbered_modules.push(UnNumberedModule {
                    mod_name: line[4..line.len()-1].to_string(),
                    fn_name: "".to_string(), // Will be overwritten further down
                });
            } else if line.starts_with("pub use ") {
                assert!(line.ends_with(";"), "Visibility declarations (eg. pub use ..) not declared in modules mod file correctly! Visibility declaration line:\n{}", line);
                using_declarations.push(line[4..line.len()-1].to_string());
            } else {
                panic!("Line given is not recognised as either a module declaration or visibility declaration!\nLine: {}", line);
            }
        }

        'a: for numbered_module in numbered_modules.iter_mut() {
            for using_declaration in using_declarations.iter() {
                let module_inner = format!("{}::", numbered_module.mod_name);
                if using_declaration.starts_with(&module_inner) {
                    numbered_module.fn_name = using_declaration[module_inner.len()..using_declaration.len()].to_string();
                    continue 'a;
                }
            }
        }

        'a: for un_numbered_module in un_numbered_modules.iter_mut() {
            for using_declaration in using_declarations.iter() {
                let module_inner = format!("{}::", un_numbered_module.mod_name);
                if using_declaration.starts_with(&module_inner) {
                    un_numbered_module.fn_name = using_declaration[module_inner.len()..using_declaration.len()].to_string();
                    continue 'a;
                }
            }
        }

        // If we can always assume that all the numbered modules are ordered then it will make it super easy to add new modules
        numbered_modules.sort_by(|a, b| a.module_number.cmp(&b.module_number));

        ModulesModFile {
            numbered_modules,
            un_numbered_modules,
        }
    }

    pub(crate) fn get_file_contents(self) -> String {
        let numbered_module_mod_blocks = self.numbered_modules.iter().map(|x| x.get_mod_block()).collect::<Vec<_>>(); // Not sorted as we'd prefer to display in order by module number
        let mut un_numbered_module_mod_blocks = self.un_numbered_modules.iter().map(|x| x.get_mod_block()).collect::<Vec<_>>();
        let mut numbered_module_use_statements = self.numbered_modules.iter().map(|x| x.get_use_statement()).collect::<Vec<_>>();
        let mut un_numbered_module_use_statements = self.un_numbered_modules.iter().map(|x| x.get_use_statement()).collect::<Vec<_>>();

        un_numbered_module_mod_blocks.sort();
        numbered_module_use_statements.sort();
        un_numbered_module_use_statements.sort();

        format!("{}\n\n{}\n\n{}\n\n{}\n", numbered_module_mod_blocks.join("\n\n"), un_numbered_module_mod_blocks.join("\n"), numbered_module_use_statements.join("\n"), un_numbered_module_use_statements.join("\n"))
    }
}

struct NumberedModule {
    filename: String,
    mod_name: String,
    fn_name: String,
    module_number: u8
}

impl NumberedModule {
    fn get_mod_block(&self) -> String {
        format!("#[path = \"{}\"]\nmod {};", self.filename, self.mod_name)
    }

    fn get_use_statement(&self) -> String {
        format!("pub use {}::{};", self.mod_name, self.fn_name)
    }
}

struct UnNumberedModule {
    mod_name: String, // Filename can always be derived from the module name as module_name.rs
    fn_name: String,
}

impl UnNumberedModule {
    fn get_mod_block(&self) -> String {
        format!("mod {};", self.mod_name)
    }

    fn get_use_statement(&self) -> String {
        format!("pub use {}::{};", self.mod_name, self.fn_name)
    }
}