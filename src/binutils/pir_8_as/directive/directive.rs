use self::super::super::super::super::isa::instruction::ParseInstructionError;
use self::super::super::{LabelFragment, LabelLoad};
use self::super::AssemblerDirectiveObeyError;
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

    /// Load the specified part of the label with the specified name, or wait for it to be saved later, adding the specified
    /// offset
    ///
    /// Not having saved all previously loaded labels by the end of input is an error
    ///
    /// Attempting to load a full label when the current instruction doesn't expect 2-byte data is an error, and
    /// attempting to load a partial label when the current instruction doesn't expect 1-byte data is an error.
    ///
    /// Syntax: `:label load [full|high|low] [name]`
    ///
    /// Syntax: `:label load-offset [full|high|low] [name] [offset]`
    LoadLabel(&'s str, i16, LabelFragment),

    /// Blindly write the specified literal
    ///
    /// Attempting to insert a literal when the current instruction is expecting data is an error
    ///
    /// Syntax: `:literal "UwU"`
    InsertLiteral(&'s str),
}

impl<'s> AssemblerDirective<'s> {
    /// Parse a directive as found among other assembly
    ///
    /// If the specified string doesn't start with a colon, `Ok(None)` is returned
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::binutils::pir_8_as::{AssemblerDirective, LabelFragment};
    /// assert_eq!(AssemblerDirective::from_str(":origin 0x0110"),
    ///            Ok(Some(AssemblerDirective::SetOrigin(0x0110))));
    ///
    /// assert_eq!(AssemblerDirective::from_str(": label load full OwO"),
    ///            Ok(Some(AssemblerDirective::LoadLabel("OwO", 0, LabelFragment::Full))));
    ///
    /// assert_eq!(AssemblerDirective::from_str(": label load-offset high OwO -1"),
    ///            Ok(Some(AssemblerDirective::LoadLabel("OwO", -1, LabelFragment::High))));
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
    /// # use pir_8_emu::binutils::pir_8_as::{AssemblerDirective, LabelFragment, LabelLoad};
    /// # use std::collections::BTreeMap;
    /// let mut next_output_address = None;
    /// let mut labels = BTreeMap::new();
    ///
    /// assert_eq!(AssemblerDirective::SetOrigin(0x0110)
    ///                .obey(&mut next_output_address, &mut labels),
    ///            Ok(None));
    ///
    /// assert_eq!(AssemblerDirective::LoadLabel("owo", 0, LabelFragment::Full)
    ///                .obey(&mut next_output_address, &mut labels),
    ///            Ok(Some(Ok(LabelLoad::WaitFor("owo".to_string(), 0, LabelFragment::Full)))));
    /// assert_eq!(AssemblerDirective::SaveLabel("owo")
    ///                .obey(&mut next_output_address, &mut labels),
    ///            Ok(None));
    /// assert_eq!(AssemblerDirective::LoadLabel("owo", 0, LabelFragment::High)
    ///                .obey(&mut next_output_address, &mut labels),
    ///            Ok(Some(Ok(LabelLoad::HaveImmediately(0x0110, LabelFragment::High)))));
    /// assert_eq!(AssemblerDirective::LoadLabel("owo", 0x0F, LabelFragment::Low)
    ///                .obey(&mut next_output_address, &mut labels),
    ///            Ok(Some(Ok(LabelLoad::HaveImmediately(0x011F, LabelFragment::Low)))));
    ///
    /// assert_eq!(AssemblerDirective::InsertLiteral("EwE")
    ///                .obey(&mut next_output_address, &mut labels),
    ///            Ok(Some(Err("EwE"))));
    ///
    /// assert_eq!(next_output_address, Some(0x0110));
    /// assert_eq!(labels, vec![("owo".to_string(), 0x0110)].into_iter().collect());
    /// ```
    pub fn obey(&self, next_output_address: &mut Option<u16>, labels: &mut BTreeMap<String, u16>)
                -> Result<Option<Result<LabelLoad, &'s str>>, AssemblerDirectiveObeyError<'s>> {
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
            AssemblerDirective::LoadLabel(lbl, offset, fragment) => {
                match labels.get(*lbl) {
                    None => Ok(Some(Ok(LabelLoad::WaitFor(lbl.to_string(), *offset, *fragment)))),
                    Some(&addr) => {
                        Ok(Some(Ok(LabelLoad::HaveImmediately(if *offset < 0 {
                                                                  addr.wrapping_sub(-*offset as u16)
                                                              } else {
                                                                  addr.wrapping_add(*offset as u16)
                                                              },
                                                              *fragment))))
                    }
                }
            }
            AssemblerDirective::InsertLiteral(lit) => Ok(Some(Err(lit))),
        }
    }
}
