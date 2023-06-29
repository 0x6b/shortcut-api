use std::{env, error::Error};

use url::Url;

use shortcut_api::{
    apis::{
        configuration::{ApiKey, Configuration},
        default_api::search_epics,
    },
    models::{
        search::{Detail, EntityTypes},
        Search,
    },
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

    let mut results = Vec::new();

    let search_term = "documentation";

    let mut search_option = Search {
        query: search_term.to_string(),
        page_size: Some(5),
        detail: Some(Detail::Full),
        entity_types: Some(vec![EntityTypes::Epic]),
        ..Search::default()
    };

    loop {
        let epics = search_epics(&config, search_option.clone()).await?;
        results.extend(epics.data);

        if epics.next.is_none() {
            break;
        }

        println!("count: {}", results.len());

        // From [Shortcut Rest API, V3](https://developer.shortcut.com/api/rest/v3#Search-1273),
        // the `next` value from the previous response can be used as the path and
        // query string for the next page to ensure stable ordering. Since auto-generated client
        // can't handle this, we have to do it manually.
        search_option.query = format!(
            "{}&{}",
            search_term,
            // parse the next URL with the base path in order to get the query string for next
            // request
            Url::parse(format!("{}{}", config.base_path, epics.next.unwrap()).as_str())?
                .query_pairs()
                // filter out the "query" parameter, otherwise it will be duplicated
                // (query=...&query=...)
                .filter(|(k, _)| k != "query")
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("&")
        );
    }

    results
        .iter()
        .for_each(|e| println!("{}: {}", e.id, e.name));

    Ok(())
}
