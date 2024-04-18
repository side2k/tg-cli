use crate::utils::request_input;
use grammers_client::{
    types::{Dialog, Message, User},
    Client, Config, SignInError,
};
use grammers_session::Session;

// A wrapper for grammers_client::Client
pub struct TgCliClient {
    session_file: String,
    client: Client,
}

impl TgCliClient {
    pub async fn connect(api_id: i32, api_hash: String, session_file: String) -> TgCliClient {
        let session = Session::load_file_or_create(&session_file).unwrap();
        let client = Client::connect(Config {
            session: session,
            api_id: api_id,
            api_hash: api_hash,
            params: Default::default(),
        })
        .await
        .unwrap();
        TgCliClient {
            session_file,
            client,
        }
    }

    pub async fn save_session(&self) {
        self.client
            .session()
            .save_to_file(self.session_file.clone())
            .unwrap()
    }

    pub async fn is_authorized(&self) -> bool {
        self.client.is_authorized().await.unwrap()
    }

    pub async fn login(
        &self,
        phone: String,
        password: Option<String>,
    ) -> Result<User, SignInError> {
        if self.client.is_authorized().await.unwrap() {
            return Ok(self.client.get_me().await.unwrap());
        }

        let token = self.client.request_login_code(&phone).await.unwrap();
        let code = request_input("Enter code:").unwrap();

        match self.client.sign_in(&token, &code).await {
            Ok(user) => Ok(user),
            Err(SignInError::PasswordRequired(password_token)) => {
                let hint = password_token.hint().unwrap_or("-");
                let password = password.unwrap_or_else(|| {
                    request_input(format!("Enter password(hint: {}):", &hint).as_str()).unwrap()
                });

                self.client
                    .check_password(password_token, password.trim())
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_dialog_by_id(&self, dialog_id: i64) -> Result<Dialog, String> {
        let mut dialogs = self.client.iter_dialogs();

        while let Some(dialog) = dialogs.next().await.unwrap() {
            if dialog.chat().id() == dialog_id {
                return Ok(dialog);
            }
        }

        Err(format!("No dialog with id {} was found", dialog_id))
    }

    pub async fn get_dialogs(&self) -> Vec<Dialog> {
        let mut dialogs: Vec<Dialog> = vec![];
        let mut dialogs_iter = self.client.iter_dialogs();

        while let Some(dialog) = dialogs_iter.next().await.unwrap() {
            dialogs.push(dialog)
        }
        dialogs
    }

    /// If `pattern_or_username` starts with `@`, returns vector filled with dialogs
    /// with matching usernames, otherwise it will be a vector of dialogs
    /// with title containing `pattern_or_username`
    pub async fn get_dialogs_by_name(&self, pattern_or_username: String) -> Vec<Dialog> {
        let mut dialogs = self.get_dialogs().await;

        if pattern_or_username.len() > 0 {
            if pattern_or_username.starts_with("@") {
                let username = pattern_or_username.trim_start_matches("@");
                dialogs = dialogs
                    .into_iter()
                    .filter(|dialog| {
                        dialog.chat().username().unwrap_or("").to_lowercase()
                            == username.to_lowercase()
                    })
                    .collect();
            } else {
                let pattern = pattern_or_username;
                dialogs = dialogs
                    .into_iter()
                    .filter(|dialog| {
                        dialog
                            .chat()
                            .name()
                            .to_lowercase()
                            .contains(pattern.to_lowercase().as_str())
                    })
                    .collect();
            }
        }
        dialogs
    }

    pub async fn send_message(&self, dialog_id: i64, message: String) -> Message {
        let dialog = self.get_dialog_by_id(dialog_id).await.unwrap();
        self.client
            .send_message(dialog.chat(), message)
            .await
            .unwrap()
    }
}
