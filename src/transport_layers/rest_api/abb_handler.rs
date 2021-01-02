use crate::domain;
use crate::domain::services;
use crate::transport_layers::rest_api;

pub type RequestDataBuilder<Service> = fn(actix_web::HttpRequest) -> <Service as services::ABBService>::RequestData;
pub type ResponseBuilder<Service> = fn(domain::Response<<Service as services::ABBService>::ResponseData>) -> actix_web::HttpResponse;

#[derive(Clone)]
pub struct ABBHandler<Service>
where
    Service: services::ABBService,
{
    service: Service,
    request_data_builder: RequestDataBuilder<Service>,
    response_builder: ResponseBuilder<Service>,
}

impl<Service> ABBHandler<Service>
where
    Service: services::ABBService,
{
    pub fn new(
        service: Service,
        request_data_builder: RequestDataBuilder<Service>,
        response_builder: Option<ResponseBuilder<Service>>,
    ) -> ABBHandler<Service> {
        ABBHandler {
            service,
            request_data_builder,
            response_builder: response_builder.unwrap_or(body_response),
        }
    }

    pub async fn handle(&self, actix_request: actix_web::HttpRequest) -> impl actix_web::Responder {
        let data = (self.request_data_builder)(actix_request);
        let request = domain::Request {data};
        let response = match self.service.run(request).await {
            Ok(response) => response,
            Err(_err) => {
                let error = domain::Message::internal_error();
                domain::Response::only_messages(vec![error])
            }
        };
        (self.response_builder)(response)
    }
}

fn do_nothing(_actix_request: actix_web::HttpRequest) -> () {

}

fn body_response<Data>(response: domain::Response<Data>) -> actix_web::HttpResponse
where Data: serde::Serialize
{
    let body = rest_api::ResponseBody::from_response(response, None);
    actix_web::HttpResponse::Ok().json(body)
}

impl<Service> ABBHandler<Service>
where
    Service: services::ABBService<RequestData=()>,
{
    pub fn without_request(
        service: Service, response_builder: Option<ResponseBuilder<Service>>
    ) -> ABBHandler<Service> {
        ABBHandler {
            service,
            request_data_builder: do_nothing,
            response_builder: response_builder.unwrap_or(body_response),
        }
    }
}
