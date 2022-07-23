use ricq::Client;

use crate::app::login::LoginPageMsg;

pub(crate) async fn login(
    account: i64,
    password: String,
    sender: &relm4::Sender<LoginPageMsg>,
    client: &Client,
) {
    use crate::app::login::LoginPageMsg::{LoginFailed, LoginRespond};

    let operate = || async {
        sender.send(LoginRespond(
            client
                .password_login(account, &password)
                .await
                .map_err(|err| err.to_string())?
                .into(),
        ));

        Result::<_, String>::Ok(())
    };

    if let Err(err) = operate().await {
        sender.send(LoginFailed(err))
    }
}
