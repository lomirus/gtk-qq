use crate::app::login::{service::init_client, LOGIN_SENDER};

pub(crate) async fn login(account: i64, password: String) {
    use crate::app::login::LoginPageMsg::{LoginFailed, LoginRespond};
    let sender = LOGIN_SENDER.get().unwrap();

    let operate = || async {
        let client = init_client().await.map_err(|err| err.to_string())?;

        sender.input(LoginRespond(
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
        sender.input(LoginFailed(err))
    }
}
