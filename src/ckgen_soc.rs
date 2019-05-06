#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control clock and reset of SOC"]
    pub control: CONTROL,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Indicates the reset reason from Cortex-M0"]
    pub reason_rst: REASON_RST,
    _reserved1: [u8; 19usize],
    #[doc = "0x1c - Identification information of the device"]
    pub die_id: DIE_ID,
    #[doc = "0x20 - Enable or gates the APB clock of the peripherals"]
    pub clock_en: CLOCK_EN,
    #[doc = "0x24 - DMA config"]
    pub dma_config: DMA_CONFIG,
}
#[doc = "Control clock and reset of SOC"]
pub struct CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control clock and reset of SOC"]
pub mod control;
#[doc = "Indicates the reset reason from Cortex-M0"]
pub struct REASON_RST {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Indicates the reset reason from Cortex-M0"]
pub mod reason_rst;
#[doc = "Identification information of the device"]
pub struct DIE_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Identification information of the device"]
pub mod die_id;
#[doc = "Enable or gates the APB clock of the peripherals"]
pub struct CLOCK_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable or gates the APB clock of the peripherals"]
pub mod clock_en;
#[doc = "DMA config"]
pub struct DMA_CONFIG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DMA config"]
pub mod dma_config;
