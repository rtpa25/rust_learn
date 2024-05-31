pub struct Flac2 {
    pub version: u8,
    pub channels: u8,
    pub sample_rate: u32,
    pub bits_per_sample: u8,
    pub total_samples: u64,
    pub md5: [u8; 16],
}

impl Flac2 {
    pub fn new(
        version: u8,
        channels: u8,
        sample_rate: u32,
        bits_per_sample: u8,
        total_samples: u64,
        md5: [u8; 16],
    ) -> Self {
        Flac2 {
            version,
            channels,
            sample_rate,
            bits_per_sample,
            total_samples,
            md5,
        }
    }

    pub fn decode(&self) {
        println!(
            "Decoding FLAC audio with version {}, {} channels, {} Hz sample rate, {} bits per sample, {} total samples, and MD5 checksum {:x?}",
            self.version, self.channels, self.sample_rate, self.bits_per_sample, self.total_samples, self.md5
        );
    }
}
