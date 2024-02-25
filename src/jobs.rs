mod daily;
mod monthly;
mod weekly;

use tokio_cron_scheduler::{JobScheduler, JobSchedulerError};

pub async fn jobs() -> Result<(), JobSchedulerError> {
    let mut sched = JobScheduler::new().await?;

    // Add code to be run during/after shutdown
    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Shut down done");
        })
    }));

    sched.add(daily::create()?).await?;
    sched.add(weekly::create()?).await?;
    sched.add(monthly::create()?).await?;
    
    sched.start().await?;

    Ok(())
}
