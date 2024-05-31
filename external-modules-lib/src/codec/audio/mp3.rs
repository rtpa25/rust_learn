pub struct Mp3Codec {
    pub quality: u8,
    pub channels: u8,
    pub bitrate: u32,
}

impl Mp3Codec {
    pub fn new(quality: u8, channels: u8, bitrate: u32) -> Self {
        Mp3Codec {
            quality,
            channels,
            bitrate,
        }
    }

    pub fn encode(&self) {
        println!(
            "Encoding MP3 audio with {} quality, {} channels, and {} bitrate",
            self.quality, self.channels, self.bitrate
        );
    }
}
