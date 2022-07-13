use std::convert::Infallible;

use rand::{prelude::StdRng, SeedableRng};

use crate::{static_data::load_cfg, SyncLoadResource};

pub struct Device;

impl SyncLoadResource<ricq::device::Device> for Device {
    type Args = ();

    type Error = Infallible;

    fn load_resource(_: Self::Args) -> Result<ricq::device::Device, Self::Error> {
        let seed = load_cfg().client.device_seed;
        Ok(ricq::device::Device::random_with_rng(
            &mut StdRng::seed_from_u64(seed),
        ))
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
