use std::{io, str::FromStr};
use tracing;

use ulid::Ulid;
use zenoh::{self, config::ZenohId, Config};

use crate::holobank;
use holobank::HoloBank;

/// Folder structure - ./nebula/{system_name}/{spaceport_name}
const ROOT_DIR: &str = "./nebula";
const BANK_DATA: &str = "holobank";

pub struct Spaceport {
    // Designed around a zenoh session
    // multiple spaceports on one celestial body is possible
    // use hyperrail to communicate between local spaceports

    // has means to discover (scout) and establish
    // (auto config) comms with other spaceports

    // Has two means of communication: fast messages (radio) or
    // large payloads (ships)
    id: Ulid,
    name: String,
    transport: zenoh::Session,
    bank: Option<HoloBank>,
}

impl Spaceport {
    /// Build a new spaceport.  There can be multiple spaceports on a
    /// celestial body but only a single spaceport per process.
    pub async fn new(name: &str) -> Spaceport {
        let config = Spaceport::configure().unwrap();
        let z_session = zenoh::open(config).await.unwrap();
        let bank = HoloBank::new("./test");

        // look up system name
        // if system is new, name the system

        Spaceport {
            z_session,
            bank
        }
    }
    /// Open essential docks and facilities.
    pub async fn open(name: &str) -> Spaceport {
        todo!()
    }
    /// Close all docks and put facilities into hibernation.
    pub async fn close() {
        todo!()
    }

    fn configure() -> io::Result<Config> {
        let mut config = zenoh::Config::default();
        config.set_id(ZenohId::from_str("spaceport1").unwrap()).unwrap();
        config.connect.endpoints.set(
            ["tcp/10.10.10.10:7447", "tcp/11.11.11.11:7447"].iter().map(|s|s.parse().unwrap()).collect()).unwrap();
        Ok(config)
    }
}