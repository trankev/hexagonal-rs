#[derive(serde::Serialize)]
pub struct HttpLink {
    name: String,
    title: String,
    href: String,
    method: String,
}
