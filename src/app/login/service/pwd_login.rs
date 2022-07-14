use crate::app::login::{service::init_client, LoginPageMsg};

pub(crate) async fn login(account: i64, password: String, sender: relm4::Sender<LoginPageMsg>) {
    use crate::app::login::LoginPageMsg::{LoginFailed, LoginRespond};

    let operate = || async {
        let client = init_client().await.map_err(|err| err.to_string())?;

        sender.send(LoginRespond(
            client
                .password_login(account, &password)
                .await
                .map_err(|err| err.to_string())?
                .into(),
            client,
        ));

        Result::<_, String>::Ok(())
    };

    if let Err(err) = operate().await {
        sender.send(LoginFailed(err))
    }
}
