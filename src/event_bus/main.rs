// Playground code from reading the article from https://blog.digital-horror.com/blog/event-bus-in-tokio/

use async_trait::async_trait;
use eyre::Result;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum EventKind {
    StubEvent(String)
}

#[derive(Debug, Clone)]
pub struct Event {
    pub module: String,
    pub inner: EventKind
}

struct EventBus {
    sender: broadcast::Sender<Event>
}

impl EventBus {
    fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self {
            sender
        }
    }

    fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    fn publish(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}

#[derive(Debug)]
pub struct ModuleCtx {
    pub name: String,
    pub sender: broadcast::Sender<Event>,
    pub receiver: broadcast::Receiver<Event>
}

impl ModuleCtx {
    fn new(name: &str, bus: &EventBus) -> Self {
        let sender = bus.sender.clone();
        let receiver = sender.subscribe();
        Self {
            name: name.to_string(),
            sender,
            receiver
        }
    }
}

#[async_trait]
pub trait Module {
    fn new(ctx: ModuleCtx) -> Self;
    async fn run(&mut self) -> Result<()>;
}

// --- Module examples ---
pub struct Network {
    ctx: ModuleCtx
}


#[async_trait]
impl Module for Network {
    fn new(ctx:ModuleCtx) -> Self {
        Self {
            ctx
        }
    }

    async fn run (&mut self) -> Result<()> {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let event = Event {
                        module: self.ctx.name.to_string(),
                        inner: EventKind::StubEvent("Received some packet".to_string())
                    };
                    self.ctx.sender.send(event).unwrap();
                }
            }
        }
    }
}


pub struct Logger {
    ctx: ModuleCtx
}

#[async_trait]
impl Module for Logger {
    fn new(ctx:ModuleCtx) -> Self {
        Self {
            ctx
        }
    }

    async fn run(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                res = self.ctx.receiver.recv() => {
                    match res {
                        Ok(event) => {
                            println!("{event:?} received");
                        },
                        Err(err) => {
                            println!("Error: {err}")
                        }
                    }
                }
            };
        }
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    let event_bus = EventBus::new();
    
    let logger_ctx = ModuleCtx::new("logger", &event_bus);
    let mut logger_mod = Logger::new(logger_ctx);

    let network_ctx = ModuleCtx::new("network", &event_bus);
    let mut network_mod = Network::new(network_ctx);

    tokio::join!(logger_mod.run(), network_mod.run()).0?;
    
    Ok(())
}