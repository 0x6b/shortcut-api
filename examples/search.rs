use std::{env, error::Error};

use shortcut_api::{
    apis::{
        configuration::{ApiKey, Configuration},
        default_api::{search_epics, search_stories},
    },
    models::search::{Detail, EntityTypes},
    models::Search,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Configuration {
        api_key: Some(ApiKey {
            key: env::var("SHORTCUT_API_TOKEN")?,
            prefix: None,
        }),
        ..Configuration::default()
    };

    let epics = search_epics(
        &config,
        Search {
            query: "documentation".to_string(),
            page_size: Some(25),
            detail: Some(Detail::Full),
            entity_types: Some(vec![EntityTypes::Epic]),
            ..Search::default()
        },
    )
    .await?;
    epics
        .data
        .iter()
        .for_each(|e| println!("{}: {}", e.id, e.name));

    let stories = search_stories(
        &config,
        Search {
            query: "documentation".to_string(),
            page_size: Some(25),
            detail: Some(Detail::Full),
            entity_types: Some(vec![EntityTypes::Story]),
            ..Search::default()
        },
    )
    .await?;
    stories
        .data
        .iter()
        .for_each(|s| println!("{}: {}", s.id, s.name));

    Ok(())
}
