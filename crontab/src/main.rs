use chrono::Utc;
use std::{future::Future, str::FromStr};
use tokio::time::sleep;

use cron::Schedule;
use cyndra_runtime::async_trait;

// "Run every 2 seconds"
const SCHEDULE: &str = "*/2 * * * * *";

// The function that will be run.
async fn my_job() {
    let now = chrono::offset::Utc::now();
    println!("It is {}", now.format("%Y-%m-%d %H:%M:%S"));
}

pub struct CronService<F> {
    schedule: Schedule,
    job: fn() -> F,
}

impl<F: Future> CronService<F> {
    async fn start(&self) {
        while let Some(next_run) = self.schedule.upcoming(Utc).next() {
            let next_run_in = next_run
                .signed_duration_since(chrono::offset::Utc::now())
                .to_std()
                .unwrap();
            sleep(next_run_in).await;
            (self.job)().await;
        }
    }
}

#[async_trait]
impl<F> cyndra_service::Service for CronService<F>
where
    F: Future + Send + Sync + 'static,
{
    async fn bind(
        mut self,
        _addr: std::net::SocketAddr,
    ) -> Result<(), cyndra_service::error::Error> {
        self.start().await;

        println!("All done.");

        Ok(())
    }
}

#[cyndra_runtime::main]
async fn init(
) -> Result<CronService<impl Future<Output = ()> + Send + 'static>, cyndra_service::Error> {
    let schedule = Schedule::from_str(SCHEDULE).unwrap();
    Ok(CronService {
        schedule,
        job: my_job,
    })
}
