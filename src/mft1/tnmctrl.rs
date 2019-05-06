#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::TNMCTRL {
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
#[doc = "Possible values of the field `TNMDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNMDSELR {
    #[doc = "PWM mode and system timer or pulse train"]
    PWM,
    #[doc = "Dual-input capture and system timer"]
    DUAL_INPUT_CAPTURE,
    #[doc = "Dual independent timer/counter"]
    DUAL_INDEPENDENT,
    #[doc = "Single timer and single input capture"]
    SINGLE,
}
impl TNMDSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TNMDSELR::PWM => 0,
            TNMDSELR::DUAL_INPUT_CAPTURE => 1,
            TNMDSELR::DUAL_INDEPENDENT => 2,
            TNMDSELR::SINGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TNMDSELR {
        match value {
            0 => TNMDSELR::PWM,
            1 => TNMDSELR::DUAL_INPUT_CAPTURE,
            2 => TNMDSELR::DUAL_INDEPENDENT,
            3 => TNMDSELR::SINGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline]
    pub fn is_pwm(&self) -> bool {
        *self == TNMDSELR::PWM
    }
    #[doc = "Checks if the value of the field is `DUAL_INPUT_CAPTURE`"]
    #[inline]
    pub fn is_dual_input_capture(&self) -> bool {
        *self == TNMDSELR::DUAL_INPUT_CAPTURE
    }
    #[doc = "Checks if the value of the field is `DUAL_INDEPENDENT`"]
    #[inline]
    pub fn is_dual_independent(&self) -> bool {
        *self == TNMDSELR::DUAL_INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == TNMDSELR::SINGLE
    }
}
#[doc = "Possible values of the field `TNAEDG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNAEDGR {
    #[doc = "Input is sensitive to falling edges"]
    FALLING_EDGE,
    #[doc = "Input is sensitive to rising edges"]
    RISING_EDGE,
}
impl TNAEDGR {
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
            TNAEDGR::FALLING_EDGE => false,
            TNAEDGR::RISING_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TNAEDGR {
        match value {
            false => TNAEDGR::FALLING_EDGE,
            true => TNAEDGR::RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == TNAEDGR::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == TNAEDGR::RISING_EDGE
    }
}
#[doc = r" Value of the field"]
pub struct TNBEDGR {
    bits: bool,
}
impl TNBEDGR {
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
#[doc = "Possible values of the field `TNAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNAENR {
    #[doc = "TnA in disable"]
    TNA_IN_DISABLE,
    #[doc = "TnA in enable"]
    TNA_IN_ENABLE,
}
impl TNAENR {
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
            TNAENR::TNA_IN_DISABLE => false,
            TNAENR::TNA_IN_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TNAENR {
        match value {
            false => TNAENR::TNA_IN_DISABLE,
            true => TNAENR::TNA_IN_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `TNA_IN_DISABLE`"]
    #[inline]
    pub fn is_tna_in_disable(&self) -> bool {
        *self == TNAENR::TNA_IN_DISABLE
    }
    #[doc = "Checks if the value of the field is `TNA_IN_ENABLE`"]
    #[inline]
    pub fn is_tna_in_enable(&self) -> bool {
        *self == TNAENR::TNA_IN_ENABLE
    }
}
#[doc = "Possible values of the field `TNBEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNBENR {
    #[doc = "TNB in disable"]
    TNB_IN_DISABLE,
    #[doc = "TNB in enable"]
    TNB_IN_ENABLE,
}
impl TNBENR {
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
            TNBENR::TNB_IN_DISABLE => false,
            TNBENR::TNB_IN_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TNBENR {
        match value {
            false => TNBENR::TNB_IN_DISABLE,
            true => TNBENR::TNB_IN_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `TNB_IN_DISABLE`"]
    #[inline]
    pub fn is_tnb_in_disable(&self) -> bool {
        *self == TNBENR::TNB_IN_DISABLE
    }
    #[doc = "Checks if the value of the field is `TNB_IN_ENABLE`"]
    #[inline]
    pub fn is_tnb_in_enable(&self) -> bool {
        *self == TNBENR::TNB_IN_ENABLE
    }
}
#[doc = "Possible values of the field `TNAOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNAOUTR {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl TNAOUTR {
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
            TNAOUTR::LOW => false,
            TNAOUTR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TNAOUTR {
        match value {
            false => TNAOUTR::LOW,
            true => TNAOUTR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == TNAOUTR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == TNAOUTR::HIGH
    }
}
#[doc = "Possible values of the field `TNEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNENR {
    #[doc = "MFTX disable"]
    MFTX_DISABLE,
    #[doc = "MFTX enable"]
    MFTX_ENABLE,
}
impl TNENR {
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
            TNENR::MFTX_DISABLE => false,
            TNENR::MFTX_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TNENR {
        match value {
            false => TNENR::MFTX_DISABLE,
            true => TNENR::MFTX_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `MFTX_DISABLE`"]
    #[inline]
    pub fn is_mftx_disable(&self) -> bool {
        *self == TNENR::MFTX_DISABLE
    }
    #[doc = "Checks if the value of the field is `MFTX_ENABLE`"]
    #[inline]
    pub fn is_mftx_enable(&self) -> bool {
        *self == TNENR::MFTX_ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct TNPTENR {
    bits: bool,
}
impl TNPTENR {
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
#[doc = r" Value of the field"]
pub struct TNPTSER {
    bits: bool,
}
impl TNPTSER {
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
#[doc = r" Value of the field"]
pub struct TNPTETR {
    bits: bool,
}
impl TNPTETR {
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
#[doc = "Values that can be written to the field `TNMDSEL`"]
pub enum TNMDSELW {
    #[doc = "PWM mode and system timer or pulse train"]
    PWM,
    #[doc = "Dual-input capture and system timer"]
    DUAL_INPUT_CAPTURE,
    #[doc = "Dual independent timer/counter"]
    DUAL_INDEPENDENT,
    #[doc = "Single timer and single input capture"]
    SINGLE,
}
impl TNMDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TNMDSELW::PWM => 0,
            TNMDSELW::DUAL_INPUT_CAPTURE => 1,
            TNMDSELW::DUAL_INDEPENDENT => 2,
            TNMDSELW::SINGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNMDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TNMDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNMDSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PWM mode and system timer or pulse train"]
    #[inline]
    pub fn pwm(self) -> &'a mut W {
        self.variant(TNMDSELW::PWM)
    }
    #[doc = "Dual-input capture and system timer"]
    #[inline]
    pub fn dual_input_capture(self) -> &'a mut W {
        self.variant(TNMDSELW::DUAL_INPUT_CAPTURE)
    }
    #[doc = "Dual independent timer/counter"]
    #[inline]
    pub fn dual_independent(self) -> &'a mut W {
        self.variant(TNMDSELW::DUAL_INDEPENDENT)
    }
    #[doc = "Single timer and single input capture"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(TNMDSELW::SINGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TNAEDG`"]
pub enum TNAEDGW {
    #[doc = "Input is sensitive to falling edges"]
    FALLING_EDGE,
    #[doc = "Input is sensitive to rising edges"]
    RISING_EDGE,
}
impl TNAEDGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TNAEDGW::FALLING_EDGE => false,
            TNAEDGW::RISING_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNAEDGW<'a> {
    w: &'a mut W,
}
impl<'a> _TNAEDGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNAEDGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is sensitive to falling edges"]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TNAEDGW::FALLING_EDGE)
    }
    #[doc = "Input is sensitive to rising edges"]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TNAEDGW::RISING_EDGE)
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
#[doc = r" Proxy"]
pub struct _TNBEDGW<'a> {
    w: &'a mut W,
}
impl<'a> _TNBEDGW<'a> {
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
#[doc = "Values that can be written to the field `TNAEN`"]
pub enum TNAENW {
    #[doc = "TnA in disable"]
    TNA_IN_DISABLE,
    #[doc = "TnA in enable"]
    TNA_IN_ENABLE,
}
impl TNAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TNAENW::TNA_IN_DISABLE => false,
            TNAENW::TNA_IN_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TNAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TnA in disable"]
    #[inline]
    pub fn tna_in_disable(self) -> &'a mut W {
        self.variant(TNAENW::TNA_IN_DISABLE)
    }
    #[doc = "TnA in enable"]
    #[inline]
    pub fn tna_in_enable(self) -> &'a mut W {
        self.variant(TNAENW::TNA_IN_ENABLE)
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
#[doc = "Values that can be written to the field `TNBEN`"]
pub enum TNBENW {
    #[doc = "TNB in disable"]
    TNB_IN_DISABLE,
    #[doc = "TNB in enable"]
    TNB_IN_ENABLE,
}
impl TNBENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TNBENW::TNB_IN_DISABLE => false,
            TNBENW::TNB_IN_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNBENW<'a> {
    w: &'a mut W,
}
impl<'a> _TNBENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNBENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TNB in disable"]
    #[inline]
    pub fn tnb_in_disable(self) -> &'a mut W {
        self.variant(TNBENW::TNB_IN_DISABLE)
    }
    #[doc = "TNB in enable"]
    #[inline]
    pub fn tnb_in_enable(self) -> &'a mut W {
        self.variant(TNBENW::TNB_IN_ENABLE)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TNAOUT`"]
pub enum TNAOUTW {
    #[doc = "Pin is low"]
    LOW,
    #[doc = "Pin is high"]
    HIGH,
}
impl TNAOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TNAOUTW::LOW => false,
            TNAOUTW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNAOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TNAOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNAOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(TNAOUTW::LOW)
    }
    #[doc = "Pin is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(TNAOUTW::HIGH)
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
#[doc = "Values that can be written to the field `TNEN`"]
pub enum TNENW {
    #[doc = "MFTX disable"]
    MFTX_DISABLE,
    #[doc = "MFTX enable"]
    MFTX_ENABLE,
}
impl TNENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TNENW::MFTX_DISABLE => false,
            TNENW::MFTX_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNENW<'a> {
    w: &'a mut W,
}
impl<'a> _TNENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MFTX disable"]
    #[inline]
    pub fn mftx_disable(self) -> &'a mut W {
        self.variant(TNENW::MFTX_DISABLE)
    }
    #[doc = "MFTX enable"]
    #[inline]
    pub fn mftx_enable(self) -> &'a mut W {
        self.variant(TNENW::MFTX_ENABLE)
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
#[doc = r" Proxy"]
pub struct _TNPTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TNPTENW<'a> {
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
#[doc = r" Proxy"]
pub struct _TNPTSEW<'a> {
    w: &'a mut W,
}
impl<'a> _TNPTSEW<'a> {
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
#[doc = r" Proxy"]
pub struct _TNPTETW<'a> {
    w: &'a mut W,
}
impl<'a> _TNPTETW<'a> {
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
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:1 - MFTX mode select:<ul><li>00b: Mode 1 or 1a: PWM mode and system timer or pulse train.</li><li>01b: Mode 2: Dual-input capture and system timer.</li><li>10b: Mode 3: Dual independent timer/counter.</li><li>11b: Mode 4: Single timer and single input capture.</li></ul>"]
    #[inline]
    pub fn tnmdsel(&self) -> TNMDSELR {
        TNMDSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 2 - TnA edge polarity:<ul><li>0: Input is sensitive to falling edges.</li><li>1: Input is sensitive to rising edges.</li></ul>"]
    #[inline]
    pub fn tnaedg(&self) -> TNAEDGR {
        TNAEDGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - TnB edge polarity:<ul><li>0: Input is sensitive to falling edges.</li><li>1: Input is sensitive to rising edges.</li></ul>"]
    #[inline]
    pub fn tnbedg(&self) -> TNBEDGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TNBEDGR { bits }
    }
    #[doc = "Bit 4 - TnA enable:<ul><li>0: TnA in disable.</li><li>1: TnA in enable.</li></ul>"]
    #[inline]
    pub fn tnaen(&self) -> TNAENR {
        TNAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - TnB enable:<ul><li>0: TnB in disable.</li><li>1: TnB in enable.</li></ul>"]
    #[inline]
    pub fn tnben(&self) -> TNBENR {
        TNBENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - TnA output data:<ul><li>0: Pin is low.</li><li>1: Pin is high.</li></ul>"]
    #[inline]
    pub fn tnaout(&self) -> TNAOUTR {
        TNAOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - MFTX enable:<ul><li>0: MFTX disable.</li><li>1: MFTX enable.</li></ul>"]
    #[inline]
    pub fn tnen(&self) -> TNENR {
        TNENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Tn pulse-train mode enable:<ul><li>0: Mode 1a not selected.</li><li>1: Mode 1a selected (if TnMDSEL = 00).</li></ul>"]
    #[inline]
    pub fn tnpten(&self) -> TNPTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TNPTENR { bits }
    }
    #[doc = "Bit 9 - Tn pulse-train sofware trigger enable:<ul><li>0: No effect.</li><li>1: Pulse-train generation trigger (in mode 1a)</li></ul>"]
    #[inline]
    pub fn tnptse(&self) -> TNPTSER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TNPTSER { bits }
    }
    #[doc = "Bit 10 - Tn pulse-train event trigger:<ul><li>0: No pulse-train event trigger occurred.</li><li>1: Pulse-train event trigger occurred (in mode 1a).</li></ul>"]
    #[inline]
    pub fn tnptet(&self) -> TNPTETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TNPTETR { bits }
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
    #[doc = "Bits 0:1 - MFTX mode select:<ul><li>00b: Mode 1 or 1a: PWM mode and system timer or pulse train.</li><li>01b: Mode 2: Dual-input capture and system timer.</li><li>10b: Mode 3: Dual independent timer/counter.</li><li>11b: Mode 4: Single timer and single input capture.</li></ul>"]
    #[inline]
    pub fn tnmdsel(&mut self) -> _TNMDSELW {
        _TNMDSELW { w: self }
    }
    #[doc = "Bit 2 - TnA edge polarity:<ul><li>0: Input is sensitive to falling edges.</li><li>1: Input is sensitive to rising edges.</li></ul>"]
    #[inline]
    pub fn tnaedg(&mut self) -> _TNAEDGW {
        _TNAEDGW { w: self }
    }
    #[doc = "Bit 3 - TnB edge polarity:<ul><li>0: Input is sensitive to falling edges.</li><li>1: Input is sensitive to rising edges.</li></ul>"]
    #[inline]
    pub fn tnbedg(&mut self) -> _TNBEDGW {
        _TNBEDGW { w: self }
    }
    #[doc = "Bit 4 - TnA enable:<ul><li>0: TnA in disable.</li><li>1: TnA in enable.</li></ul>"]
    #[inline]
    pub fn tnaen(&mut self) -> _TNAENW {
        _TNAENW { w: self }
    }
    #[doc = "Bit 5 - TnB enable:<ul><li>0: TnB in disable.</li><li>1: TnB in enable.</li></ul>"]
    #[inline]
    pub fn tnben(&mut self) -> _TNBENW {
        _TNBENW { w: self }
    }
    #[doc = "Bit 6 - TnA output data:<ul><li>0: Pin is low.</li><li>1: Pin is high.</li></ul>"]
    #[inline]
    pub fn tnaout(&mut self) -> _TNAOUTW {
        _TNAOUTW { w: self }
    }
    #[doc = "Bit 7 - MFTX enable:<ul><li>0: MFTX disable.</li><li>1: MFTX enable.</li></ul>"]
    #[inline]
    pub fn tnen(&mut self) -> _TNENW {
        _TNENW { w: self }
    }
    #[doc = "Bit 8 - Tn pulse-train mode enable:<ul><li>0: Mode 1a not selected.</li><li>1: Mode 1a selected (if TnMDSEL = 00).</li></ul>"]
    #[inline]
    pub fn tnpten(&mut self) -> _TNPTENW {
        _TNPTENW { w: self }
    }
    #[doc = "Bit 9 - Tn pulse-train sofware trigger enable:<ul><li>0: No effect.</li><li>1: Pulse-train generation trigger (in mode 1a)</li></ul>"]
    #[inline]
    pub fn tnptse(&mut self) -> _TNPTSEW {
        _TNPTSEW { w: self }
    }
    #[doc = "Bit 10 - Tn pulse-train event trigger:<ul><li>0: No pulse-train event trigger occurred.</li><li>1: Pulse-train event trigger occurred (in mode 1a).</li></ul>"]
    #[inline]
    pub fn tnptet(&mut self) -> _TNPTETW {
        _TNPTETW { w: self }
    }
}
