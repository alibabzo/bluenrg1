#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RNG configuration register"]
    pub cr: CR,
    #[doc = "0x04 - RNG status register"]
    pub sr: SR,
    #[doc = "0x08 - RNG 16 bit random value"]
    pub val: VAL,
}
#[doc = "RNG configuration register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG configuration register"]
pub mod cr;
#[doc = "RNG status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG status register"]
pub mod sr;
#[doc = "RNG 16 bit random value"]
pub struct VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG 16 bit random value"]
pub mod val;
