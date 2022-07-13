use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct ClientConfig {
    #[serde(default = "Default::default")]
    #[derivative(Default(value = "Default::default()"))]
    pub(crate) protocol: Protocol,

    #[serde(default = "default_seed")]
    #[derivative(Default(value = "default_seed()"))]
    pub(crate) device_seed: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Derivative, Deserialize)]
#[derivative(Default)]
pub enum Protocol {
    #[serde(alias = "ipad")]
    IPad,
    #[serde(alias = "android-phone")]
    #[serde(alias = "android_phone")]
    AndroidPhone,
    #[serde(alias = "android-watch")]
    #[serde(alias = "android_watch")]
    AndroidWatch,
    #[derivative(Default)]
    #[serde(alias = "macos")]
    MacOS,
    #[serde(alias = "qi_dian")]
    #[serde(alias = "qi-dian")]
    QiDian,
}

impl From<Protocol> for ricq::version::Protocol {
    fn from(val: Protocol) -> Self {
        use ricq::version::Protocol::*;
        match val {
            Protocol::IPad => IPad,
            Protocol::AndroidPhone => AndroidPhone,
            Protocol::AndroidWatch => AndroidWatch,
            Protocol::MacOS => MacOS,
            Protocol::QiDian => QiDian,
        }
    }
}

fn default_seed() -> u64 {
    1145141919810
}
