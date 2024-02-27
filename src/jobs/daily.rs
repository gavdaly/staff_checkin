use tokio_cron_scheduler::{Job, JobBuilder, JobSchedulerError};

/// Create a new job that runs every day at 4am
pub fn create() -> Result<Job, JobSchedulerError> {
    JobBuilder::new()
    .with_cron_job_type()
    .with_schedule("0 0 4 * * *")
    .unwrap()
    .with_run_async(Box::new(|_uuid, mut _l| {
        Box::pin(async move {
            println!("I run async day at 4am UTC");
        })
    }))
    .build()
}
