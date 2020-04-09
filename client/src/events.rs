pub enum WorldEvent {
    GenerateRequest,

    /// Generate the world around this point.
    Generate(f32, f32),

    /// The world has been updated and should be re-rendered.
    Updated,
}
