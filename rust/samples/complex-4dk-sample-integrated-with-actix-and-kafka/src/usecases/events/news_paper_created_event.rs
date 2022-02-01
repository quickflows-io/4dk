use dddk_macro::Event;
use dddk_core::dddk::event::event::Event;
use std::any::Any;

#[derive(Event, Debug)]
pub struct NewsPaperCreatedEvent {
    pub name: String,
}

impl NewsPaperCreatedEvent {
    pub fn new(news_paper_name: &String) -> NewsPaperCreatedEvent {
        NewsPaperCreatedEvent {
            name: news_paper_name.clone(),
        }
    }
}
