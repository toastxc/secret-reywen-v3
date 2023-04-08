use crate::methods::driver::Delta;

// enter values here for testing
pub async fn tester() -> Delta {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Delta::new(
        // url
        "https://api.revolt.chat/",
        // token
        include_str!("token.txt"),
        // timeout in seconds
        10,
        // is bot account
        true,
    )
}
