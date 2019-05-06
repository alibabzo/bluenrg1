#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA channel configuration register"]
    pub ccr: CCR,
    #[doc = "0x04 - DMA channel number of data register."]
    pub cndtr: CNDTR,
    #[doc = "0x08 - DMA channel peripheral address register"]
    pub cpar: CPAR,
    #[doc = "0x0c - DMA channel memory address register"]
    pub cmar: CMAR,
}
#[doc = "DMA channel configuration register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA channel configuration register"]
pub mod ccr;
#[doc = "DMA channel number of data register."]
pub struct CNDTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA channel number of data register."]
pub mod cndtr;
#[doc = "DMA channel peripheral address register"]
pub struct CPAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA channel peripheral address register"]
pub mod cpar;
#[doc = "DMA channel memory address register"]
pub struct CMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA channel memory address register"]
pub mod cmar;
