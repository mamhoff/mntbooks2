use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::*;
use crate::schema::documents::dsl::*;
use diesel::dsl::*;
use uuid::Uuid;
use regex::Regex;

use chrono::prelude::*;

// TODO: missing SKU in frontends

// TODO move to a utility module
pub fn utc_iso_date_string(utc: &DateTime<Utc>) -> String {
    format!("{:04}-{:02}-{:02}", utc.year(), utc.month(), utc.day())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LineItem {
    pub sku: Option<String>,
    pub title: String,
    pub description: String,
    pub quantity: i32,
    pub price_cents: i32,
    pub amount_cents: i32
}

pub fn line_items(inv: &Document) -> Vec<LineItem> {
    let mut items:Vec<LineItem> = Vec::new();
    if let Some(items_str) = &inv.line_items {
        if let Ok(i) = serde_json::from_str::<Vec<LineItem>>(items_str) {
            items.extend(i);
        }
    }
    items
}

pub fn new_invoice_id(conn: &SqliteConnection) -> String {
    let utc: DateTime<Utc> = Utc::now();
    let year = utc.year();
    // FIXME this may assign the same invoice ID to multiple documents
    // as it only increments up from invoice IDs of those with a proper
    // doc_date
    let new_invoice_id = match documents.select(max(invoice_id))
        .filter(doc_date.like(format!("{}-%", year)))
        .filter(kind.eq("invoice"))
        .first::<Option<String>>(conn) {
        Ok(Some(i)) => {
            let parts:Vec<&str> = i.split('-').collect();
            let number = parts.last().unwrap().to_string().parse::<i32>().unwrap();
            format!("{}-{:04}", year, number+1)
        }
        _ => format!("{}-0001", year)
    };

    new_invoice_id
}

pub fn create_invoice(conn: &SqliteConnection, new_document: &Document) -> Document {
    let new_invoice_id = new_invoice_id(conn);
    let inv = Document {
        id: Uuid::new_v4().to_string(),
        invoice_id: Some(new_invoice_id),
        updated_at: utc_iso_date_string(&Utc::now()), // FIXME missing time?
        created_at: utc_iso_date_string(&Utc::now()),
        ..(*new_document).clone()
    };
    // TODO more input validations
    // TODO improve feedback to user providing bad input
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match(&inv.doc_date));

    let res = diesel::insert_into(documents).values(&inv).execute(conn);
    println!("create_invoice result: {:?}", res);

    inv
}

// TODO update_invoice
// FIXME arbitrary limit
pub fn get_all_documents(conn: &SqliteConnection) -> Vec<Document> {
    documents.limit(1000).load::<Document>(conn).unwrap()
}

pub fn get_document_by_id(conn: &SqliteConnection, uuid: &String) -> Document {
    documents.find(uuid).first(conn).unwrap()
}
