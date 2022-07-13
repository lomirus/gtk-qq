use std::convert::Infallible;

use rand::{prelude::StdRng, SeedableRng};
use ricq::version::get_version;

use crate::{static_data::load_cfg, SyncLoadResource};

pub struct Device;

impl SyncLoadResource<StdRng> for Device {
    type Args = ();

    type Error = Infallible;

    fn load_resource(_: Self::Args) -> Result<StdRng, Self::Error> {
        let seed = load_cfg().client.device_seed;
        Ok(StdRng::seed_from_u64(seed))
    }
}

pub struct Protocol;

impl SyncLoadResource<&'static ricq::version::Version> for Protocol {
    type Args = ();

    type Error = Infallible;

    fn load_resource(_: Self::Args) -> Result<&'static ricq::version::Version, Self::Error> {
        Ok(load_cfg().client.version)
    }
}
