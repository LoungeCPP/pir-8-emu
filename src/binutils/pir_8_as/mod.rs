//! `pir-8-as`'s output and directive handling


mod output_with_queue;
mod directive;

pub use self::directive::{AssemblerDirectiveObeyError, AssemblerDirective};
pub use self::output_with_queue::OutputWithQueue;


/// How to handle a label load directive
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LabelLoad {
    /// The specified fragment of the label is present with the specified address
    HaveImmediately(u16, LabelFragment),
    /// The label isn't present and needs to be waited for under the specified name, adding the specified offset afterward,
    /// writing only the specified fragment
    WaitFor(String, i16, LabelFragment),
}

/// Which part of the label to write
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LabelFragment {
    /// Write both bytes of the label address
    Full,
    /// Write only the high byte of the label address
    High,
    /// Write only the low byte of the label address
    Low,
}

impl LabelFragment {
    /// Get the amount of bytes this label fragment takes
    ///
    /// # Examples
    ///
    /// ```
    /// # use pir_8_emu::binutils::pir_8_as::LabelFragment;
    /// assert_eq!(LabelFragment::Full.len(), 2);
    /// assert_eq!(LabelFragment::High.len(), 1);
    /// ```
    pub fn len(self) -> u8 {
        match self {
            LabelFragment::Full => 2,
            LabelFragment::High | LabelFragment::Low => 1,
        }
    }
}
