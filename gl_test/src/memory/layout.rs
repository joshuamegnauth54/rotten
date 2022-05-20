#[derive(Debug, Clone, Copy)]
pub struct Layout {
    /// Location index for shader
    pub index: usize,
    /// Number of items in this component
    /// A component, [f32; 3], has a size of 3
    pub size: usize,
    /// Stride: total size of each component that represents an integral part of an object but
    /// not of the full object. A vertex (ex. three floats) and an associated color (three floats)
    /// would be two components and a single stride. A full object may have many of these pairs.
    ///
    /// ```rust
    /// let triangle = [-0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
    ///                  0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
    ///                  0.0,  0.5, 0.0, 0.0, 0.0, 1.0];
    /// ```
    ///
    /// The first three f32 are vertices passed to location 0. The second set of three f32 are
    /// colors. Therefore, the first three f32 are one component while the second set are the
    /// second component.
    ///
    /// There are three sets of two components. The entire object consists of 18 f32. The stride is
    /// the size of the two components. In other words, the stride represents the distance between
    /// a certain type of component and the next of the same type of component. That size or
    /// distance is equivalent to:
    ///
    /// ```rust
    /// let stride = 6 * std::mem::size_of::<f32>();
    /// ```
    pub stride: usize,
    /// Start location for current component as an offset in bytes
    pub start: usize,
}
