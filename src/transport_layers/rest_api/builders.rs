use crate::domain::services;
use crate::transport_layers::rest_api;

pub struct Builder {
    pub scope: actix_web::Scope,
    pub handlers: rest_api::HandlerMap,
}

impl Default for Builder {
    fn default() -> Builder {
        Builder::new("", None)
    }
}

impl Builder {
    pub fn new(name: &str, handlers: Option<rest_api::HandlerMap>) -> Builder {
        Builder {
            scope: actix_web::web::scope(format!("/{}", name).as_str()),
            handlers: handlers.unwrap_or_else(rest_api::HandlerMap::new),
        }
    }

    pub fn abb_handler<Service>(
        mut self,
        http_method: actix_web::http::Method,
        handler: rest_api::ABBHandler<Service>,
    ) -> Builder
    where
        Service: services::ABBService + 'static,
    {
        self.handlers.insert(handler);
        let scope = self.scope.route(
            "/",
            actix_web::web::method(http_method).to(
                async move |data: actix_web::web::Data<rest_api::RestApiState>,
                            request: actix_web::HttpRequest| {
                    data.handlers
                        .get::<rest_api::ABBHandler<Service>>()
                        .unwrap()
                        .handle(request)
                        .await
                },
            ),
        );
        Builder {
            scope,
            handlers: self.handlers,
        }
    }

    pub fn resource(self, name: &str, factory: impl Fn(Builder) -> Builder) -> Builder {
        let builder = factory(Builder::new(name, Some(self.handlers)));
        Builder {
            scope: self.scope.service(builder.scope),
            handlers: builder.handlers,
        }
    }

    pub fn scope(self) -> actix_web::Scope {
        let state = rest_api::RestApiState {
            handlers: self.handlers,
        };
        self.scope.data(state)
    }
}
