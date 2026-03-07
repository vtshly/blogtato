use std::collections::HashMap;

use crate::data::schema::FeedSource;
use crate::shorthand::compute_shorthands;

pub(crate) struct FeedEntry {
    pub feed: FeedSource,
    pub id: String,
    pub shorthand: String,
}

pub(crate) struct FeedIndex {
    pub entries: Vec<FeedEntry>,
}

impl FeedIndex {
    pub(crate) fn id_for_shorthand(&self, shorthand: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|e| e.shorthand == shorthand)
            .map(|e| e.id.as_str())
    }
}

pub(crate) fn feed_index(table: &synctato::Table<FeedSource>) -> FeedIndex {
    let mut feeds = table.items();
    feeds.sort_by(|a, b| a.url.cmp(&b.url));
    let ids: Vec<String> = feeds.iter().map(|f| table.id_of(f)).collect();
    let shorthands = compute_shorthands(&ids);
    let entries = feeds
        .into_iter()
        .zip(ids)
        .zip(shorthands)
        .map(|((feed, id), shorthand)| FeedEntry {
            feed,
            id,
            shorthand,
        })
        .collect();
    FeedIndex { entries }
}

pub(crate) fn resolve_shorthand(
    feeds_table: &synctato::Table<FeedSource>,
    shorthand: &str,
) -> Option<String> {
    let fi = feed_index(feeds_table);
    fi.entries
        .iter()
        .find(|e| e.shorthand == shorthand)
        .map(|e| e.feed.url.clone())
}

pub(crate) fn build_feed_labels(fi: &FeedIndex) -> HashMap<String, String> {
    fi.entries
        .iter()
        .map(|e| {
            let label = if e.feed.title.is_empty() {
                format!("@{} {}", e.shorthand, e.feed.url)
            } else {
                format!("@{} {}", e.shorthand, e.feed.title)
            };
            (e.id.clone(), label)
        })
        .collect()
}
