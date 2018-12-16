use common::pattern::{Pattern, CompiledPattern, PatternMatch};
use common::compilation::CompilableTo;
use common::validation::Issue;

#[derive(Clone)]
pub struct Trigger {
    pattern: Pattern,
    id: String,
}

#[derive(Clone)]
pub struct CompiledTrigger {
    pattern: CompiledPattern,
    id: String,
}

impl CompilableTo<CompiledTrigger> for Trigger {
    fn compile(&self) -> Result<CompiledTrigger, Issue> {
        match self.pattern.compile() {
            Ok(compiled_pattern) => Ok(CompiledTrigger {
                pattern: compiled_pattern,
                id: self.id.clone()
            }),
            Err(issue) => Err(issue)
        }
    }
}

impl CompiledTrigger {
    pub fn quick_check(&self, other: &String) -> bool {
        self.pattern.quick_check(other)
    }

    pub fn full_check(&self, other: &String) -> Option<PatternMatch> {
        self.pattern.full_check(other)
    }
}