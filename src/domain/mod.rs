mod messages;
pub mod services;
mod requests;
mod responses;

pub use messages::Message;
pub use requests::Request;
pub use responses::Response;

#[derive(serde::Serialize)]
pub struct Empty {}
