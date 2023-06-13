pub mod backup_count;
pub mod backup_list;
pub mod failed_archiving_count;
pub mod incremental_count;
pub mod last_archived_time;
pub mod last_backup;
pub mod last_backup_duration;
pub mod last_backup_size;
pub mod oldest_backup;

pub trait Metric<T> {
    fn calculate(&self, data: &T);
}
