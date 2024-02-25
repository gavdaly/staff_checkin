use tokio_cron_scheduler::{Job, JobBuilder, JobSchedulerError};

pub fn create() -> Result<Job, JobSchedulerError> {
    JobBuilder::new()
        .with_cron_job_type()
        .with_schedule("@weekly")
        .unwrap()
        .with_run_async(Box::new(|_uuid, mut _l| {
            Box::pin(async move {
                println!("I run async every Week");
            })
        }))
        .build()
}
