use actix_web::{http::header::Accept, web};

pub fn accepts_mime_type(accept: &web::Header<Accept>, mime_type: &mime::Mime) -> bool {
    accept
        .iter()
        .map(|quality_item| quality_item.item.as_ref())
        .any(|m| m == *mime_type)
}
