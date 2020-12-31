use crate::domain;

#[derive(serde::Serialize)]
pub struct Response<Data>
where
    Data: serde::Serialize,
{
    pub data: Option<Data>,
    pub messages: Vec<domain::Message>,
}

impl<Data> Response<Data>
where
    Data: serde::Serialize,
{
    pub fn only_messages(messages: Vec<domain::Message>) -> Response<Data> {
        Response {
            data: None,
            messages,
        }
    }

    pub fn only_data(data: Data) -> Response<Data> {
        Response {
            data: Some(data),
            messages: Vec::new(),
        }
    }
}
