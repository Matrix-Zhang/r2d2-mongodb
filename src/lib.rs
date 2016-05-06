extern crate r2d2;
extern crate mongodb;

use std::fmt;
use std::error;
use std::error::Error as _StdError;
use mongodb::{ThreadedClient, Client};

#[derive(Debug)]
pub enum Error {
    Other(mongodb::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {}", self.description(), self.cause().unwrap())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Other(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Other(ref err) => err.cause()
        }
    }
}

pub struct MongodbConnectionManager {
    host: String,
    port: u16,
}

impl MongodbConnectionManager {
    pub fn new(host: &str, port: u16) 
            -> Result<MongodbConnectionManager, mongodb::error::Error> {
        Ok(MongodbConnectionManager {
            host: host.to_owned(),
            port: port,
        })
    }   
}

impl r2d2::ManageConnection for MongodbConnectionManager {
    type Connection = Client;
    type Error = Error;

    fn connect(&self) -> Result<Client, Error> {
        Client::connect(&self.host, self.port).map_err(|err| Error::Other(err))
    }

    fn is_valid(&self, _conn: &mut Client) -> Result<(), Error> {
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Client) -> bool {
        false
    }
}
