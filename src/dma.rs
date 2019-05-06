#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register"]
    pub isr: ISR,
    #[doc = "0x04 - DMA interrupt flag clear register"]
    pub ifcr: IFCR,
}
#[doc = "DMA interrupt status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA interrupt status register"]
pub mod isr;
#[doc = "DMA interrupt flag clear register"]
pub struct IFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA interrupt flag clear register"]
pub mod ifcr;
