use crate::core::part::{param::PartParam, part::Part};

pub struct Sequencer {
    pub step: usize,
    pub act_step: [bool; 16],
}

impl Sequencer {
    pub fn default() -> Self {
        Self {
            step: 0,
            act_step: [true; 16],
        }
    }

    pub fn step(&mut self) {
        let mut next_step = self.step + 1;

        if !self.act_step.contains(&true) {
            self.act_step[0] = true;
        }

        loop {
            if next_step >= 16 {
                next_step = 0;
            }

            if self.act_step[next_step] {
                break;
            }
            next_step += 1;
        }
        self.step = next_step;
    }

    pub fn step_jump(&mut self, target: usize) -> bool {
        if target >= 16 || !self.act_step[target] {
            return false;
        }
        self.step = target;
        true
    }

    pub fn part_step_param(&self, part: Part) -> PartParam {
        let mut param = part.param.fallback_default();
        if let Some(motion) = part.motion_seq {
            param = motion[self.step].fallback(param);
        }
        param
    }
}
