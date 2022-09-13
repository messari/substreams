use crate::errors::Error;

const DEFAULT_START_BLOCK: u64 = 13e6 as u64;
const DEFAULT_NUM_BLOCKS: usize = 10;

pub struct Module {
    pub method: String,
    pub path: String,
    pub cache_file: String,
    pub start_block: Option<u64>,
    pub num_blocks: Option<usize>,
}

impl Module {
    pub fn new(method: &str, path: &str, cache_file: &str) -> Self {
        Self {
            method: method.to_string(),
            path: path.to_string(),
            cache_file: cache_file.to_string(),
            start_block: None,
            num_blocks: None,
        }
    }

    pub fn cached_output(&self) -> Result<Option<String>, Error> {
        let cache = std::fs::read_to_string(&self.cache_file).ok();
        Ok(cache)
    }

    pub fn check_cached_output(&self, output: String) -> Result<(), Error> {
        let cache = self.cached_output()?;

        if output.is_empty() {
            return Err(Error::EmptyOutput);
        } else if cache.is_none() || cache.clone().unwrap().is_empty() {
            self.cache_output(output)?;
        } else if cache.unwrap() != output {
            return Err(Error::OutputMismatch);
        }

        Ok(())
    }

    pub fn cache_output(&self, output: String) -> Result<(), Error> {
        if output.is_empty() {
            return Err(Error::EmptyOutput);
        }

        std::fs::write(&self.cache_file, output)?;
        Ok(())
    }

    // Runs substream module using the `substreams` cli.
    // NOTE: `substreams` cli must be installed to run this command.
    pub fn run(&self) -> Result<String, Error> {
        use std::process::Command;

        // generate pb files under src/pb
        let output = Command::new("substreams")
            .args(&[
                "run",
                "-e",
                "api-dev.streamingfast.io:443",
                &self.path,
                &self.method,
                "-s",
                self.start_block
                    .unwrap_or(DEFAULT_START_BLOCK)
                    .to_string()
                    .as_str(),
                "-t",
                format!("+{}", self.num_blocks.unwrap_or(DEFAULT_NUM_BLOCKS)).as_str(),
            ])
            .output()?;

        if !output.stderr.is_empty() {
            return Err(Error::Substream(String::from_utf8(output.stderr).unwrap()));
        }

        let content = String::from_utf8_lossy(&output.stdout);

        Ok(content.to_string())
    }

    pub fn test(&self) -> Result<(), Error> {
        self.check_cached_output(self.run()?)
    }
}
