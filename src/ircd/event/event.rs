use crate::ircd::Id;
use crate::ircd::event::EventClock;
use crate::ircd::event::details;

#[derive(Clone,Debug)]
pub struct Event {
    pub id: Id,
    pub timestamp: i64,
    pub clock: EventClock,
    pub target: Id,

    pub details: EventDetails,
}

#[derive(Clone,Debug)]
pub enum EventDetails {
    NewUser(details::NewUser),
    NewChannel(details::NewChannel),
    ChannelJoin(details::ChannelJoin),
}
