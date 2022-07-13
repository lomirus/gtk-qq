use crate::app::login::service::Color;
use crate::app::login::service::LoginUnknownStatus;
use crate::app::login::service::QrCode;
use crate::app::login::{service::finish_login, Arc, LoginPageMsg, LOGIN_SENDER};
use qrcode_png::QrCodeEcc;
use resource_loader::AsyncCreatePath;
use resource_loader::CaptchaQrCode;
use ricq::LoginDeviceLocked;
use ricq::{Client, LoginNeedCaptcha, LoginResponse};
use tokio::fs;

pub(in crate::app) async fn handle_login_response(
    res: LoginResponse,
    client: Arc<Client>,
) {
    let sender = LOGIN_SENDER.get().unwrap();

    use LoginPageMsg::LoginFailed;
    match res {
        LoginResponse::Success(_) => {
            finish_login(client).await;
        }
        LoginResponse::NeedCaptcha(LoginNeedCaptcha { verify_url, .. }) => {
            // Get the captcha url qrcode image path
            let path = match CaptchaQrCode::create_and_get_path_async().await {
                Ok(path) => path,
                Err(err) => {
                    sender.input(LoginFailed(err.to_string()));
                    return;
                }
            };

            // Generate qrcode image
            let verify_url = verify_url.unwrap();
            let mut qrcode = QrCode::new(verify_url.clone(), QrCodeEcc::Low).unwrap();
            qrcode.margin(10);
            qrcode.zoom(5);

            // Write the image
            let buf = qrcode.generate(Color::Grayscale(0, 255)).unwrap();
            if let Err(err) = fs::write(path, buf).await {
                sender.input(LoginFailed(err.to_string()));
                return;
            };
            sender.input(LoginPageMsg::NeedCaptcha(
                verify_url,
                client.clone(),
            ));
        }
        LoginResponse::AccountFrozen => {
            sender.input(LoginFailed("Account Frozen".to_string()));
        }
        LoginResponse::DeviceLocked(LoginDeviceLocked {
            sms_phone,
            verify_url,
            ..
        }) => {
            sender.input(LoginFailed(
                "Device Locked. See more in the pop-up window.".to_string(),
            ));

            sender.input(LoginPageMsg::DeviceLock(
                verify_url.unwrap_or_else(|| "<unknown>".into()),
                sms_phone,
            ));
        }
        LoginResponse::TooManySMSRequest => {
            sender.input(LoginFailed("Too Many SMS Request".to_string()));
        }
        LoginResponse::DeviceLockLogin(_) => {
            if let Err(err) = client.device_lock_login().await {
                sender.input(LoginFailed(err.to_string()));
            } else {
                finish_login(client).await;
            }
        }
        LoginResponse::UnknownStatus(LoginUnknownStatus { message, .. }) => {
            sender.input(LoginFailed(message));
        }
    }
}