use crate::utils::request_input;
use grammers_client::{
    client::bots::InvocationError,
    types::{Dialog, Message, User},
    Client, Config, SignInError,
};
use grammers_session::Session;

// A wrapper for grammers_client::Client
pub struct TgCliClient {}

pub struct TgCliConnectedClient {
    /// path to session file
    session_file: String,
    client: Client,
}
pub struct TgCliLoggedInClient {
    /// path to session file
    session_file: String,
    client: Client,
    /// logged in user object
    pub user: User,
}

impl TgCliClient {
    /// Creates and returns new [`TgCliConnectedClient`] instance upon successfull
    /// connection to Telegram.
    pub async fn connect(
        api_id: i32,
        api_hash: String,
        session_file: String,
    ) -> Result<TgCliConnectedClient, String> {
        let session = Session::load_file_or_create(&session_file).unwrap();

        match Client::connect(Config {
            session: session,
            api_id: api_id,
            api_hash: api_hash,
            params: Default::default(),
        })
        .await
        {
            Ok(client) => Ok(TgCliConnectedClient {
                session_file,
                client,
            }),
            Err(error) => Err(format!("Error logging in: {}", error)),
        }
    }
}

impl TgCliConnectedClient {
    /// Check, whether client session is authorized, and return [`TgCliLoggedInClient`], if yes
    pub async fn authorized(&self) -> Result<TgCliLoggedInClient, String> {
        match self.client.is_authorized().await {
            Ok(true) => Ok(TgCliLoggedInClient {
                session_file: self.session_file.clone(),
                client: self.client.clone(),
                user: self.client.get_me().await.unwrap(),
            }),
            Ok(false) => Err(String::from("Not logged in")),
            Err(error) => Err(format!("{}", error)),
        }
    }

    /// Perform login using phone number. If password is required, `password` argument
    /// will be used, if provided - otherwise, user will be asked for entering password
    pub async fn login(
        &self,
        phone: String,
        password: Option<String>,
    ) -> Result<TgCliLoggedInClient, String> {
        if self.client.is_authorized().await.unwrap() {
            return Ok(TgCliLoggedInClient {
                session_file: self.session_file.clone(),
                client: self.client.clone(),
                user: self.client.get_me().await.unwrap(),
            });
        }

        let token = self.client.request_login_code(&phone).await.unwrap();
        let code = request_input("Enter code:").unwrap();

        match self.client.sign_in(&token, &code).await {
            Ok(user) => Ok(TgCliLoggedInClient {
                session_file: self.session_file.clone(),
                client: self.client.clone(),
                user: user,
            }),
            Err(SignInError::PasswordRequired(password_token)) => {
                let hint = password_token.hint().unwrap_or("-");
                let password = password.unwrap_or_else(|| {
                    request_input(format!("Enter password(hint: {}):", &hint).as_str()).unwrap()
                });

                match self
                    .client
                    .check_password(password_token, password.trim())
                    .await
                {
                    Ok(user) => Ok(TgCliLoggedInClient {
                        user: user,
                        client: self.client.clone(),
                        session_file: self.session_file.clone(),
                    }),
                    Err(error) => Err(format!("{}", error)),
                }
            }
            Err(error) => Err(format!("{}", error)),
        }
    }
}

impl TgCliLoggedInClient {
    /// Returns dialog with specified `id`. It is still O(n), but doesn't necessarily
    /// require to iterate through all dialogs - iteration stops when a dialog is
    /// found.
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

    pub async fn send_message(
        &self,
        dialog_id: i64,
        message: String,
    ) -> Result<Message, InvocationError> {
        let dialog = self.get_dialog_by_id(dialog_id).await.unwrap();
        self.client.send_message(dialog.chat(), message).await
    }
}

pub trait SessionSaver {
    async fn save_session(&self) -> Result<(), std::io::Error>;
}

impl SessionSaver for TgCliConnectedClient {
    async fn save_session(&self) -> Result<(), std::io::Error> {
        self.client
            .session()
            .save_to_file(self.session_file.clone())
    }
}

impl SessionSaver for TgCliLoggedInClient {
    async fn save_session(&self) -> Result<(), std::io::Error> {
        self.client
            .session()
            .save_to_file(self.session_file.clone())
    }
}
