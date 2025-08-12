use axum::routing::{get, MethodRouter, Router};
use axum::response::Html;
use axum::extract::Query;
use async_channel::{unbounded, Sender, Receiver};
use crate::secrets::get_api_key;
use super::OAuthToken;
use super::OAuthError;
use std::sync::LazyLock;

static CHANNEL: LazyLock<(Sender<String>, Receiver<String>)> = LazyLock::new(unbounded);

pub struct OAuthHandler {
    server_port: usize,
}
impl OAuthHandler {
    pub fn builder() -> OAuthHandlerBuilder { OAuthHandlerBuilder::default() }

    #[cfg(feature = "builtin-callback")]
    pub async fn auth(&self) -> Result<OAuthToken, OAuthError> {
        Self::open_auth_site(&format!("http://localhost:{}/callback", self.server_port));
        tokio::select! {
            _ = self.run_webserver() => { Err(OAuthError::WebserverFailed) },
            token = self.auth_channel_rx() => Ok(OAuthToken(token))
        }
    }

    #[cfg(feature = "custom-callback")]
    pub async fn auth_custom(&self, callback: &str) -> Result<OAuthToken, OAuthError> {
        Self::open_auth_site(callback);
        Ok(OAuthToken(self.auth_channel_rx().await))
    }

    fn open_auth_site(cb: &str) {
        open::that_in_background(format!("https://last.fm/api/auth?api_key={}&cb={cb}", get_api_key()));
    }

    async fn run_webserver(&self) -> Result<(), OAuthError> {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.server_port)).await?;
        let router = Router::new().route("/callback", callback_router());
        axum::serve(listener, router).await
            .map_err(|_| OAuthError::WebserverFailed)
    }

    async fn auth_channel_rx(&self) -> String {
        CHANNEL.1.recv().await.unwrap()
    }
}

pub fn callback_router() -> MethodRouter {
    get(move |Query(params): Query<CallbackQuery>| async move {
            let webpage = format!("
                <div align=center>
                    <h3>LastFM authenticated. Close this tab.</h3><p>token: {}</p>
                </div>
            ", params.token);
            CHANNEL.0.send(params.token.clone()).await.unwrap();
            Html(webpage)
    })
}

#[derive(Debug, Clone, serde::Deserialize)]
struct CallbackQuery {
    token: String,
}



pub struct OAuthHandlerBuilder {
    port: usize,
}
impl Default for OAuthHandlerBuilder {
    fn default() -> Self { Self {
        port: 6227
    }}
}
impl OAuthHandlerBuilder {
    pub async fn build(self) -> OAuthHandler {
        OAuthHandler {
            server_port: self.port,
        }
    }

    pub fn port(&mut self, port: usize) -> &mut Self {
        self.port = port;
        self
    }
}
