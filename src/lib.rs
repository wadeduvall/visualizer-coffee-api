pub mod shot;

pub mod builders;

#[cfg(test)]
mod tests {
    use crate::builders::{Builder, ClientBuilder};
    use crate::shot::{Shot, ShotList};

    #[tokio::test]
    async fn test_get_shots_essentials() {
        let client = ClientBuilder::connect("", "");
        let shot: Shot = client
            .shot("b95d245c-3a33-4b4d-906d-a9cbdd4b052b")
            .essentials()
            .build()
            .await
            .unwrap();
        println!("{shot:?}");
    }

    #[tokio::test]
    async fn test_get_shots() {
        let client = ClientBuilder::connect("", "");
        let shot: Shot = client
            .shot("b95d245c-3a33-4b4d-906d-a9cbdd4b052b")
            .build()
            .await
            .unwrap();
        println!("{shot:?}");
    }

    #[tokio::test]
    async fn test_get_shot_list() {
        let client = ClientBuilder::connect("", "");
        let shot_list: ShotList = client.shot_list().build().await.unwrap();
        println!("{shot_list:?}");
    }
}
