/// Get the `shard` array for the Identify payload.
/// You could pass this to [`Gateway::authenticate`].
/// Returns: (`shard_id`, `total_shards`)
///
/// Provide the guild ID and total number of shards, outputs:
/// ```python
/// shard_id = (guild_id >> 22) % num_shards
/// ```
///
/// # Example
/// ```rust
/// use omu::gateway::sharding::get_sharding;
///
/// let (shard_id, num_shards) = get_sharding(1, 2);
/// assert_eq!(shard_id, 1);
/// assert_eq!(num_shards, 2);
pub fn get_sharding(guild_id: u64, num_shards: u64) -> (u64, u64) {
    ((guild_id >> 22) % num_shards, num_shards)
}
