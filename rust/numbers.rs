
fn clamp(v: f32, lb: f32, ub: f32) -> f32 {
    f32::min(f32::max(v, lb), ub)
}
