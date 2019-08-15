use serde::de::{Deserializer, Deserialize, MapAccess as DeserialiserMapAccess, Visitor as DeserialisationVisitor};
use self::super::super::super::isa::{GeneralPurposeRegisterBank, GeneralPurposeRegister};
use toml::de::{Error as TomlError, from_str as from_toml_str};
use toml::to_string_pretty as toml_to_string;
use std::io::Error as IoError;
use std::path::PathBuf;
use std::mem::size_of;
use serde::Serialize;
use std::{fmt, fs};


const GP_REGISTER_COUNT: usize = size_of::<GeneralPurposeRegisterBank>() / size_of::<GeneralPurposeRegister>();


/// A configuration set, specifying various execution tunings
#[derive(Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExecutionConfig {
    /// Automatically load the next instruction, silently performing the
    /// [`NEXT_INSTRUCTION`](../../micro/static.NEXT_INSTRUCTION.html) microops
    pub auto_load_next_instruction: bool,
    /// Whether to perform all of instructions' μOps at once
    pub execute_full_instructions: bool,
    /// The register letters
    ///
    /// Validated and optionally truncated/reset at load
    pub general_purpose_register_letters: [char; GP_REGISTER_COUNT],
}

impl ExecutionConfig {
    /// Create a new default-initialised config
    pub fn new() -> ExecutionConfig {
        ExecutionConfig {
            auto_load_next_instruction: false,
            execute_full_instructions: false,
            general_purpose_register_letters: default_letters(),
        }
    }

    /// Read an execution config from file named `"exec_cfg.toml"` under the specified config directory
    ///
    /// Returns `Err(Ok(ioe))` if reading the file failed with `ioe`.<br />
    /// Returns `Err(Err(te))` if parsing the read file failed with `te`.<br />
    /// Returns `Ok(None)` if the file didn't exist.<br />
    /// Returns `Ok(Some(ec))`, if the file was correctly read and parsed as TOML,
    /// the contents of `ec` start off as `ExecutionConfig::new()`, and are then updated with each valid key,
    /// this behaviour is consistent with the `Deserialize` implementation.
    ///
    /// # Examples
    ///
    /// Given `"$ROOT/exec_cfg.toml"` containing:
    ///
    /// ```toml
    /// execute_full_instructions = true
    /// hewwo = "uWu"
    /// general_purpose_register_letters = ['H', 'e', 'w', 'w', 'o', 'U', 'w', 'U']
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
    /// # hewwo = "uWu"
    /// # general_purpose_register_letters = ['H', 'e', 'w', 'w', 'o', 'U', 'w', 'U']"#.as_bytes()).unwrap();
    /// # /*
    /// let root = Path::new("$ROOT");
    /// # */
    /// assert_eq!(ExecutionConfig::read_from_config_dir(root).unwrap(),
    ///            Some(ExecutionConfig {
    ///                execute_full_instructions: true,
    ///                general_purpose_register_letters: ['H', 'e', 'w', 'w', 'o', 'U', 'w', 'U'],
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
        from_toml_str(&data).map(Some).map_err(Err)
    }

    /// Write this execution config to the file named `"exec_cfg.toml"` under the specified config directory
    ///
    /// The specified config directory and all its ascendants will be created
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::binutils::pir_8_emu::ExecutionConfig;
    /// # use std::env::temp_dir;
    /// # use std::path::Path;
    /// # use std::fs;
    /// # let root = temp_dir().join("pir_8_emu-doctest").join("binutils-pir_8_emu-ExecutionConfig-write_to_config_dir-0");
    /// # /*
    /// let root = Path::new("$ROOT");
    /// # */
    /// ExecutionConfig::new().write_to_config_dir(&root).unwrap();
    ///
    /// assert_eq!(fs::read_to_string(root.join("exec_cfg.toml")).unwrap(),
    ///            "auto_load_next_instruction = false\n\
    ///             execute_full_instructions = false\n\
    ///             general_purpose_register_letters = [\n\
    ///             \x20   'F',\n\
    ///             \x20   'S',\n\
    ///             \x20   'X',\n\
    ///             \x20   'Y',\n\
    ///             \x20   'A',\n\
    ///             \x20   'B',\n\
    ///             \x20   'C',\n\
    ///             \x20   'D',\n\
    ///             ]\n");
    /// ```
    pub fn write_to_config_dir<P: Into<PathBuf>>(&self, cfg_dir: P) -> Result<(), IoError> {
        self.write_to_config_dir_impl(cfg_dir.into())
    }

    fn write_to_config_dir_impl(&self, mut cfg: PathBuf) -> Result<(), IoError> {
        fs::create_dir_all(&cfg)?;

        cfg.push("exec_cfg.toml");

        let data = toml_to_string(&self).expect("ExecutionConfig is TOML-serialisable");
        fs::write(cfg, data.as_bytes())
    }
}

impl<'de> Deserialize<'de> for ExecutionConfig {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_struct("ExecutionConfig",
                                        &["auto_load_next_instruction", "execute_full_instructions", "general_purpose_register_letters"],
                                        ExecutionConfigVisitor)
    }
}

impl Default for ExecutionConfig {
    #[inline]
    fn default() -> ExecutionConfig {
        ExecutionConfig::new()
    }
}


struct ExecutionConfigVisitor;

impl<'de> DeserialisationVisitor<'de> for ExecutionConfigVisitor {
    type Value = ExecutionConfig;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct ExecutionConfig")
    }

    fn visit_map<V: DeserialiserMapAccess<'de>>(self, mut map: V) -> Result<ExecutionConfig, V::Error> {
        let mut ret = ExecutionConfig::new();

        while let Some(key) = map.next_key()? {
            match key {
                "auto_load_next_instruction" => ret.auto_load_next_instruction = map.next_value()?,
                "execute_full_instructions" => ret.execute_full_instructions = map.next_value()?,
                "general_purpose_register_letters" => {
                    let gprl: [char; GP_REGISTER_COUNT] = map.next_value()?;

                    if gprl.iter().all(char::is_ascii) {
                        ret.general_purpose_register_letters = gprl;
                    }
                }
                _ => drop(map.next_value::<()>()),
            }
        }

        Ok(ret)
    }
}


fn default_letters() -> [char; GP_REGISTER_COUNT] {
    let mut ret = ['\0'; GP_REGISTER_COUNT];

    for (i, reg) in GeneralPurposeRegister::defaults().into_iter().enumerate() {
        ret[i] = reg.letter();
    }

    ret
}
