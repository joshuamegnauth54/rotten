#[derive(Default, Debug, Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub size: Size,
}
