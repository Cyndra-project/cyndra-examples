use cyndra_crontab::{CrontabService, CyndraCrontab};
use cyndra_persist::{Persist, PersistInstance};

#[cyndra_runtime::main]
async fn crontab(#[Persist] persist: PersistInstance) -> CyndraCrontab {
    CrontabService::new(persist)
}
