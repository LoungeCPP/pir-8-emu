use toml::de::{Error as TomlError, from_str as from_toml_str};
use serde::{Deserialize, Serialize};
use std::io::Error as IoError;
use toml::Value as TomlValue;
use std::path::PathBuf;
use std::fs;


/// A configuration set, specifying various execution tunings
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExecutionConfig {
    /// Automatically load the next instruction, silently performing the
    /// [`NEXT_INSTRUCTION`](../../micro/static.NEXT_INSTRUCTION.html) microops
    pub auto_load_next_instruction: bool,
    /// Whetherto perform all of instructions' μOps at once
    pub execute_full_instructions: bool,
}

impl ExecutionConfig {
    /// Create a new default-initialised config
    pub fn new() -> ExecutionConfig {
        ExecutionConfig {
            auto_load_next_instruction: false,
            execute_full_instructions: false,
        }
    }

    /// Read an execution config from file named `"exec_cfg.toml"` under the specified config directory
    ///
    /// Returns `Err(Ok(ioe))` if reading the file failed with `ioe`.<br />
    /// Returns `Err(Err(te))` if parsing the read file failed with `te`.<br />
    /// Returns `Ok(None)` if the file didn't exist.<br />
    /// Returns `Ok(Some(ec))`, if the file was correctly read and parsed as TOML,
    /// the contents of `ec` start off as `ExecutionConfig::new()`, and are then updated with each valid key.
    ///
    /// # Examples
    ///
    /// Given `"$ROOT/exec_cfg.toml"` containing:
    ///
    /// ```toml
    /// execute_full_instructions = true
    /// hewwo = "uWu"
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use pir_8_emu::binutils::pir_8_emu::ExecutionConfig;
    /// # use std::env::temp_dir;
    /// # use std::path::Path;
    /// # use std::fs;
    /// # let mut root = temp_dir();
    /// # let _ = fs::create_dir(&root);
    /// # root.push("pir_8_emu-doctest");
    /// # let _ = fs::create_dir(&root);
    /// # root.push("binutils-pir_8_emu-ExecutionConfig-read_from_config_dir-0");
    /// # let _ = fs::create_dir(&root);
    /// # fs::write(root.join("exec_cfg.toml"), r#"
    /// # execute_full_instructions = true
    /// # hewwo = "uWu""#.as_bytes()).unwrap();
    /// # /*
    /// let root = Path::new("$ROOT");
    /// # */
    /// assert_eq!(ExecutionConfig::read_from_config_dir(root).unwrap(),
    ///            Some(ExecutionConfig {
    ///                execute_full_instructions: true,
    ///                ..ExecutionConfig::new()
    ///            }));
    /// ```
    pub fn read_from_config_dir<P: Into<PathBuf>>(cfg_dir: P) -> Result<Option<ExecutionConfig>, Result<IoError, TomlError>> {
        ExecutionConfig::read_from_config_dir_impl(cfg_dir.into())
    }

    fn read_from_config_dir_impl(mut cfg: PathBuf) -> Result<Option<ExecutionConfig>, Result<IoError, TomlError>> {
        cfg.push("exec_cfg.toml");
        if !cfg.exists() {
            return Ok(None);
        }

        let data = fs::read_to_string(cfg).map_err(Ok)?;
        let val: TomlValue = from_toml_str(&data).map_err(Err)?;

        let mut ret = ExecutionConfig::new();

        if let Some(auto_load_next_instruction) = val.get("auto_load_next_instruction").and_then(|v| v.as_bool()) {
            ret.auto_load_next_instruction = auto_load_next_instruction;
        }

        if let Some(execute_full_instructions) = val.get("execute_full_instructions").and_then(|v| v.as_bool()) {
            ret.execute_full_instructions = execute_full_instructions;
        }

        Ok(Some(ret))
    }
}

impl Default for ExecutionConfig {
    #[inline]
    fn default() -> ExecutionConfig {
        ExecutionConfig::new()
    }
}