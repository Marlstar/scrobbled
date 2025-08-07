# Scrobbled
Rust bindings for the Last.fm API

# Tutorial
### Important Prerequisite
To make requests to Last.fm you need an API key and secret, which you can get [here](https://www.last.fm/api/account/create). \
Once you have these, make sure to set them in `scrobbled` before making any API requests. To do so:
```rust
#[tokio::main]
async fn main() {
    scrobbled::set_api_key("API KEY");
    scrobbled::set_api_secret("API SHARED SECRET");
}
```

### Authentication
To make authenticated requests to Last.fm, a token is required, which needs users to accept your application as a connection. `scrobbled` does this for you in one simple function:
```rust
let token = scrobbled::auth::get_token().await.unwrap();
println!("{token}"); // abcde...
```
This will open a tab in the user's default browser requesting permission to access their Last.fm account. Once they accept, they will be redirected to a page showing the token, which will automatically be received by `scrobbled` and returned to the caller. Users can close the tab without needing any further action.
