use std::any::{Any, TypeId};
use std::sync::Arc;
use log::info;
use crate::dddk::event::event::Event;
use crate::dddk::event::event_handler::EventHandlerInBus;

pub struct EventHandlerLogger {
    inner_event_handler: Box<dyn EventHandlerInBus>,
}

impl EventHandlerLogger {
    pub fn new(inner_event_handler: Box<dyn EventHandlerInBus>) -> EventHandlerLogger {
        EventHandlerLogger {
            inner_event_handler
        }
    }
}

impl EventHandlerInBus for EventHandlerLogger {
    fn handle_from_bus(&self, event: Arc<dyn Event>) {
        let event_name = event.get_event_name();
        info!("Handling an event [{}] by [{}].",
            event_name.clone(),
            self.inner_event_handler.get_event_handler_name()
        );
        self.inner_event_handler.handle_from_bus(event);
        info!("Event[{}] has been handled by [{}].",
            event_name,
            self.inner_event_handler.get_event_handler_name()
        );
    }

    fn get_associated_event_from_bus(&self) -> TypeId {
        self.inner_event_handler.get_associated_event_from_bus()
    }

    fn as_any(&self) -> &dyn Any {
        self.inner_event_handler.as_any()
    }

    fn get_event_handler_name(&self) -> String {
        self.inner_event_handler.get_event_handler_name()
    }
}

pub fn encapsulated_event_handler_with_logger(event_handlers: Vec<Box<dyn EventHandlerInBus>>) -> Vec<Box<dyn EventHandlerInBus>> {
    event_handlers.into_iter()
        .map(|event_handler|
            { Box::new(EventHandlerLogger::new(event_handler)) as Box<dyn EventHandlerInBus> })
        .collect::<Vec<Box<dyn EventHandlerInBus>>>()
}