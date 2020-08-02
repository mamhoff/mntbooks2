// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::*;


#[derive(serde::Serialize, serde::Deserialize, Clone, Queryable, Insertable, Identifiable)]
pub struct BookingDoc {
    pub id: i32,
    pub booking_id: Option<String>,
    pub doc_id: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Queryable, Insertable, Identifiable)]
pub struct Booking {
    pub id: String,
    pub booking_date: String,
    pub amount_cents: i32,
    pub details: String,
    pub currency: String,
    pub receipt_url: Option<String>,
    pub tax_code: Option<String>,
    pub debit_account: String,
    pub credit_account: String,
    pub txn_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub comment: Option<String>,
    pub done: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Queryable, Insertable, Identifiable)]
#[primary_key(path)]
pub struct DocumentImage {
    pub path: String,
    pub pdf_path: String,
    pub mime_type: String,
    pub doc_id: Option<String>,
    pub extracted_text: String,
    pub done: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Queryable, Insertable, Identifiable)]
pub struct Document {
    pub id: String,
    pub kind: String,
    pub doc_date: String,
    pub amount_cents: Option<i32>,
    pub currency: Option<String>,
    pub tax_code: Option<String>,
    pub serial_id: Option<String>,
    pub order_id: Option<String>,
    pub payment_method: Option<String>,
    pub line_items: Option<String>,
    pub customer_account: Option<String>,
    pub customer_company: Option<String>,
    pub customer_name: Option<String>,
    pub customer_address_1: Option<String>,
    pub customer_address_2: Option<String>,
    pub customer_zip: Option<String>,
    pub customer_city: Option<String>,
    pub customer_state: Option<String>,
    pub customer_country: Option<String>,
    pub vat_included: Option<String>,
    pub replaces_id: Option<String>,
    pub replaced_by_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub account: Option<String>,
}

