use axum::routing::{get, MethodRouter, Router};
use axum::response::Html;
use axum::extract::Query;
use async_channel::{unbounded, Sender};
use crate::secrets::get_api_key;
use super::OAuthError;

pub struct OAuthHandler {
    server_port: usize,
}
impl OAuthHandler {
    pub fn builder() -> OAuthHandlerBuilder { OAuthHandlerBuilder::default() }

    pub async fn auth(&self) -> Result<String, OAuthError> {
        let (tx, rx) = unbounded::<String>();

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.server_port)).await?;
        let router = Router::new().route("/callback", Self::callback(tx));
        let ws = axum::serve(listener, router);

        open::that_in_background(format!("https://last.fm/api/auth?api_key={}&cb=http://localhost:{}/callback", get_api_key(), self.server_port));

        tokio::select! {
            _ = ws => { Err(OAuthError::WebserverFailed) },
            token = rx.recv() => Ok(token.unwrap())
        }
    }

    fn callback(tx: Sender<String>) -> MethodRouter {
        get(move |Query(params): Query<CallbackQuery>| async move {
            let webpage = format!("
                <div align=center>
                    <h3>LastFM authenticated. Close this tab.</h3><p>token: {}</p>
                </div>
            ", params.token);
            tx.send(params.token.clone()).await.unwrap();
            Html(webpage)
        })
    }
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
