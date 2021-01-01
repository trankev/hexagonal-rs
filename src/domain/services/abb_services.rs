use crate::domain;

#[async_trait::async_trait]
pub trait ABBService: Clone + Send {
    type RequestData;
    type ResponseData: serde::Serialize;

    async fn run(
        &self,
        request: domain::Request<Self::RequestData>
    ) -> Result<
        domain::Response<Self::ResponseData>,
        Box<dyn std::error::Error>
    >;
}
