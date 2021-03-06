use crate::{
    quote::{FileType, Quote, ALL_PERMS},
    utils::Error,
};
use std::fs::read_to_string;
use tui::widgets::ListState;

pub fn add_quote_to_db(mut q: Quote) -> Result<Vec<Quote>, Error> {
    let db_content = read_to_string(FileType::Database.get_location()).unwrap_or_default();
    let mut parsed: Vec<Quote> = serde_json::from_str(&db_content).unwrap_or_default();

    if q.1.is_empty() {
        q.1.push("Other".into());
    }

    parsed.push(q);
    std::fs::write(
        FileType::Database.get_location(),
        &serde_json::to_vec(&parsed)?,
    )?;
    Ok(parsed)
}

pub fn remove_quote_by_quote(list_state: &mut ListState, q: &Quote) -> Result<(), Error> {
    if let Some(selected) = list_state.selected() {
        let db_contents = read_to_string(FileType::Database.get_location())?;
        let mut parsed: Vec<Quote> = serde_json::from_str(&db_contents)?;
        let pos = parsed.iter().position(|q_loco| q == q_loco).unwrap();
        parsed.remove(pos);
        std::fs::write(
            FileType::Database.get_location(),
            &serde_json::to_vec(&parsed)?,
        )?;

        if selected != 0 {
            list_state.select(Some(selected - 1));
        }
    }

    Ok(())
}

pub fn read_db() -> Result<Vec<Quote>, Error> {
    let db_content =
        read_to_string(FileType::Database.get_location()).unwrap_or_else(|_| "[]".into());
    let parsed: Vec<Quote> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn get_quote(category_state: &mut ListState, item_state: &mut ListState) -> Quote {
    let quote_type_index = category_state.selected().expect("quote type selected");
    let db = read_db().expect("can read db");

    let q = ALL_PERMS[quote_type_index].to_string();
    db.into_iter()
        .filter(|quote| quote.1.contains(&q))
        .nth(item_state.selected().unwrap_or_default())
        .unwrap()
}

pub fn get_quote_by_content(content: &str) -> Option<Quote> {
    read_db()
        .unwrap_or_default()
        .into_iter()
        .find(|quote| quote.0 == content)
}

pub fn sort_list() -> Result<(), Error> {
    let mut db: Vec<_> = read_db()?
        .into_iter()
        .map(|quote| {
            let mut l = quote.1.clone();
            l.sort();

            Quote(quote.0, l)
        })
        .collect();
    db.sort();

    std::fs::write(FileType::Database.get_location(), &serde_json::to_vec(&db)?)?;

    Ok(())
}
