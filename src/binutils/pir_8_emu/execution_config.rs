/// A configuration set, specifying various execution tunings
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExecutionConfig {
    /// Automatically load the next instruction, silently performing the
    /// [`NEXT_INSTRUCTION`](../../micro/static.NEXT_INSTRUCTION.html) microops
    pub auto_load_next_instruction: bool,
    /// Whetherto perform all of instructions' μOps at once
    pub execute_full_instructions: bool,
}

impl ExecutionConfig {
    pub fn new() -> ExecutionConfig {
        ExecutionConfig {
            auto_load_next_instruction: false,
            execute_full_instructions: false,
        }
    }
}

impl Default for ExecutionConfig {
    #[inline]
    fn default() -> ExecutionConfig {
        ExecutionConfig::new()
    }
}
