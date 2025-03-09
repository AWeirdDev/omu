use dashmap::DashMap;

use crate::dataclasses::Channel;

pub struct InMemoryCache<T> {
    channels: DashMap<String, Channel<T>>,
}
