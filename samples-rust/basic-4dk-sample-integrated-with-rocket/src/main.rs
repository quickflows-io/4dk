#[macro_use]
extern crate rocket;
extern crate dddk_core;
#[macro_use]
extern crate diesel;

use dddk_core::dddk::command::bus_impl::command_bus_shared_btw_threads_with_box::CommandBusParent;
use dddk_core::dddk::command::bus_impl::command_dispatcher_with_box::CommandDispatcher;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_bus::CommandBus;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::event::event::Event;
use crate::infrastructure::api::get_all_foo;
use crate::infrastructure::database::{establish_connection, FooRepositoryAdapter};
use crate::usecases::a_command_handler::ACommandHandler;
use crate::usecases::another_command_handler::AnotherCommandHandler;

pub mod infrastructure;
pub mod domain;
pub mod usecases;
pub mod schema;

pub struct App {
    command_bus: CommandBusParent
}

impl App {
    fn new() -> App {
        let foo_repository = FooRepositoryAdapter::new(establish_connection());
        let a_command_handler = ACommandHandler::new(Box::new(foo_repository));

        let foo_repository = FooRepositoryAdapter::new(establish_connection());
        let another_command_handler = AnotherCommandHandler::new(Box::new(foo_repository));

        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler));
        command_handlers.push(Box::new(another_command_handler));
        let command_dispatcher = CommandDispatcher::new(command_handlers);
        let app = App {
            command_bus: CommandBusParent::new(Box::new(command_dispatcher))
        };
        return app;
    }

}

impl CommandBus for App {
    fn dispatch<'b>(&self, command: &'b dyn Command) -> Vec<Box<dyn Event>> {
        self.command_bus.dispatch(command)
    }
}

unsafe impl Sync for App { }
unsafe impl Send for App { }

#[rocket::main]
async fn main() {
    {
        let app = App::new();
        let _server = rocket::build()
            .manage(app)
            .mount("/", routes![get_all_foo]).launch().await;
    }
}
