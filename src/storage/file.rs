pub trait ObjectStorage: Send {
    fn upload(&self, origin: &str, dst: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn download(&self, origin: &str, dst: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn init_storage(config: crate::config::config::OssConfig) -> Box<dyn ObjectStorage> {
    match config.provider.as_str() {
        "qiniu" => Box::new(crate::storage::qiniu::file::QiniuCloud::new(
            config.configuration.qiniu,
        )),
        _ => panic!("Unknown provider: {}", config.provider),
    }
}

pub struct Default;

impl Default {
    pub fn new() -> Self {
        Self {}
    }
}

impl ObjectStorage for Default {
    fn upload(&self, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        panic!("Not implemented");
    }
    fn download(&self, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        panic!("Not implemented")
    }
}
