mod output_with_queue;
mod directive;

pub use self::directive::{AssemblerDirectiveObeyError, AssemblerDirective};
pub use self::output_with_queue::OutputWithQueue;


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LabelLoad {
    HaveImmediately(u16),
    WaitFor(String),
}
