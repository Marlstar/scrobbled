# Scrobbled
Rust bindings for the Last.fm API

# Tutorial
## Important Prerequisite
To make requests to Last.fm you need an API key and secret, which you can get [here](https://www.last.fm/api/account/create). \
Once you have these, make sure to set them in `scrobbled` before making any API requests. To do so:
```rust
#[tokio::main]
async fn main() {
    scrobbled::set_api_key("API KEY");
    scrobbled::set_api_secret("API SHARED SECRET");
}
```

## Authentication
### Token
To make authenticated requests to Last.fm, a "web service session" is required. To obtain one, we first need an auth token:
```rust
let token = scrobbled::auth::get_token().await.unwrap();
println!("{token}"); // abcde...
```
This will open a tab in the user's default browser requesting permission to access their Last.fm account. Once they accept, they will be redirected to a page showing the token, which will automatically be received by `scrobbled` and returned to the caller. Users can close the tab without needing any further action.

### Session
Once we have a token, to get a session and start interacting with the API, create a `Session`:
```rust
use scrobbled::Session;
let mut session = Session::new(token).await.unwrap();
// Get a session key
session.start().await.unwrap();
// Ready to go =)
```
