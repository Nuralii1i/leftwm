use super::utils::screen::Screen;
use super::utils::window;
//use std::collections::VecDeque;
//use std::sync::{Arc, Mutex};

pub enum EventQueueItem {
    KeyDown(Vec<String>, String),
    WindowCreate(window::Window),
    WindowDestroy(window::WindowHandle),
    ScreenCreate(Screen),
}

//pub type EventQueue = Arc<Mutex<VecDeque<EventQueueItem>>>;
//
//pub fn new() -> EventQueue {
//    let q: VecDeque<EventQueueItem> = VecDeque::new();
//    Arc::new(Mutex::new(q))
//}
