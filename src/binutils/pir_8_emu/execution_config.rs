/// A configuration set, specifying various execution tunings
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExecutionConfig {
    /// Automatically load the next instruction, silently performing the
    /// [`NEXT_INSTRUCTION`](../../micro/static.NEXT_INSTRUCTION.html) microops
    pub auto_load_next_instruction: bool,
}

impl ExecutionConfig {
    pub fn new() -> ExecutionConfig {
        ExecutionConfig { auto_load_next_instruction: false }
    }
}

impl Default for ExecutionConfig {
    #[inline]
    fn default() -> ExecutionConfig {
        ExecutionConfig::new()
    }
}
