//! `pir-8-as`'s output and directive handling


mod output_with_queue;
mod directive;

pub use self::directive::{AssemblerDirectiveObeyError, AssemblerDirective};
pub use self::output_with_queue::OutputWithQueue;


/// How to handle a label load directive
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LabelLoad {
    /// The label is present with the specified address
    HaveImmediately(u16),
    /// The label isn't present and needs to be waited for under the specified name
    WaitFor(String),
}
