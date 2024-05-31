pub struct VP9 {
    pub width: u32,
    pub height: u32,
    pub framerate: f32,
    pub bitrate: u32,
}

impl VP9 {
    pub fn new(width: u32, height: u32, framerate: f32, bitrate: u32) -> Self {
        VP9 {
            width,
            height,
            framerate,
            bitrate,
        }
    }

    pub fn encode(&self) {
        println!(
            "Encoding VP9 video with {}x{} resolution, {} fps, and {} bitrate",
            self.width, self.height, self.framerate, self.bitrate
        );
    }
}
