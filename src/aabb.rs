use ggez::mint::Point2;

#[derive(Debug)]
pub(crate) struct AABB {
    pub(crate)x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

