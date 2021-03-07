use crate::domain::services;
use crate::transport_layers::rest_api;

fn register_route<Service, State>(
    state: &mut State,
    handler: rest_api::ABBHandler<Service>,
    scope: actix_web::Scope,
    http_method: actix_web::http::Method,
) -> actix_web::Scope
where
    State: rest_api::WithHandlerMap + 'static,
    Service: services::ABBService + 'static,
{
    state.mut_handler_map().insert(handler);
    scope.route(
        "/",
        actix_web::web::method(http_method).to(
            async move |data: actix_web::web::Data<State>, request: actix_web::HttpRequest| {
                let found_handler = data
                    .get_handler_map()
                    .get::<rest_api::ABBHandler<Service>>();
                match found_handler {
                    Some(handler) => handler.handle(request).await,
                    None => unreachable!(),
                }
            },
        ),
    )
}

pub struct CollectionResource<'a, State>
where
    State: rest_api::WithHandlerMap + 'static,
{
    state: &'a mut State,
    scope: actix_web::Scope,
}

impl<'a, State> CollectionResource<'a, State>
where
    State: rest_api::WithHandlerMap + 'static,
{
    pub fn new(name: &str, state: &'a mut State) -> CollectionResource<'a, State> {
        CollectionResource {
            state,
            scope: actix_web::web::scope(format!("/{}", name).as_str()),
        }
    }

    pub fn list<Service>(
        self,
        handler: rest_api::ABBHandler<Service>,
    ) -> CollectionResource<'a, State>
    where
        Service: services::ABBService + 'static,
    {
        let scope = register_route(
            self.state,
            handler,
            self.scope,
            actix_web::http::Method::GET,
        );
        CollectionResource {
            state: self.state,
            scope,
        }
    }

    pub fn scope(self) -> actix_web::Scope {
        self.scope
    }
}
