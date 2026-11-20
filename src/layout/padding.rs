use crate::utils::types::SpacingStep;

/// Lowers the public spacing scale to the container system's spacing tokens.
/// Kept under Layout, which owns the upstream spacing-step map.
pub(crate) const fn spacing_step_css(step: SpacingStep) -> &'static str {
    match step {
        SpacingStep::Zero => "var(--spacing-0)",
        SpacingStep::Half => "var(--spacing-0-5)",
        SpacingStep::One => "var(--spacing-1)",
        SpacingStep::OneAndHalf => "var(--spacing-1-5)",
        SpacingStep::Two => "var(--spacing-2)",
        SpacingStep::Three => "var(--spacing-3)",
        SpacingStep::Four => "var(--spacing-4)",
        SpacingStep::Five => "var(--spacing-5)",
        SpacingStep::Six => "var(--spacing-6)",
        SpacingStep::Eight => "var(--spacing-8)",
        SpacingStep::Ten => "var(--spacing-10)",
    }
}
