#[derive(Debug)]
pub struct Constraints {
    pub required_width: u16,
    pub required_height: u16,
}

impl Default for Constraints {
    fn default() -> Self {
        Self { required_width: 10, required_height: 20 }
    }
}

#[derive(Debug)]
pub struct AnimationParams {
    pub frame_rate: u32,
    pub starting_y_pos: u16,
    pub direction: i16,
}

impl Default for AnimationParams {
    fn default() -> Self {
        Self { frame_rate: 60, starting_y_pos: 0, direction: 1 }
    }
}



