mod messages;
mod requests;
mod responses;
pub mod services;

pub use messages::Message;
pub use requests::Request;
pub use responses::Response;

#[derive(serde::Serialize)]
pub struct Empty {}
