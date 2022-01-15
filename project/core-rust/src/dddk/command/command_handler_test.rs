#[cfg(test)]
mod tests {
    use crate::dddk::command::command_handler::CommandHandler;
    use crate::dddk::test::some_command_for_test::{ACommand, AnotherCommand};
    use crate::dddk::test::some_command_handler_for_test::ACommandHandler;
    use crate::dddk::test::some_event_for_test::AnEvent;

    #[test]
    fn it_should_handle_command_when_command_is_associated_to_this_handler() {
        // Given
        let a_command_handler = ACommandHandler::new();
        let a_command = ACommand { };

        // When
        let events = a_command_handler.handle_generic_command(&a_command);

        // Then
        assert_eq!(1, events.len());
        let event = events.get(0).unwrap();
        let an_event = event.as_ref().as_any().downcast_ref::<AnEvent>();
        assert_eq!(true, an_event.is_some());
    }

    #[test]
    fn it_should_not_handle_command_when_command_is_not_associated_to_this_handler() {
        // Given
        let a_command_handler = ACommandHandler::new();
        let another_command = AnotherCommand { };

        // When
        let events = a_command_handler.handle_generic_command(&another_command);

        // Then
        assert_eq!(0, events.len());
    }
}