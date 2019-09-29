use actix::{Actor, Context, Handler, Message};
use syslog::{Facility, Formatter3164, Logger, LoggerBackend};

pub struct LogActor {
    writer: Logger<LoggerBackend, String, Formatter3164>,
}

impl LogActor {
    pub fn new() -> Self {
        let formatter = Formatter3164 {
            facility: Facility::LOG_USER,
            hostname: None,
            process: "rust-microservice".into(),
            pid: 0,
        };
        let writer = syslog::unix(formatter).unwrap();

        Self { writer }
    }
}

impl Actor for LogActor {
    type Context = Context<Self>;
}

pub struct Log(pub String);

impl Message for Log {
    type Result = ();
}

impl Handler<Log> for LogActor {
    type Result = ();

    fn handle(&mut self, Log(msg): Log, _: &mut Context<Self>) -> Self::Result {
        self.writer.info(msg).ok();
    }
}
