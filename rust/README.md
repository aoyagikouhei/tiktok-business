# tiktok-business

TikTok Business API library.

[Documentation](https://docs.rs/tiktok-business)

- OAuth2
- OAuth web example

## Supported APIs


## Features
### default
- reqwest/default-tls

### rustls-tls
- reqwest/rustls-tls

## Changes
[CHANGELOG.md](https://github.com/aoyagikouhei/tiktok-business/blob/main/rust/CHANGELOG.md)

## Examples

### API
```rust
use tiktokapi_v2::{
    apis::get_v2_user_info::Api,
    responses::user::UserField,
};
let access_token = "xxx";
let api = Api::new(UserField::all());
let res = api.execute(access_token).await.unwrap();
println!("{:?}", res);
```