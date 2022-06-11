use typed_builder::TypedBuilder;

#[derive(Debug)]
pub enum Output {
    Submit { ticket: String },
    CopyLink,
}

#[derive(TypedBuilder)]
pub struct PayLoad {
    pub(crate) verify_url: String,
    #[builder(default = String::from("https://github.com/mzdluo123/TxCaptchaHelper"))]
    pub(crate) scanner_url : String,
}
