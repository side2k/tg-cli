use crate::utils::request_input;
use grammers_client::{types::Dialog, types::User, Client, Config, SignInError};
use grammers_session::Session;

pub fn get_session(session_file_path: String) -> Session {
    Session::load_file_or_create(&session_file_path).unwrap()
}

pub async fn get_client(api_id: i32, api_hash: String, session: Session) -> Client {
    Client::connect(Config {
        session: session,
        api_id: api_id,
        api_hash: api_hash,
        params: Default::default(),
    })
    .await
    .unwrap()
}

pub async fn login(
    client: &Client,
    phone: String,
    password: Option<String>,
) -> Result<User, SignInError> {
    if client.is_authorized().await.unwrap() {
        return Ok(client.get_me().await.unwrap());
    }

    let token = client.request_login_code(&phone).await.unwrap();
    let code = request_input("Enter code:").unwrap();

    match client.sign_in(&token, &code).await {
        Ok(user) => Ok(user),
        Err(SignInError::PasswordRequired(password_token)) => {
            let hint = password_token.hint().unwrap_or("-");
            let password = password.unwrap_or_else(|| {
                request_input(format!("Enter password(hint: {}):", &hint).as_str()).unwrap()
            });

            client.check_password(password_token, password.trim()).await
        }
        Err(error) => Err(error),
    }
}

pub async fn get_dialog_by_id(client: &Client, dialog_id: i64) -> Result<Dialog, String> {
    let mut dialogs = client.iter_dialogs();

    while let Some(dialog) = dialogs.next().await.unwrap() {
        if dialog.chat().id() == dialog_id {
            return Ok(dialog);
        }
    }

    Err(format!("No dialog with id {} was found", dialog_id))
}
