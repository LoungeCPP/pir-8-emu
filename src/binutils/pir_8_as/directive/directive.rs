use self::super::super::super::super::isa::instruction::ParseInstructionError;
use self::super::AssemblerDirectiveObeyError;
use self::super::super::LabelLoad;
use std::collections::BTreeMap;


/// An assembler directive, extending the normal assembly syntax
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssemblerDirective<'s> {
    /// Start writing the code at the specified address, 0-padding thereupto
    ///
    /// Using this directive the second time or after processing an instruction is an error
    ///
    /// Syntax: `:origin [address]`
    SetOrigin(u16),

    /// Save the current output address to recall later
    ///
    /// Syntax: `:label save [name]`
    SaveLabel(&'s str),

    /// Load the label with the specified name, or wait for it to be saved later
    ///
    /// Not having saved all previously loaded labels by the end of input is an error
    ///
    /// Attempting to load a label when the current instruction doesn't expect 2-byte data is an error
    ///
    /// Syntax: `:label load [name]`
    LoadLabel(&'s str),
}

impl<'s> AssemblerDirective<'s> {
    /// Parse a directive as found among other assembly
    ///
    /// If the specified string doesn't start with a colon, `Ok(None)` is returned
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::binutils::pir_8_as::AssemblerDirective;
    /// assert_eq!(AssemblerDirective::from_str(":origin 0x0110"),
    ///            Ok(Some(AssemblerDirective::SetOrigin(0x0110))));
    ///
    /// assert_eq!(AssemblerDirective::from_str(": label load OwO"),
    ///            Ok(Some(AssemblerDirective::LoadLabel("OwO"))));
    ///
    /// assert_eq!(AssemblerDirective::from_str("label save uwu"),
    ///            Ok(None));
    /// ```
    #[inline]
    pub fn from_str(s: &'s str) -> Result<Option<Self>, ParseInstructionError> {
        AssemblerDirective::from_str_impl(s)
    }

    /// Obey this directive, updating the output address and labelset as required
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::binutils::pir_8_as::{AssemblerDirective, LabelLoad};
    /// # use std::collections::BTreeMap;
    /// let mut next_output_address = None;
    /// let mut labels = BTreeMap::new();
    ///
    /// assert_eq!(AssemblerDirective::SetOrigin(0x0110)
    ///                .obey(&mut next_output_address, &mut labels),
    ///            Ok(None));
    ///
    /// assert_eq!(AssemblerDirective::LoadLabel("owo")
    ///                .obey(&mut next_output_address, &mut labels),
    ///            Ok(Some(LabelLoad::WaitFor("owo".to_string()))));
    /// assert_eq!(AssemblerDirective::SaveLabel("owo")
    ///                .obey(&mut next_output_address, &mut labels),
    ///            Ok(None));
    /// assert_eq!(AssemblerDirective::LoadLabel("owo")
    ///                .obey(&mut next_output_address, &mut labels),
    ///            Ok(Some(LabelLoad::HaveImmediately(0x0110))));
    ///
    /// assert_eq!(next_output_address, Some(0x0110));
    /// assert_eq!(labels, vec![("owo".to_string(), 0x0110)].into_iter().collect());
    /// ```
    pub fn obey(&self, next_output_address: &mut Option<u16>, labels: &mut BTreeMap<String, u16>)
                -> Result<Option<LabelLoad>, AssemblerDirectiveObeyError<'s>> {
        match &self {
            AssemblerDirective::SetOrigin(origin) => {
                if let Some(&nao) = next_output_address.as_ref() {
                    Err(AssemblerDirectiveObeyError::OutputAddressAlreadySet(nao, *origin))
                } else {
                    *next_output_address = Some(*origin);
                    Ok(None)
                }
            }
            AssemblerDirective::SaveLabel(lbl) => {
                if !labels.contains_key(*lbl) {
                    labels.insert(lbl.to_string(),
                                  if let Some(&oa) = next_output_address.as_ref() {
                                      oa
                                  } else {
                                      *next_output_address = Some(0);
                                      0
                                  });

                    Ok(None)
                } else {
                    Err(AssemblerDirectiveObeyError::LabelNameTaken(lbl))
                }
            }
            AssemblerDirective::LoadLabel(lbl) => {
                match labels.get(*lbl) {
                    None => Ok(Some(LabelLoad::WaitFor(lbl.to_string()))),
                    Some(&addr) => Ok(Some(LabelLoad::HaveImmediately(addr))),
                }
            }
        }
    }
}
