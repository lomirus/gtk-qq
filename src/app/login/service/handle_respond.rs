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

pub(in crate::app) async fn handle_login_response(res: &LoginResponse, client: Arc<Client>) {
    let sender = LOGIN_SENDER.get().unwrap();

    use LoginPageMsg::LoginFailed;
    match res {
        LoginResponse::Success(_) => {
            finish_login(client).await;
        }
        LoginResponse::NeedCaptcha(LoginNeedCaptcha { verify_url, .. }) => {
            let verify_url = Into::<Option<&String>>::into(verify_url);
            let operate = || async {
                // Get the captcha url qrcode image path
                let path = CaptchaQrCode::create_and_get_path_async()
                    .await
                    .map_err(|err| err.to_string())?;

                // Generate qrcode image
                let verify_url = verify_url.unwrap();
                let qrcode = QrCode::new(&verify_url, QrCodeEcc::Low)
                    .map_err(|err| err.to_string())?
                    .margin(10)
                    .zoom(5)
                    .generate(Color::Grayscale(0, 255))
                    .map_err(|err| err.to_string())?;

                // Write the image
                fs::write(path, qrcode)
                    .await
                    .map_err(|err| err.to_string())?;

                sender.input(LoginPageMsg::NeedCaptcha(
                    verify_url.clone(),
                    client,
                ));
                Result::<_, String>::Ok(())
            };

            if let Err(err) = operate().await {
                sender.input(LoginFailed(err));
            }
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
                verify_url.clone().unwrap_or_else(|| "<unknown>".into()),
                sms_phone.clone(),
            ));
        }
        LoginResponse::TooManySMSRequest => {
            sender.input(LoginFailed("Too Many SMS Request".to_string()));
        }
        LoginResponse::DeviceLockLogin(_) => match client.device_lock_login().await {
            Err(err) => {
                sender.input(LoginFailed(err.to_string()));
            }
            Ok(_) => {
                finish_login(client).await;
            }
        },
        LoginResponse::UnknownStatus(LoginUnknownStatus { message, .. }) => {
            sender.input(LoginFailed(message.clone()));
        }
    }
}
