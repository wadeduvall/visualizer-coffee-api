pub mod shot;

pub mod builder;

#[cfg(test)]
mod tests {
    use crate::builder::{Builder, ClientBuilder};
    use crate::shot::Shot;

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
}
