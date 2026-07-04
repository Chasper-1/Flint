use super::segment::Segment;

#[derive(Default, Clone, Debug)]
pub struct MarkupCache {
    pub segments: Vec<Segment>,
}