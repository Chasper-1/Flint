#[derive(Clone, Debug)]
pub enum SegmentStyle {
    Plain,
    Bold,
    Italic,
    Strikethrough,
    Superscript,
    Subscript,
    Code,
}

#[derive(Clone, Debug)]
pub struct Segment {
    pub text: String,

    pub style: SegmentStyle,

    pub left_marker_len: usize,
    pub right_marker_len: usize,

    pub raw_start: usize,
    pub raw_end: usize,

    pub visible_start: usize,
    pub visible_end: usize,
}