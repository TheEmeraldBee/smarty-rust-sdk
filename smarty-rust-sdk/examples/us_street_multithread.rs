use std::{error::Error, sync::Arc};

use smarty_rust_sdk::{
    sdk::{authentication::SecretKeyCredential, batch::Batch, options::OptionsBuilder},
    us_street_api::{
        client::USStreetAddressClient,
        lookup::{Lookup, MatchStrategy},
    },
};

use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = Lookup {
        street: "1600 Amphitheatre Pkwy".to_string(),
        last_line: "Mountain View, CA".to_string(),
        max_candidates: 10,
        match_strategy: MatchStrategy::Enhanced,
        ..Default::default()
    };

    let mut tasks = vec![];

    env_logger::init();

    let authentication = SecretKeyCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    // Set Up The Options Here
    let options = OptionsBuilder::new()
        .with_license("us-core-cloud")
        .with_logging()
        .authenticate(authentication)
        .build()
        .unwrap();

    let client = Arc::new(USStreetAddressClient::new(options).expect("Failed to create client"));

    for _ in 0..1000 {
        let lookup = lookup.clone();
        let client = client.clone();
        let result = tokio::spawn(async move {
            let mut batch = Batch::default();
            for _ in 0..100 {
                batch.push(lookup.clone()).expect("Overflowed Batch");
            }

            client
                .send(&mut batch)
                .await
                .expect("Failed to send client");
            batch
        });

        tasks.push(result);
    }

    let _ = join_all(tasks).await;

    Ok(())
}
