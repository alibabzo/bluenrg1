#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONR {
    #[doc = "ADC analog part disable"]
    OFF,
    #[doc = "ADC analog part enable"]
    ON,
}
impl ONR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ONR::OFF => false,
            ONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONR {
        match value {
            false => ONR::OFF,
            true => ONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == ONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == ONR::ON
    }
}
#[doc = "Possible values of the field `CALEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALENR {
    #[doc = "ADC automatic calibration disable"]
    CAL_OFF,
    #[doc = "ADC automatic calibration enable"]
    CAL_ON,
}
impl CALENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CALENR::CAL_OFF => false,
            CALENR::CAL_ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CALENR {
        match value {
            false => CALENR::CAL_OFF,
            true => CALENR::CAL_ON,
        }
    }
    #[doc = "Checks if the value of the field is `CAL_OFF`"]
    #[inline]
    pub fn is_cal_off(&self) -> bool {
        *self == CALENR::CAL_OFF
    }
    #[doc = "Checks if the value of the field is `CAL_ON`"]
    #[inline]
    pub fn is_cal_on(&self) -> bool {
        *self == CALENR::CAL_ON
    }
}
#[doc = "Possible values of the field `SWSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSTARTR {
    #[doc = "Starts the ADC conversion phase when set."]
    START,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SWSTARTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SWSTARTR::START => true,
            SWSTARTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWSTARTR {
        match value {
            true => SWSTARTR::START,
            i => SWSTARTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == SWSTARTR::START
    }
}
#[doc = "Possible values of the field `RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETR {
    #[doc = "Reset all the registers content"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RESETR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RESETR::RESET => true,
            RESETR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESETR {
        match value {
            true => RESETR::RESET,
            i => RESETR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == RESETR::RESET
    }
}
#[doc = "Possible values of the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPR {
    #[doc = "Stop the continuous mode conversion"]
    STOP,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl STOPR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            STOPR::STOP => true,
            STOPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPR {
        match value {
            true => STOPR::STOP,
            i => STOPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == STOPR::STOP
    }
}
#[doc = r" Value of the field"]
pub struct ENAB_COMPR {
    bits: bool,
}
impl ENAB_COMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `RSTCALEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTCALENR {
    #[doc = "Reset the ADCCALEN bit. Disable the automatic calibration when it is enabled"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RSTCALENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RSTCALENR::RESET => true,
            RSTCALENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTCALENR {
        match value {
            true => RSTCALENR::RESET,
            i => RSTCALENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == RSTCALENR::RESET
    }
}
#[doc = "Possible values of the field `AUTO_OFFSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_OFFSETR {
    #[doc = "ADC automatic calibration disable"]
    CAL_OFF,
    #[doc = "ADC automatic calibration enable"]
    CAL_ON,
}
impl AUTO_OFFSETR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUTO_OFFSETR::CAL_OFF => false,
            AUTO_OFFSETR::CAL_ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTO_OFFSETR {
        match value {
            false => AUTO_OFFSETR::CAL_OFF,
            true => AUTO_OFFSETR::CAL_ON,
        }
    }
    #[doc = "Checks if the value of the field is `CAL_OFF`"]
    #[inline]
    pub fn is_cal_off(&self) -> bool {
        *self == AUTO_OFFSETR::CAL_OFF
    }
    #[doc = "Checks if the value of the field is `CAL_ON`"]
    #[inline]
    pub fn is_cal_on(&self) -> bool {
        *self == AUTO_OFFSETR::CAL_ON
    }
}
#[doc = "Possible values of the field `MIC_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIC_ONR {
    #[doc = "Filter chain disable"]
    MIC_OFF,
    #[doc = "Filter chain enable"]
    MIC_ON,
}
impl MIC_ONR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MIC_ONR::MIC_OFF => false,
            MIC_ONR::MIC_ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIC_ONR {
        match value {
            false => MIC_ONR::MIC_OFF,
            true => MIC_ONR::MIC_ON,
        }
    }
    #[doc = "Checks if the value of the field is `MIC_OFF`"]
    #[inline]
    pub fn is_mic_off(&self) -> bool {
        *self == MIC_ONR::MIC_OFF
    }
    #[doc = "Checks if the value of the field is `MIC_ON`"]
    #[inline]
    pub fn is_mic_on(&self) -> bool {
        *self == MIC_ONR::MIC_ON
    }
}
#[doc = "Possible values of the field `DMA_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ENR {
    #[doc = "ADC DMA disable"]
    DMA_OFF,
    #[doc = "ADC DMA enable"]
    DMA_ON,
}
impl DMA_ENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DMA_ENR::DMA_OFF => false,
            DMA_ENR::DMA_ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_ENR {
        match value {
            false => DMA_ENR::DMA_OFF,
            true => DMA_ENR::DMA_ON,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_OFF`"]
    #[inline]
    pub fn is_dma_off(&self) -> bool {
        *self == DMA_ENR::DMA_OFF
    }
    #[doc = "Checks if the value of the field is `DMA_ON`"]
    #[inline]
    pub fn is_dma_on(&self) -> bool {
        *self == DMA_ENR::DMA_ON
    }
}
#[doc = "Values that can be written to the field `ON`"]
pub enum ONW {
    #[doc = "ADC analog part disable"]
    OFF,
    #[doc = "ADC analog part enable"]
    ON,
}
impl ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONW::OFF => false,
            ONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONW<'a> {
    w: &'a mut W,
}
impl<'a> _ONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC analog part disable"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(ONW::OFF)
    }
    #[doc = "ADC analog part enable"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(ONW::ON)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CALEN`"]
pub enum CALENW {
    #[doc = "ADC automatic calibration disable"]
    CAL_OFF,
    #[doc = "ADC automatic calibration enable"]
    CAL_ON,
}
impl CALENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CALENW::CAL_OFF => false,
            CALENW::CAL_ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CALENW<'a> {
    w: &'a mut W,
}
impl<'a> _CALENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CALENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC automatic calibration disable"]
    #[inline]
    pub fn cal_off(self) -> &'a mut W {
        self.variant(CALENW::CAL_OFF)
    }
    #[doc = "ADC automatic calibration enable"]
    #[inline]
    pub fn cal_on(self) -> &'a mut W {
        self.variant(CALENW::CAL_ON)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SWSTART`"]
pub enum SWSTARTW {
    #[doc = "Starts the ADC conversion phase when set."]
    START,
}
impl SWSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWSTARTW::START => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWSTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWSTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Starts the ADC conversion phase when set."]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(SWSTARTW::START)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESET`"]
pub enum RESETW {
    #[doc = "Reset all the registers content"]
    RESET,
}
impl RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESETW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset all the registers content"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESETW::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STOP`"]
pub enum STOPW {
    #[doc = "Stop the continuous mode conversion"]
    STOP,
}
impl STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop the continuous mode conversion"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOPW::STOP)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENAB_COMPW<'a> {
    w: &'a mut W,
}
impl<'a> _ENAB_COMPW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSTCALEN`"]
pub enum RSTCALENW {
    #[doc = "Reset the ADCCALEN bit. Disable the automatic calibration when it is enabled"]
    RESET,
}
impl RSTCALENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTCALENW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTCALENW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTCALENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTCALENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the ADCCALEN bit. Disable the automatic calibration when it is enabled"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(RSTCALENW::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUTO_OFFSET`"]
pub enum AUTO_OFFSETW {
    #[doc = "ADC automatic calibration disable"]
    CAL_OFF,
    #[doc = "ADC automatic calibration enable"]
    CAL_ON,
}
impl AUTO_OFFSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTO_OFFSETW::CAL_OFF => false,
            AUTO_OFFSETW::CAL_ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTO_OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTO_OFFSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTO_OFFSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC automatic calibration disable"]
    #[inline]
    pub fn cal_off(self) -> &'a mut W {
        self.variant(AUTO_OFFSETW::CAL_OFF)
    }
    #[doc = "ADC automatic calibration enable"]
    #[inline]
    pub fn cal_on(self) -> &'a mut W {
        self.variant(AUTO_OFFSETW::CAL_ON)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MIC_ON`"]
pub enum MIC_ONW {
    #[doc = "Filter chain disable"]
    MIC_OFF,
    #[doc = "Filter chain enable"]
    MIC_ON,
}
impl MIC_ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MIC_ONW::MIC_OFF => false,
            MIC_ONW::MIC_ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MIC_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _MIC_ONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MIC_ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Filter chain disable"]
    #[inline]
    pub fn mic_off(self) -> &'a mut W {
        self.variant(MIC_ONW::MIC_OFF)
    }
    #[doc = "Filter chain enable"]
    #[inline]
    pub fn mic_on(self) -> &'a mut W {
        self.variant(MIC_ONW::MIC_ON)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA_EN`"]
pub enum DMA_ENW {
    #[doc = "ADC DMA disable"]
    DMA_OFF,
    #[doc = "ADC DMA enable"]
    DMA_ON,
}
impl DMA_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_ENW::DMA_OFF => false,
            DMA_ENW::DMA_ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC DMA disable"]
    #[inline]
    pub fn dma_off(self) -> &'a mut W {
        self.variant(DMA_ENW::DMA_OFF)
    }
    #[doc = "ADC DMA enable"]
    #[inline]
    pub fn dma_on(self) -> &'a mut W {
        self.variant(DMA_ENW::DMA_ON)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Starts ADC analog subsystem. This bit must be set before starting a conversion.<ul><li>0: ADC is OFF.</li><li>1: ADC is ON.</li></ul>"]
    #[inline]
    pub fn on(&self) -> ONR {
        ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Enables the calibration phase when set to 1. This bit is cleared and the calibration is disabled by setting the RSTADCCALEN bit."]
    #[inline]
    pub fn calen(&self) -> CALENR {
        CALENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Starts the ADC conversion phase when set."]
    #[inline]
    pub fn swstart(&self) -> SWSTARTR {
        SWSTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Reset all the ADC APB registers when set."]
    #[inline]
    pub fn reset(&self) -> RESETR {
        RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Permits to stop the continuous conversion.<ul><li>0: continuous conversion is enabled but SWSTART and ADCON bits must be set.</li><li>1: stop the continuous conversion and switch off the ADC.</li></ul>"]
    #[inline]
    pub fn stop(&self) -> STOPR {
        STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Enables the window comparator when set to 1. WDOG flag is ADC_SR register is set if the converted value is between ADCTHRESHOLD_HI and ADCTHRESHOLD_LO value."]
    #[inline]
    pub fn enab_comp(&self) -> ENAB_COMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        ENAB_COMPR { bits }
    }
    #[doc = "Bit 6 - Disable the calibration phase when set to 1. This bit has to be set to disable the calibration each time calibration is enabled."]
    #[inline]
    pub fn rstcalen(&self) -> RSTCALENR {
        RSTCALENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Enables the update of ADC_OFFSET register.<ul><li>0: ADC_OFFSET register is not updated.</li><li>1: ADC_OFFSET register is updated.</li></ul>"]
    #[inline]
    pub fn auto_offset(&self) -> AUTO_OFFSETR {
        AUTO_OFFSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Enables the filter chain for voice when set to 1.<ul><li>0: Filter chain is disabled.</li><li>1: Filter chain is enabled.</li></ul>"]
    #[inline]
    pub fn mic_on(&self) -> MIC_ONR {
        MIC_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - Enables the DMA.<ul><li>0: DMA is disabled.</li><li>1: DMA is enabled.</li></ul>"]
    #[inline]
    pub fn dma_en(&self) -> DMA_ENR {
        DMA_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Starts ADC analog subsystem. This bit must be set before starting a conversion.<ul><li>0: ADC is OFF.</li><li>1: ADC is ON.</li></ul>"]
    #[inline]
    pub fn on(&mut self) -> _ONW {
        _ONW { w: self }
    }
    #[doc = "Bit 1 - Enables the calibration phase when set to 1. This bit is cleared and the calibration is disabled by setting the RSTADCCALEN bit."]
    #[inline]
    pub fn calen(&mut self) -> _CALENW {
        _CALENW { w: self }
    }
    #[doc = "Bit 2 - Starts the ADC conversion phase when set."]
    #[inline]
    pub fn swstart(&mut self) -> _SWSTARTW {
        _SWSTARTW { w: self }
    }
    #[doc = "Bit 3 - Reset all the ADC APB registers when set."]
    #[inline]
    pub fn reset(&mut self) -> _RESETW {
        _RESETW { w: self }
    }
    #[doc = "Bit 4 - Permits to stop the continuous conversion.<ul><li>0: continuous conversion is enabled but SWSTART and ADCON bits must be set.</li><li>1: stop the continuous conversion and switch off the ADC.</li></ul>"]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bit 5 - Enables the window comparator when set to 1. WDOG flag is ADC_SR register is set if the converted value is between ADCTHRESHOLD_HI and ADCTHRESHOLD_LO value."]
    #[inline]
    pub fn enab_comp(&mut self) -> _ENAB_COMPW {
        _ENAB_COMPW { w: self }
    }
    #[doc = "Bit 6 - Disable the calibration phase when set to 1. This bit has to be set to disable the calibration each time calibration is enabled."]
    #[inline]
    pub fn rstcalen(&mut self) -> _RSTCALENW {
        _RSTCALENW { w: self }
    }
    #[doc = "Bit 7 - Enables the update of ADC_OFFSET register.<ul><li>0: ADC_OFFSET register is not updated.</li><li>1: ADC_OFFSET register is updated.</li></ul>"]
    #[inline]
    pub fn auto_offset(&mut self) -> _AUTO_OFFSETW {
        _AUTO_OFFSETW { w: self }
    }
    #[doc = "Bit 8 - Enables the filter chain for voice when set to 1.<ul><li>0: Filter chain is disabled.</li><li>1: Filter chain is enabled.</li></ul>"]
    #[inline]
    pub fn mic_on(&mut self) -> _MIC_ONW {
        _MIC_ONW { w: self }
    }
    #[doc = "Bit 9 - Enables the DMA.<ul><li>0: DMA is disabled.</li><li>1: DMA is enabled.</li></ul>"]
    #[inline]
    pub fn dma_en(&mut self) -> _DMA_ENW {
        _DMA_ENW { w: self }
    }
}
