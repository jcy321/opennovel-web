use super::StreamEvent;
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct MessageStream {
    events: Vec<StreamEvent>,
    position: usize,
}

impl MessageStream {
    pub fn new(events: Vec<StreamEvent>) -> Self {
        Self {
            events,
            position: 0,
        }
    }

    pub fn from_single(event: StreamEvent) -> Self {
        Self::new(vec![event])
    }
}

impl Stream for MessageStream {
    type Item = StreamEvent;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.position < self.events.len() {
            let event = self.events[self.position].clone();
            self.position += 1;
            Poll::Ready(Some(event))
        } else {
            Poll::Ready(None)
        }
    }
}

pub struct StreamBuilder {
    events: Vec<StreamEvent>,
}

impl StreamBuilder {
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn thinking(mut self, agent: &str, content: &str) -> Self {
        self.events.push(StreamEvent::thinking_start(agent));
        for chunk in content.chars().collect::<Vec<_>>().chunks(50) {
            let delta: String = chunk.iter().collect();
            self.events.push(StreamEvent::thinking_delta(agent, &delta));
        }
        self.events.push(StreamEvent::thinking_end(agent));
        self
    }

    pub fn content(mut self, agent: &str, content: &str) -> Self {
        self.events.push(StreamEvent::content_start(agent));
        for chunk in content.chars().collect::<Vec<_>>().chunks(50) {
            let delta: String = chunk.iter().collect();
            self.events.push(StreamEvent::content_delta(agent, &delta));
        }
        self.events.push(StreamEvent::content_end(agent));
        self
    }

    pub fn complete(mut self) -> Self {
        self.events.push(StreamEvent::complete());
        self
    }

    pub fn build(self) -> MessageStream {
        MessageStream::new(self.events)
    }
}

impl Default for StreamBuilder {
    fn default() -> Self {
        Self::new()
    }
}
