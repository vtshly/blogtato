use std::collections::HashSet;
use std::io::IsTerminal;

use anyhow::ensure;

use crate::data::BlogData;
use crate::data::index::resolve_posts;
use crate::data::schema::FeedItem;
use crate::display::{RenderCtx, render_grouped};
use crate::query::Query;

pub(crate) fn cmd_show(store: &BlogData, query: &Query) -> anyhow::Result<()> {
    let query = query.or_default_view();
    let resolved = resolve_posts(store, &query)?;
    ensure!(!resolved.items.is_empty(), "No matching posts");

    let read_ids: HashSet<String> = store
        .reads()
        .items()
        .into_iter()
        .map(|r| r.post_id)
        .collect();

    let color = std::io::stdout().is_terminal();
    let max_width = terminal_size::terminal_size().map(|(w, _)| w.0 as usize);
    let refs: Vec<&FeedItem> = resolved.items.iter().collect();
    let ctx = RenderCtx {
        all_keys: &query.keys,
        shorthands: &resolved.shorthands,
        feed_labels: &resolved.feed_labels,
        read_ids: &read_ids,
        color,
        shorthand_width: RenderCtx::shorthand_width_from(&refs, &resolved.shorthands),
        max_width,
    };
    print!("{}", render_grouped(&refs, &ctx));
    Ok(())
}
