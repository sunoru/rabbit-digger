use std::time::SystemTime;

use rd_interface::{Address, Arc};
use serde::ser::Serializer;
use serde_derive::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub enum EventType {
    NewTcp(Address),
    CloseConnection,
    Outbound(usize),
    Inbound(usize),
}

#[derive(Debug, Serialize)]
pub struct Event {
    pub uuid: Uuid,
    pub event_type: EventType,
    #[serde(serialize_with = "serialize_system_time")]
    pub time: SystemTime,
}

fn serialize_system_time<S>(system_time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let n = system_time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    serializer.serialize_u64(n.as_millis() as u64)
}

pub type BatchEvent = Vec<Arc<Event>>;

impl Event {
    pub fn new(uuid: Uuid, event_type: EventType) -> Event {
        Event {
            uuid,
            event_type,
            time: SystemTime::now(),
        }
    }
}
