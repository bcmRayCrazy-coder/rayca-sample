use crate::core::part::{option::PartOption, param::PartParam};

pub struct Part {
    pub param: PartParam,
    pub option: PartOption,
    pub active: [bool; 16],
    pub motion_seq: Option<[PartParam; 16]>,
}

impl Part {
    pub fn default() -> Self {
        let none_param = PartParam::default_none();
        Self {
            param: PartParam::default(),
            option: PartOption::default(),
            active: [false; 16],
            motion_seq: Some(std::array::from_fn(|_| none_param.clone())),
        }
    }
}
