use crate::app::login::{
    service::{finish_login, Color, LoginUnknownStatus, QrCode},
    Arc, LoginPageMsg,
};
use qrcode_png::QrCodeEcc;
use relm4::Sender;
use resource_loader::{AsyncCreatePath, CaptchaQrCode};
use ricq::{Client, LoginDeviceLocked, LoginNeedCaptcha, LoginResponse};
use tokio::fs;

pub(in crate::app) async fn handle_login_response(
    res: &LoginResponse,
    client: &Arc<Client>,
    sender: &Sender<LoginPageMsg>,
) {
    use LoginPageMsg::LoginFailed;
    match res {
        LoginResponse::Success(_) => {
            finish_login(Arc::clone(client), &sender).await;
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

                sender.send(LoginPageMsg::NeedCaptcha(verify_url.clone(), Arc::clone(client)));
                Result::<_, String>::Ok(())
            };

            if let Err(err) = operate().await {
                sender.send(LoginFailed(err));
            }
        }
        LoginResponse::AccountFrozen => {
            sender.send(LoginFailed("Account Frozen".to_string()));
        }
        LoginResponse::DeviceLocked(LoginDeviceLocked {
            sms_phone,
            verify_url,
            ..
        }) => {
            sender.send(LoginFailed(
                "Device Locked. See more in the pop-up window.".to_string(),
            ));

            sender.send(LoginPageMsg::DeviceLock(
                verify_url.clone().unwrap_or_else(|| "<unknown>".into()),
                sms_phone.clone(),
            ));
        }
        LoginResponse::TooManySMSRequest => {
            sender.send(LoginFailed("Too Many SMS Request".to_string()));
        }
        LoginResponse::DeviceLockLogin(_) => match client.device_lock_login().await {
            Err(err) => {
                sender.send(LoginFailed(err.to_string()));
            }
            Ok(_) => {
                finish_login(Arc::clone(client), &sender).await;
            }
        },
        LoginResponse::UnknownStatus(LoginUnknownStatus { message, .. }) => {
            sender.send(LoginFailed(message.clone()));
        }
    }
}
