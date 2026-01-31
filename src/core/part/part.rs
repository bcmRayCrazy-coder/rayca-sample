use crate::core::part::{option::PartOption, param::PartParam};

pub struct Part {
    pub param: PartParam,
    pub option: PartOption,
    pub seq: [bool; 16],
    pub motion_seq: Option<[PartParam; 16]>,
}
