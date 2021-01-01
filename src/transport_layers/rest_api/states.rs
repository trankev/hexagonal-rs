pub type HandlerMap = anymap::Map<dyn anymap::any::CloneAny + Send>;

pub trait WithHandlerMap {
    fn mut_handler_map(&mut self) -> &mut HandlerMap;
    fn get_handler_map(&self) -> &HandlerMap;
}
