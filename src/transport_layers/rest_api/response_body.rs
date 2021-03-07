use crate::domain;
use crate::transport_layers::rest_api;

#[derive(serde::Serialize)]
pub struct ResponseBody<Data>
where
    Data: serde::Serialize,
{
    data: Option<Data>,
    messages: Vec<domain::Message>,
    links: Vec<rest_api::HttpLink>,
}

impl<Data> ResponseBody<Data>
where
    Data: serde::Serialize,
{
    pub fn new(
        data: Option<Data>,
        messages: Option<Vec<domain::Message>>,
        links: Option<Vec<rest_api::HttpLink>>,
    ) -> ResponseBody<Data> {
        ResponseBody {
            data,
            messages: messages.unwrap_or(Vec::new()),
            links: links.unwrap_or(Vec::new()),
        }
    }

    pub fn from_response(
        response: domain::Response<Data>,
        links: Option<Vec<rest_api::HttpLink>>,
    ) -> ResponseBody<Data> {
        ResponseBody {
            data: response.data,
            messages: response.messages,
            links: links.unwrap_or(Vec::new()),
        }
    }
}
