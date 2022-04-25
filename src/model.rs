use std::time::Duration;

#[derive(Debug)]
pub struct ClusterMetadata {
    pub version: i16,
    pub throttle_time: Duration,
    pub brokers: Vec<String>,
    pub cluster_id: String,
    pub controller_id: i32,
    pub topics: Vec<TopicMetadata>,
}

#[derive(Debug)]
pub struct TopicMetadata {
    pub name: String,
    /*
    Err        KError
    Name       string
    IsInternal bool // Only valid for Version >= 1
    Partitions []*PartitionMetadata
     */
}
