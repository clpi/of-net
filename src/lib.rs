pub mod serve;
pub mod state;
pub mod routes;
pub mod models;
pub mod http;
pub mod util;
pub mod client;
pub(crate) mod error;

use std::{
    fmt::{self, Display},
    error::Error
};
use anyhow::{Context, Error as AError};

#[derive(Debug)]
pub struct ServerERror {

}
impl Display for ServerERror {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server Error")
    }
}  
