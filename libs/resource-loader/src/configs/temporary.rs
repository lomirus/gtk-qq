use std::path::Path;

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use tempfile::TempDir;

use super::{free_path_ref, static_leak};

default_string! {
    CaptchaQrCode => "captcha_url.png"
}

#[derive(Debug, Clone, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct TemporaryConfig {
    #[derivative(Default(value = "CaptchaQrCode::get_default()"))]
    #[serde(default = "CaptchaQrCode::get_default")]
    #[serde(alias = "captcha", alias = "captcha_url")]
    captcha_qrcode: String,
}

#[derive(Debug)]
pub(crate) struct InnerTemporaryConfig {
    pub(crate) temp_dir: TempDir,
    pub(crate) captcha_file: &'static Path,
}

impl TemporaryConfig {
    pub(crate) fn into_inner(self) -> InnerTemporaryConfig {
        let temp_dir = tempfile::tempdir().expect("Cannot Create Temporary Directory");

        let captcha_file = static_leak(temp_dir.path().join(self.captcha_qrcode).into_boxed_path());

        InnerTemporaryConfig {
            temp_dir,
            captcha_file,
        }
    }
}

impl Drop for InnerTemporaryConfig {
    fn drop(&mut self) {
        free_path_ref(self.captcha_file)
    }
}

#[cfg(test)]
mod test {
    use super::TemporaryConfig;

    #[test]
    fn test_tmp_file() {
        let temp = TemporaryConfig::default();

        let inner = temp.into_inner();

        println!("{:?}", inner)
    }
}
