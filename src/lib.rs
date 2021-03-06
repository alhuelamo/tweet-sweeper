use chrono::Duration;
use chrono::prelude::*;
use egg_mode;
use egg_mode::error::Result;
use tokio::runtime::Runtime;

use api::TwitterApi;
use egg_mode::tweet::Tweet;

pub mod api;
pub mod config;

pub fn run(config: &config::Config) -> Result<()> {
    let mut rt = Runtime::new()
        .expect("Failed to create Tokio runtime");

    let client = TwitterApi::new(&config);

    let delete_date = Utc::now() - Duration::days(config.app.delete_days);

    rt.block_on(delete_older_tweets(client, delete_date))
}

async fn delete_older_tweets(api: TwitterApi, delete_date: DateTime<Utc>) -> Result<()> {
    let mut timeline = api.get_initial_timeline();
    timeline.reset();

    while {
        let tl = timeline.older(None).await.unwrap();
        timeline = tl.0;
        let feed = tl.1.response;

        log::debug!("feed size = {}", feed.len());

        let removed = process_feed(&api, &feed, &delete_date).await?;
        log::debug!("Removed {} tweets", removed);

        log::debug!("!feed.is_empty() = {}", !feed.is_empty());
        !feed.is_empty()
    } {}
    log::debug!("END");

    Ok(())
}

async fn process_feed(api: &TwitterApi, feed: &Vec<Tweet>, delete_date: &DateTime<Utc>) -> Result<usize> {
    let mut count: usize = 0;
    for tw in feed {
        if tw.created_at <= *delete_date {
            count += 1;
            log::debug!("Deleting {} -> {}", tw.id, tw.text);
            log::debug!("date = {}", tw.created_at);
            api.delete_tweet(tw.id).await?;
        }
    }
    Ok(count)
}

