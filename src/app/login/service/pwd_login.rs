use crate::app::login::{
    service::{handle_login_response, init_client},
    LOGIN_SENDER,
};

pub(crate) async fn login(account: i64, password: String) {
    use crate::app::login::LoginPageMsg::LoginFailed;
    let sender = LOGIN_SENDER.get().unwrap();

    let client = match init_client().await {
        Ok(client) => client,
        Err(err) => {
            sender.input(LoginFailed(err.to_string()));
            return;
        }
    };

    let res = match client.password_login(account, &password).await {
        Ok(res) => res,
        Err(err) => {
            sender.input(LoginFailed(err.to_string()));
            return;
        }
    };
    handle_login_response(res, account, password, client).await;
}
