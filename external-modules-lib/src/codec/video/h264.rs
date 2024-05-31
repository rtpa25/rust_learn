pub struct H264 {
    pub resolution: (u32, u32),
    pub frame_rate: f32,
    pub bit_rate: u32,
}

impl H264 {
    pub fn new(resolution: (u32, u32), frame_rate: f32, bit_rate: u32) -> Self {
        H264 {
            resolution,
            frame_rate,
            bit_rate,
        }
    }

    pub fn encode(&self) {
        println!(
            "Encoding H264 video with {}x{} resolution, {} fps, and {} bitrate",
            self.resolution.0, self.resolution.1, self.frame_rate, self.bit_rate
        );
    }
}
