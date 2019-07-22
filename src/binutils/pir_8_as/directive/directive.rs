use self::super::super::super::super::isa::instruction::ParseInstructionError;
use self::super::AssemblerDirectiveObeyError;
use self::super::super::LabelLoad;
use std::collections::BTreeMap;


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssemblerDirective<'s> {
    SetOrigin(u16),

    SaveLabel(&'s str),
    LoadLabel(&'s str),
}

impl<'s> AssemblerDirective<'s> {
    #[inline]
    pub fn from_str(s: &'s str) -> Result<Option<Self>, ParseInstructionError> {
        AssemblerDirective::from_str_impl(s)
    }

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
