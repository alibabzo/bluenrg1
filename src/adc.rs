#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 2usize],
    #[doc = "0x02 - ADC control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - ADC configuration register"]
    pub conf: CONF,
    #[doc = "0x08 - IRQ masked status register"]
    pub irqstat: IRQSTAT,
    _reserved1: [u8; 3usize],
    #[doc = "0x0c - It sets the mask for ADC interrupt"]
    pub irqmask: IRQMASK,
    _reserved2: [u8; 7usize],
    #[doc = "0x14 - Result of the conversion in two complement format:<ul><li>if ROUND16=0: result is mapped on all 32-bit (can be truncated with loss of ADCDATAOUT\\[30:15\\])</li><li>if ROUND16=1: result is mapped on 16-bit (can be truncated with loss of ADCDATAOUT\\[15:0\\])</li></ul>"]
    pub data_conv: DATA_CONV,
    #[doc = "0x18 - Offset for correction of converted data"]
    pub offset: OFFSET,
    _reserved3: [u8; 4usize],
    #[doc = "0x20 - ADC status register"]
    pub sr_reg: SR_REG,
    _reserved4: [u8; 3usize],
    #[doc = "0x24 - High threshold for window comparator"]
    pub threshold_hi: THRESHOLD_HI,
    #[doc = "0x28 - Low threshold for window comparator"]
    pub threshold_lo: THRESHOLD_LO,
}
#[doc = "ADC control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ADC control register"]
pub mod ctrl;
#[doc = "ADC configuration register"]
pub struct CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC configuration register"]
pub mod conf;
#[doc = "IRQ masked status register"]
pub struct IRQSTAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "IRQ masked status register"]
pub mod irqstat;
#[doc = "It sets the mask for ADC interrupt"]
pub struct IRQMASK {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "It sets the mask for ADC interrupt"]
pub mod irqmask;
#[doc = "Result of the conversion in two complement format:<ul><li>if ROUND16=0: result is mapped on all 32-bit (can be truncated with loss of ADCDATAOUT\\[30:15\\])</li><li>if ROUND16=1: result is mapped on 16-bit (can be truncated with loss of ADCDATAOUT\\[15:0\\])</li></ul>"]
pub struct DATA_CONV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result of the conversion in two complement format:<ul><li>if ROUND16=0: result is mapped on all 32-bit (can be truncated with loss of ADCDATAOUT\\[30:15\\])</li><li>if ROUND16=1: result is mapped on 16-bit (can be truncated with loss of ADCDATAOUT\\[15:0\\])</li></ul>"]
pub mod data_conv;
#[doc = "Offset for correction of converted data"]
pub struct OFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Offset for correction of converted data"]
pub mod offset;
#[doc = "ADC status register"]
pub struct SR_REG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ADC status register"]
pub mod sr_reg;
#[doc = "High threshold for window comparator"]
pub struct THRESHOLD_HI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High threshold for window comparator"]
pub mod threshold_hi;
#[doc = "Low threshold for window comparator"]
pub struct THRESHOLD_LO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low threshold for window comparator"]
pub mod threshold_lo;
