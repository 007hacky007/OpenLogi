//! HID++ transport and channel lifecycle.
//!
//! Everything between the OS HID stack and an open [`hidpp`] channel lives
//! here: the `async-hid` transport ([`transport`]), resolving a
//! [`route::DeviceRoute`] to an open channel ([`route`]), and the two reuse
//! strategies — [`ChannelPool`] for sessions that open on demand and
//! [`ChannelRegistry`] for channels owned by the inventory enumerator.

pub(crate) mod pool;
pub(crate) mod registry;
pub(crate) mod route;
#[cfg(test)]
pub(crate) mod scripted;
pub(crate) mod transport;

pub use pool::ChannelPool;
pub use registry::ChannelRegistry;
