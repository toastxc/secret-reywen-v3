#[cfg(test)]
mod tests {

    use crate::{
        methods::{data::*, user},
        tests::common::tester_user,
    };

    #[tokio::test]
    async fn add_friend() {
        let http = tester_user().await;
        if let Err(error) = user::friend::add(http, "01EXAG0ZFX02W7PNQE7W5MT339").await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn edit_user() {
        let http = tester_user().await;
        let data = DataEditUser::new().set_badges(1);

        if let Err(error) = user::edit(http, "01FSRTTGJC1XJ6ZEQJMSX8Q96C", data).await {
            panic!("{:#?}", error);
        }
    }
}
