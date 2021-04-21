use std::process;

use chrono::Duration;
use chrono::prelude::*;
use config::AppConf;
use egg_mode;
use egg_mode::error::{Error, Result};
use report::Report;
use tokio::runtime::Runtime;

use api::TwitterApi;
use egg_mode::tweet::Tweet;

pub mod api;
pub mod config;
pub mod report;

pub fn run(config: config::Config) {
    let mut rt = Runtime::new()
        .expect("Failed to create Tokio runtime");

    let client = TwitterApi::new(&config);

    match rt.block_on(delete_older_tweets(client, &config.app)) {
        Ok(report) => good_ending(report),
        Err(egg_error) => bad_ending(egg_error),
    }
}

async fn delete_older_tweets(api_client: TwitterApi, app_conf: &AppConf) -> Result<Report> {
    let delete_date = Utc::now() - Duration::days(app_conf.delete_days);

    let mut timeline = api_client.get_initial_timeline();
    timeline.reset();
    let mut total_removed: u32 = 0;

    while {
        let tl = timeline.older(None).await.unwrap();
        timeline = tl.0;
        let feed = tl.1.response;

        log::debug!("feed size = {}", feed.len());

        let removed = process_feed(&api_client, &feed, &delete_date, &app_conf.ignore_liked_by_me).await?;
        log::debug!("Removed {} tweets", removed);
        total_removed += removed as u32;

        log::debug!("!feed.is_empty() = {}", !feed.is_empty());
        !feed.is_empty()
    } {}
    log::debug!("END");

    Ok(Report::new(total_removed))
}

async fn process_feed(api_client: &TwitterApi, feed: &Vec<Tweet>, delete_date: &DateTime<Utc>, ignore_liked_by_me: &bool) -> Result<usize> {
    let mut count: usize = 0;
    for tw in feed {
        let liked_by_me = tw.favorited.unwrap_or(false);
        if tw.created_at <= *delete_date && !(*ignore_liked_by_me && liked_by_me) {
            count += 1;
            log::debug!("Deleting {} -> {}", tw.id, tw.text);
            log::debug!("date = {}", tw.created_at);
            api_client.delete_tweet(tw.id).await?;
        }
    }
    Ok(count)
}

fn good_ending(report: Report) {
    match report.get_number_of_removed_tweets() {
        0 => log::info!("No tweets removed!"),
        n_removed => log::info!("Successfully removed {} tweets!", n_removed),
    }
}

fn bad_ending(egg_error: Error) {
    log::error!("Something went wrong...");
    log::error!("{}", egg_error);
    process::exit(1);
}
