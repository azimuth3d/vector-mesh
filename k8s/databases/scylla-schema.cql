-- ScyllaDB CQL Schema for Chat History
-- Optimized for high throughput, no joins, partitioned by conversation_id for fast retrieval.

CREATE KEYSPACE IF NOT EXISTS chat_db WITH REPLICATION = { 'class': 'SimpleStrategy', 'replication_factor': 3 };

USE chat_db;

-- Table: chat_history
-- Partition Key: conversation_id
-- Clustering Key: timestamp (DESC) for latest messages first
CREATE TABLE IF NOT EXISTS chat_history (
    conversation_id UUID,
    message_id UUID,
    role text,
    content text,
    timestamp timestamp,
    PRIMARY KEY (conversation_id, timestamp, message_id)
) WITH CLUSTERING ORDER BY (timestamp DESC, message_id DESC)
  AND compaction = {'class': 'TimeWindowCompactionStrategy', 'compaction_window_unit': 'HOURS', 'compaction_window_size': 24}
  AND gc_grace_seconds = 0;

-- Add TTL to automatically expire old conversations after 30 days
ALTER TABLE chat_history WITH default_time_to_live = 2592000;
