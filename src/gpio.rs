#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IO0 to IO14 data value.<p>Writing to a bit will drive the written value on the corresponding IO when it is configured in GPIO mode and the output direction. Reading a bit indicates the pin value</p>"]
    pub data: DATA,
    #[doc = "0x04 - GPIO output enable register (1 bit per GPIO).<ul><li>0: input mode.</li><li>1: output mode</li></ul>"]
    pub oen: OEN,
    #[doc = "0x08 - Pull enable (1 bit per IO).<ul><li>0: pull disabled.</li><li>1: pull enabled</li></ul>"]
    pub pe: PE,
    #[doc = "0x0c - IO driver strength (1 bit per IO).<ul><li>0: 2mA.</li><li>1: 4 mA</li></ul>"]
    pub ds: DS,
    #[doc = "0x10 - Interrupt sense register (1 bit per IO).<ul><li>0: edge detection.</li><li>1: level detection</li></ul>"]
    pub is: IS,
    #[doc = "0x14 - Interrupt edge register (1 bit per IO).<ul><li>0: single edge.</li><li>1: both edges</li></ul>"]
    pub ibe: IBE,
    #[doc = "0x18 - Interrupt event register (1 bit per IO).<ul><li>0: falling edge or low level.</li><li>1: rising edge or high level</li></ul>"]
    pub iev: IEV,
    #[doc = "0x1c - Interrupt mask register (1 bit per IO).<ul><li>0: masked.</li><li>1: not masked</li></ul>"]
    pub ie: IE,
    _reserved0: [u8; 4usize],
    #[doc = "0x24 - Masked interrupt status register (1 bit per IO)"]
    pub mis: MIS,
    #[doc = "0x28 - Interrupt clear register (1 bit per IO).<ul><li>0: no effect.</li><li>1: clear interrupt</li></ul>"]
    pub ic: IC,
    #[doc = "0x2c - Select mode for IO0 to IO7.<ul><li>000b: GPIO mode.</li><li>001b: Serial1 mode.</li><li>100b: Serial0 mode.</li><li>101b: Microphone/ADC mode.</li></ul>"]
    pub mode0: MODE0,
    #[doc = "0x30 - Select mode for IO8 to IO14.<ul><li>000b: GPIO mode.</li><li>001b: Serial1 mode.</li><li>100b: Serial0 mode.</li><li>101b: Microphone/ADC mode.</li></ul>"]
    pub mode1: MODE1,
    _reserved1: [u8; 8usize],
    #[doc = "0x3c - Set some bits of DATA when in GPIO mode without affecting the others (1 bit per IO).<ul><li>0: no effect.</li><li>1: set at 1 the bit</li></ul>"]
    pub dats: DATS,
    #[doc = "0x40 - Clear some bits of DATA when in GPIO mode without affecting the others (1 bit per IO).<ul><li>0: no effect.</li><li>1: clear at 0 the bit</li></ul>"]
    pub datc: DATC,
    #[doc = "0x44 - Select the IO to be used as capture input for the MFTX timers"]
    pub mftx: MFTX,
}
#[doc = "IO0 to IO14 data value.<p>Writing to a bit will drive the written value on the corresponding IO when it is configured in GPIO mode and the output direction. Reading a bit indicates the pin value</p>"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO0 to IO14 data value.<p>Writing to a bit will drive the written value on the corresponding IO when it is configured in GPIO mode and the output direction. Reading a bit indicates the pin value</p>"]
pub mod data;
#[doc = "GPIO output enable register (1 bit per GPIO).<ul><li>0: input mode.</li><li>1: output mode</li></ul>"]
pub struct OEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO output enable register (1 bit per GPIO).<ul><li>0: input mode.</li><li>1: output mode</li></ul>"]
pub mod oen;
#[doc = "Pull enable (1 bit per IO).<ul><li>0: pull disabled.</li><li>1: pull enabled</li></ul>"]
pub struct PE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull enable (1 bit per IO).<ul><li>0: pull disabled.</li><li>1: pull enabled</li></ul>"]
pub mod pe;
#[doc = "IO driver strength (1 bit per IO).<ul><li>0: 2mA.</li><li>1: 4 mA</li></ul>"]
pub struct DS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO driver strength (1 bit per IO).<ul><li>0: 2mA.</li><li>1: 4 mA</li></ul>"]
pub mod ds;
#[doc = "Interrupt sense register (1 bit per IO).<ul><li>0: edge detection.</li><li>1: level detection</li></ul>"]
pub struct IS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt sense register (1 bit per IO).<ul><li>0: edge detection.</li><li>1: level detection</li></ul>"]
pub mod is;
#[doc = "Interrupt edge register (1 bit per IO).<ul><li>0: single edge.</li><li>1: both edges</li></ul>"]
pub struct IBE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt edge register (1 bit per IO).<ul><li>0: single edge.</li><li>1: both edges</li></ul>"]
pub mod ibe;
#[doc = "Interrupt event register (1 bit per IO).<ul><li>0: falling edge or low level.</li><li>1: rising edge or high level</li></ul>"]
pub struct IEV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt event register (1 bit per IO).<ul><li>0: falling edge or low level.</li><li>1: rising edge or high level</li></ul>"]
pub mod iev;
#[doc = "Interrupt mask register (1 bit per IO).<ul><li>0: masked.</li><li>1: not masked</li></ul>"]
pub struct IE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt mask register (1 bit per IO).<ul><li>0: masked.</li><li>1: not masked</li></ul>"]
pub mod ie;
#[doc = "Masked interrupt status register (1 bit per IO)"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked interrupt status register (1 bit per IO)"]
pub mod mis;
#[doc = "Interrupt clear register (1 bit per IO).<ul><li>0: no effect.</li><li>1: clear interrupt</li></ul>"]
pub struct IC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt clear register (1 bit per IO).<ul><li>0: no effect.</li><li>1: clear interrupt</li></ul>"]
pub mod ic;
#[doc = "Select mode for IO0 to IO7.<ul><li>000b: GPIO mode.</li><li>001b: Serial1 mode.</li><li>100b: Serial0 mode.</li><li>101b: Microphone/ADC mode.</li></ul>"]
pub struct MODE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select mode for IO0 to IO7.<ul><li>000b: GPIO mode.</li><li>001b: Serial1 mode.</li><li>100b: Serial0 mode.</li><li>101b: Microphone/ADC mode.</li></ul>"]
pub mod mode0;
#[doc = "Select mode for IO8 to IO14.<ul><li>000b: GPIO mode.</li><li>001b: Serial1 mode.</li><li>100b: Serial0 mode.</li><li>101b: Microphone/ADC mode.</li></ul>"]
pub struct MODE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select mode for IO8 to IO14.<ul><li>000b: GPIO mode.</li><li>001b: Serial1 mode.</li><li>100b: Serial0 mode.</li><li>101b: Microphone/ADC mode.</li></ul>"]
pub mod mode1;
#[doc = "Set some bits of DATA when in GPIO mode without affecting the others (1 bit per IO).<ul><li>0: no effect.</li><li>1: set at 1 the bit</li></ul>"]
pub struct DATS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set some bits of DATA when in GPIO mode without affecting the others (1 bit per IO).<ul><li>0: no effect.</li><li>1: set at 1 the bit</li></ul>"]
pub mod dats;
#[doc = "Clear some bits of DATA when in GPIO mode without affecting the others (1 bit per IO).<ul><li>0: no effect.</li><li>1: clear at 0 the bit</li></ul>"]
pub struct DATC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear some bits of DATA when in GPIO mode without affecting the others (1 bit per IO).<ul><li>0: no effect.</li><li>1: clear at 0 the bit</li></ul>"]
pub mod datc;
#[doc = "Select the IO to be used as capture input for the MFTX timers"]
pub struct MFTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select the IO to be used as capture input for the MFTX timers"]
pub mod mftx;
