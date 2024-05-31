pub struct Transcode {
    pub input: String,
    pub output: String,
}

impl Transcode {
    pub fn new(input: String, output: String) -> Transcode {
        Transcode { input, output }
    }

    pub fn transcode(&self) {
        println!("Transcoding from {} to {}", self.input, self.output);
    }
}
