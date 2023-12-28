use std::{env, error::Error};

use shortcut_api::{
    apis::{
        configuration::{ApiKey, Configuration},
        default_api::{get_epic, get_member, list_epic_stories},
    },
    models::{GetEpicStories, GetMember},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Configuration {
        api_key: Some(ApiKey { key: env::var("SHORTCUT_API_TOKEN")?, prefix: None }),
        ..Configuration::default()
    };

    let epic = get_epic(&config, 14798).await?;
    println!("Epic {}: {}", epic.id, epic.name);

    let stories =
        list_epic_stories(&config, epic.id, GetEpicStories { includes_description: Some(true) })
            .await?;
    for story in stories {
        println!("Story {}: {}", story.id, story.name);
        if let Some(id) = story.owner_ids.first() {
            let id = id.to_string();
            let member = get_member(&config, &id, GetMember::new());
            println!("  Owner: {}", member.await?.profile.mention_name);
        }
    }

    Ok(())
}
