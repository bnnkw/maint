use clap::{Args, Parser, Subcommand};
use maint::DataStore;
#[derive(Parser)]
pub struct Cmd {
    #[command(subcommand)]
    pub arg: Arg,
}

#[derive(Subcommand)]
pub enum Arg {
    Customer(Customer),
    Contract(Contract),
    Request(Request),
    Work(Work),
}

#[derive(Args)]
pub struct Customer {
    /// ID of the customer to edit
    pub id: u32,

    /// New name for the customer
    #[arg(long)]
    pub name: Option<String>,
}

#[derive(Args)]
pub struct Contract {
    /// ID of the contract to edit
    pub id: u32,
}

#[derive(Args)]
pub struct Request {
    /// ID of the request to edit
    pub id: u32,
}

#[derive(Args)]
pub struct Work {
    /// ID of the work to edit
    pub id: u32,
}

impl Cmd {
    pub fn run(&self, ds: &DataStore) -> Result<(), Box<dyn std::error::Error>> {
        match &self.arg {
            Arg::Customer(arg) => {
                let entity = ds.get_customer(arg.id)?;
                let updated = crate::get_editor_description(&entity.to_string())?;
                let mut new = updated.parse::<maint::Customer>()?;
                new.id = entity.id;
                let _ = ds.save_customer(new)?;
            }
            Arg::Contract(arg) => {
                let entity = ds.get_contract(arg.id)?;
                let updated = crate::get_editor_description(&entity.to_string())?;
                let mut new = updated.parse::<maint::Contract>()?;
                new.id = entity.id;
                let _ = ds.save_contract(new)?;
            }
            Arg::Request(arg) => {
                let entity = ds.get_request(arg.id)?;
                let updated = crate::get_editor_description(&entity.to_string())?;
                let mut new = updated.parse::<maint::Request>()?;
                new.id = entity.id;
                let _ = ds.save_request(new)?;
            }
            Arg::Work(arg) => {
                let entity = ds.get_work(arg.id)?;
                let updated = crate::get_editor_description(&entity.to_string())?;
                let mut new = updated.parse::<maint::Work>()?;
                new.id = entity.id;
                let _ = ds.save_work(new)?;
            }
        }

        Ok(())
    }
}
