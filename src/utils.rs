use ratatui::layout::Constraint;

/// method will be deprecated, so copied the implementation
pub fn constraint_apply(constraint: Constraint, length: u16) -> u16 {
    match constraint {
        Constraint::Percentage(p) => {
            let p = f32::from(p) / 100.0;
            let length = f32::from(length);
            (p * length).min(length) as u16
        }
        Constraint::Ratio(numerator, denominator) => {
            // avoid division by zero by using 1 when denominator is 0
            // this results in 0/0 -> 0 and x/0 -> x for x != 0
            let percentage = numerator as f32 / denominator.max(1) as f32;
            let length = f32::from(length);
            (percentage * length).min(length) as u16
        }
        Constraint::Length(l) | Constraint::Fill(l) => length.min(l),
        Constraint::Max(m) => length.min(m),
        Constraint::Min(m) => length.max(m),
    }
}
