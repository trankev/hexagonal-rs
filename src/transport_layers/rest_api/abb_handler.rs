use crate::domain;
use crate::domain::services;

pub type RequestDataBuilder<Service: services::ABBService> = fn(actix_web::HttpRequest) -> Service::RequestData;

#[derive(Clone)]
pub struct ABBHandler<Service>
where
    Service: services::ABBService,
{
    service: Service,
    request_data_builder: RequestDataBuilder<Service>,
}

impl<Service> ABBHandler<Service>
where
    Service: services::ABBService,
{
    pub fn new(service: Service, request_data_builder: RequestDataBuilder<Service>) -> ABBHandler<Service> {
        ABBHandler {service, request_data_builder}
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
        actix_web::web::Json(response)
    }
}

fn do_nothing(_actix_request: actix_web::HttpRequest) -> () {

}

impl<Service> ABBHandler<Service>
where
    Service: services::ABBService<RequestData=()>,
{
    pub fn without_request(service: Service) -> ABBHandler<Service> {
        ABBHandler {service, request_data_builder: do_nothing}
    }
}
