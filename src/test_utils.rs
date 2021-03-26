use crate::MacroOptimizationHints;

pub fn macro_optimization_hint_did_change() -> MacroOptimizationHints {
    MacroOptimizationHints { did_change: true }
}

pub fn macro_optimization_hint_unchanged() -> MacroOptimizationHints {
    MacroOptimizationHints { did_change: false }
}
