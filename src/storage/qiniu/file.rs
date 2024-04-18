use crate::storage::file::ObjectStorage;
use qiniu_sdk::{
    download::{DownloadManager, StaticDomainsUrlsGenerator, UrlsSigner},
    upload::{
        apis::credential::Credential, AutoUploader, AutoUploaderObjectParams, UploadManager,
        UploadTokenSigner,
    },
};
use std::time::Duration;

pub struct QiniuCloud {
    config: super::config::Config,
}

impl QiniuCloud {
    pub fn new(config: super::config::Config) -> Self {
        Self { config }
    }
}

impl ObjectStorage for QiniuCloud {
    fn upload(&self, origin: &str, dst: &str) -> Result<(), Box<dyn std::error::Error>> {
        let credential = Credential::new(
            self.config.access_key.clone(),
            self.config.secret_key.clone(),
        );

        let upload_manager = UploadManager::builder(UploadTokenSigner::new_credential_provider(
            credential,
            self.config.bucket.clone(),
            Duration::from_secs(3600),
        ))
        .build();

        let uploader: AutoUploader = upload_manager.auto_uploader();

        let dst= format!("{}/{}", self.config.store_path.clone(), dst);

        let params = AutoUploaderObjectParams::builder()
            .object_name(&dst)
            .file_name(&dst)
            .build();

        uploader.upload_path(origin, params)?;
        Ok(())
    }

    fn download(&self, origin: &str, dst: &str) -> Result<(), Box<dyn std::error::Error>> {
        let origin = format!("{}/{}", self.config.domain.clone(), origin);

        let download_maneger = DownloadManager::new(UrlsSigner::new(
            Credential::new(
                self.config.access_key.clone(),
                self.config.secret_key.clone(),
            ),
            StaticDomainsUrlsGenerator::builder(self.config.domain.clone())
                .use_https(self.config.use_https.clone())
                .build(),
        ));

        download_maneger.download(origin)?.to_path(dst)?;

        Ok(())
    }
}
