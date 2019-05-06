#![doc = "Peripheral access API for BLUENRG1 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn GPIO();
    fn NVM();
    fn UART();
    fn SPI();
    fn BLUE_CTRL();
    fn WDG();
    fn ADC();
    fn I2C2();
    fn I2C1();
    fn MFT1A();
    fn MFT1B();
    fn MFT2A();
    fn MFT2B();
    fn RTC();
    fn PKA();
    fn DMA();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 24] = [
    Vector { _handler: GPIO },
    Vector { _handler: NVM },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: UART },
    Vector { _handler: SPI },
    Vector {
        _handler: BLUE_CTRL,
    },
    Vector { _handler: WDG },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC },
    Vector { _handler: I2C2 },
    Vector { _handler: I2C1 },
    Vector { _reserved: 0 },
    Vector { _handler: MFT1A },
    Vector { _handler: MFT1B },
    Vector { _handler: MFT2A },
    Vector { _handler: MFT2B },
    Vector { _handler: RTC },
    Vector { _handler: PKA },
    Vector { _handler: DMA },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - GPIO bus interrupt"]
    GPIO,
    #[doc = "1 - Non-volatile memory (Flash) controller interrupt"]
    NVM,
    #[doc = "4 - UART interrupt"]
    UART,
    #[doc = "5 - SPI interrupt"]
    SPI,
    #[doc = "6 - BLUE controller interrupt"]
    BLUE_CTRL,
    #[doc = "7 - Watchdog interrupt"]
    WDG,
    #[doc = "13 - ADC interrupt"]
    ADC,
    #[doc = "14 - I2C 2 interrupt"]
    I2C2,
    #[doc = "15 - I2C 1 interrupt"]
    I2C1,
    #[doc = "17 - Multi functional timer MFT1 interrupt A"]
    MFT1A,
    #[doc = "18 - Multi functional timer MFT1 interrupt B"]
    MFT1B,
    #[doc = "19 - Multi functional timer MFT2 interrupt A"]
    MFT2A,
    #[doc = "20 - Multi functional timer MFT2 interrupt B"]
    MFT2B,
    #[doc = "21 - RTC interrupt"]
    RTC,
    #[doc = "22 - PKA interrupt"]
    PKA,
    #[doc = "23 - DMA interrupt"]
    DMA,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::GPIO => 0,
            Interrupt::NVM => 1,
            Interrupt::UART => 4,
            Interrupt::SPI => 5,
            Interrupt::BLUE_CTRL => 6,
            Interrupt::WDG => 7,
            Interrupt::ADC => 13,
            Interrupt::I2C2 => 14,
            Interrupt::I2C1 => 15,
            Interrupt::MFT1A => 17,
            Interrupt::MFT1B => 18,
            Interrupt::MFT2A => 19,
            Interrupt::MFT2B => 20,
            Interrupt::RTC => 21,
            Interrupt::PKA => 22,
            Interrupt::DMA => 23,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "GPIO Controller"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "GPIO Controller"]
pub mod gpio;
#[doc = "Flash Controller"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash::RegisterBlock {
        1074790400 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash Controller"]
pub mod flash;
#[doc = "System controller"]
pub struct SYSTEM_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEM_CTRL {}
impl SYSTEM_CTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const system_ctrl::RegisterBlock {
        1075838976 as *const _
    }
}
impl Deref for SYSTEM_CTRL {
    type Target = system_ctrl::RegisterBlock;
    fn deref(&self) -> &system_ctrl::RegisterBlock {
        unsafe { &*SYSTEM_CTRL::ptr() }
    }
}
#[doc = "System controller"]
pub mod system_ctrl;
#[doc = "UART"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart::RegisterBlock {
        1076887552 as *const _
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    fn deref(&self) -> &uart::RegisterBlock {
        unsafe { &*UART::ptr() }
    }
}
#[doc = "UART"]
pub mod uart;
#[doc = "Serial peripheral interface"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi::RegisterBlock {
        1077936128 as *const _
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    fn deref(&self) -> &spi::RegisterBlock {
        unsafe { &*SPI::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub mod spi;
#[doc = "Watchdog"]
pub struct WDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDG {}
impl WDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdg::RegisterBlock {
        1081081856 as *const _
    }
}
impl Deref for WDG {
    type Target = wdg::RegisterBlock;
    fn deref(&self) -> &wdg::RegisterBlock {
        unsafe { &*WDG::ptr() }
    }
}
#[doc = "Watchdog"]
pub mod wdg;
#[doc = "ADC"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1082130432 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "ADC"]
pub mod adc;
#[doc = "Clock Gen SOC"]
pub struct CKGEN_SOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CKGEN_SOC {}
impl CKGEN_SOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ckgen_soc::RegisterBlock {
        1083179008 as *const _
    }
}
impl Deref for CKGEN_SOC {
    type Target = ckgen_soc::RegisterBlock;
    fn deref(&self) -> &ckgen_soc::RegisterBlock {
        unsafe { &*CKGEN_SOC::ptr() }
    }
}
#[doc = "Clock Gen SOC"]
pub mod ckgen_soc;
#[doc = "I2C2"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c2::RegisterBlock {
        1084227584 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c2::RegisterBlock;
    fn deref(&self) -> &i2c2::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2C2"]
pub mod i2c2;
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c2::RegisterBlock {
        1085276160 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c2::RegisterBlock;
    fn deref(&self) -> &i2c2::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "AHB up/down converter converter"]
pub struct AHBUPCONV {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AHBUPCONV {}
impl AHBUPCONV {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ahbupconv::RegisterBlock {
        1086324736 as *const _
    }
}
impl Deref for AHBUPCONV {
    type Target = ahbupconv::RegisterBlock;
    fn deref(&self) -> &ahbupconv::RegisterBlock {
        unsafe { &*AHBUPCONV::ptr() }
    }
}
#[doc = "AHB up/down converter converter"]
pub mod ahbupconv;
#[doc = "MFT1"]
pub struct MFT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MFT1 {}
impl MFT1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mft1::RegisterBlock {
        1087373312 as *const _
    }
}
impl Deref for MFT1 {
    type Target = mft1::RegisterBlock;
    fn deref(&self) -> &mft1::RegisterBlock {
        unsafe { &*MFT1::ptr() }
    }
}
#[doc = "MFT1"]
pub mod mft1;
#[doc = "MFT2"]
pub struct MFT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MFT2 {}
impl MFT2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mft1::RegisterBlock {
        1088421888 as *const _
    }
}
impl Deref for MFT2 {
    type Target = mft1::RegisterBlock;
    fn deref(&self) -> &mft1::RegisterBlock {
        unsafe { &*MFT2::ptr() }
    }
}
#[doc = "Real-Time Counter"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1089470464 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-Time Counter"]
pub mod rtc;
#[doc = "BLUE Controller"]
pub struct BLUE_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BLUE_CTRL {}
impl BLUE_CTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const blue_ctrl::RegisterBlock {
        1207959552 as *const _
    }
}
impl Deref for BLUE_CTRL {
    type Target = blue_ctrl::RegisterBlock;
    fn deref(&self) -> &blue_ctrl::RegisterBlock {
        unsafe { &*BLUE_CTRL::ptr() }
    }
}
#[doc = "BLUE Controller"]
pub mod blue_ctrl;
#[doc = "Clock Gen BLE"]
pub struct CKGEN_BLE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CKGEN_BLE {}
impl CKGEN_BLE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ckgen_ble::RegisterBlock {
        1209008128 as *const _
    }
}
impl Deref for CKGEN_BLE {
    type Target = ckgen_ble::RegisterBlock;
    fn deref(&self) -> &ckgen_ble::RegisterBlock {
        unsafe { &*CKGEN_BLE::ptr() }
    }
}
#[doc = "Clock Gen BLE"]
pub mod ckgen_ble;
#[doc = "DMA"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma::RegisterBlock {
        2684354560 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    fn deref(&self) -> &dma::RegisterBlock {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA"]
pub mod dma;
#[doc = "DMA channel"]
pub struct DMA_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA_CH0 {}
impl DMA_CH0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma_ch0::RegisterBlock {
        2684354568 as *const _
    }
}
impl Deref for DMA_CH0 {
    type Target = dma_ch0::RegisterBlock;
    fn deref(&self) -> &dma_ch0::RegisterBlock {
        unsafe { &*DMA_CH0::ptr() }
    }
}
#[doc = "DMA channel"]
pub mod dma_ch0;
#[doc = "DMA_CH1"]
pub struct DMA_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA_CH1 {}
impl DMA_CH1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma_ch0::RegisterBlock {
        2684354588 as *const _
    }
}
impl Deref for DMA_CH1 {
    type Target = dma_ch0::RegisterBlock;
    fn deref(&self) -> &dma_ch0::RegisterBlock {
        unsafe { &*DMA_CH1::ptr() }
    }
}
#[doc = "DMA_CH2"]
pub struct DMA_CH2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA_CH2 {}
impl DMA_CH2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma_ch0::RegisterBlock {
        2684354608 as *const _
    }
}
impl Deref for DMA_CH2 {
    type Target = dma_ch0::RegisterBlock;
    fn deref(&self) -> &dma_ch0::RegisterBlock {
        unsafe { &*DMA_CH2::ptr() }
    }
}
#[doc = "DMA_CH3"]
pub struct DMA_CH3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA_CH3 {}
impl DMA_CH3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma_ch0::RegisterBlock {
        2684354628 as *const _
    }
}
impl Deref for DMA_CH3 {
    type Target = dma_ch0::RegisterBlock;
    fn deref(&self) -> &dma_ch0::RegisterBlock {
        unsafe { &*DMA_CH3::ptr() }
    }
}
#[doc = "DMA_CH4"]
pub struct DMA_CH4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA_CH4 {}
impl DMA_CH4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma_ch0::RegisterBlock {
        2684354648 as *const _
    }
}
impl Deref for DMA_CH4 {
    type Target = dma_ch0::RegisterBlock;
    fn deref(&self) -> &dma_ch0::RegisterBlock {
        unsafe { &*DMA_CH4::ptr() }
    }
}
#[doc = "DMA_CH5"]
pub struct DMA_CH5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA_CH5 {}
impl DMA_CH5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma_ch0::RegisterBlock {
        2684354668 as *const _
    }
}
impl Deref for DMA_CH5 {
    type Target = dma_ch0::RegisterBlock;
    fn deref(&self) -> &dma_ch0::RegisterBlock {
        unsafe { &*DMA_CH5::ptr() }
    }
}
#[doc = "DMA_CH6"]
pub struct DMA_CH6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA_CH6 {}
impl DMA_CH6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma_ch0::RegisterBlock {
        2684354688 as *const _
    }
}
impl Deref for DMA_CH6 {
    type Target = dma_ch0::RegisterBlock;
    fn deref(&self) -> &dma_ch0::RegisterBlock {
        unsafe { &*DMA_CH6::ptr() }
    }
}
#[doc = "DMA_CH7"]
pub struct DMA_CH7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA_CH7 {}
impl DMA_CH7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma_ch0::RegisterBlock {
        2684354708 as *const _
    }
}
impl Deref for DMA_CH7 {
    type Target = dma_ch0::RegisterBlock;
    fn deref(&self) -> &dma_ch0::RegisterBlock {
        unsafe { &*DMA_CH7::ptr() }
    }
}
#[doc = "RNG"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rng::RegisterBlock {
        2952790016 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    fn deref(&self) -> &rng::RegisterBlock {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "RNG"]
pub mod rng;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "SYSTEM_CTRL"]
    pub SYSTEM_CTRL: SYSTEM_CTRL,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "WDG"]
    pub WDG: WDG,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "CKGEN_SOC"]
    pub CKGEN_SOC: CKGEN_SOC,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "AHBUPCONV"]
    pub AHBUPCONV: AHBUPCONV,
    #[doc = "MFT1"]
    pub MFT1: MFT1,
    #[doc = "MFT2"]
    pub MFT2: MFT2,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "BLUE_CTRL"]
    pub BLUE_CTRL: BLUE_CTRL,
    #[doc = "CKGEN_BLE"]
    pub CKGEN_BLE: CKGEN_BLE,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "DMA_CH0"]
    pub DMA_CH0: DMA_CH0,
    #[doc = "DMA_CH1"]
    pub DMA_CH1: DMA_CH1,
    #[doc = "DMA_CH2"]
    pub DMA_CH2: DMA_CH2,
    #[doc = "DMA_CH3"]
    pub DMA_CH3: DMA_CH3,
    #[doc = "DMA_CH4"]
    pub DMA_CH4: DMA_CH4,
    #[doc = "DMA_CH5"]
    pub DMA_CH5: DMA_CH5,
    #[doc = "DMA_CH6"]
    pub DMA_CH6: DMA_CH6,
    #[doc = "DMA_CH7"]
    pub DMA_CH7: DMA_CH7,
    #[doc = "RNG"]
    pub RNG: RNG,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            GPIO: GPIO {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            SYSTEM_CTRL: SYSTEM_CTRL {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            WDG: WDG {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            CKGEN_SOC: CKGEN_SOC {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            AHBUPCONV: AHBUPCONV {
                _marker: PhantomData,
            },
            MFT1: MFT1 {
                _marker: PhantomData,
            },
            MFT2: MFT2 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            BLUE_CTRL: BLUE_CTRL {
                _marker: PhantomData,
            },
            CKGEN_BLE: CKGEN_BLE {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            DMA_CH0: DMA_CH0 {
                _marker: PhantomData,
            },
            DMA_CH1: DMA_CH1 {
                _marker: PhantomData,
            },
            DMA_CH2: DMA_CH2 {
                _marker: PhantomData,
            },
            DMA_CH3: DMA_CH3 {
                _marker: PhantomData,
            },
            DMA_CH4: DMA_CH4 {
                _marker: PhantomData,
            },
            DMA_CH5: DMA_CH5 {
                _marker: PhantomData,
            },
            DMA_CH6: DMA_CH6 {
                _marker: PhantomData,
            },
            DMA_CH7: DMA_CH7 {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
        }
    }
}
