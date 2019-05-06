#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Indicates the reset reason from BLE"]
    pub reason_rst: REASON_RST,
    _reserved1: [u8; 2usize],
    #[doc = "0x0c - Counter of 32 kHz clock"]
    pub clk32k_count: CLK32K_COUNT,
    _reserved2: [u8; 2usize],
    #[doc = "0x10 - Period of 32 kHz clock"]
    pub clk32k_period: CLK32K_PERIOD,
    #[doc = "0x14 - Measurement of frequency of 32 kHz clock"]
    pub clk32k_freq: CLK32K_FREQ,
    #[doc = "0x18 - Interrupt event for 32 kHz clock measurement"]
    pub clk32k_it: CLK32K_IT,
}
#[doc = "Indicates the reset reason from BLE"]
pub struct REASON_RST {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Indicates the reset reason from BLE"]
pub mod reason_rst;
#[doc = "Counter of 32 kHz clock"]
pub struct CLK32K_COUNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Counter of 32 kHz clock"]
pub mod clk32k_count;
#[doc = "Period of 32 kHz clock"]
pub struct CLK32K_PERIOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Period of 32 kHz clock"]
pub mod clk32k_period;
#[doc = "Measurement of frequency of 32 kHz clock"]
pub struct CLK32K_FREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Measurement of frequency of 32 kHz clock"]
pub mod clk32k_freq;
#[doc = "Interrupt event for 32 kHz clock measurement"]
pub struct CLK32K_IT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt event for 32 kHz clock measurement"]
pub mod clk32k_it;
