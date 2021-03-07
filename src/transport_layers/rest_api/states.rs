pub type HandlerMap = anymap::Map<dyn anymap::any::CloneAny + Send>;

pub struct RestApiState {
    pub handlers: HandlerMap,
}
