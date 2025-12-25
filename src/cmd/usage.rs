use chrono::NaiveDate;
use clap::Args;

use crate::cmd::today_utc;

#[derive(Args)]
pub struct Usage {
    /// ID of the contract to show usage for
    pub contract_id: u32,

    /// Date to show usage for (YYYY-MM-DD). Defaults to today.
    #[arg(default_value_t = today_utc())]
    pub date: NaiveDate,
}
