use crate::methods::driver::{result, Delta, DeltaError};

pub async fn message_send(http: &Delta, channel: &str, message: &str, emoji: &str, user_id: Option<String>, remove_all: Option<bool>) -> Result<(), DeltaError> {
    let mut url = format!("/channels/{channel}/messages/{message}/reactions/{emoji}").to_owned();
    let mut params = "".to_owned();
    if user_id != None {
        if params.len() > 0 {
            params.push_str("&");
        }
        let uid_string: String = user_id.clone().unwrap();
        params.push_str(&format!("user_id={uid_string}"));
    }
    if remove_all != None {
        if params.len() > 0 {
            params.push_str("&");
        }
        let rall: bool = remove_all.clone().unwrap();
        let rall_str = rall.to_string();
        params.push_str(&format!("remove_all={rall_str}"));
    }
    if params.len() > 0 {
        url.push_str("?");
        url.push_str(params.as_str());
    }
    result(http.delete(url.as_str(), None).await).await
}