#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CR {
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
#[doc = "Possible values of the field `INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTENR {
    #[doc = "Disable watchdog interrupt"]
    DISABLE,
    #[doc = "Enable watchdog interrupt"]
    ENABLE,
}
impl INTENR {
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
            INTENR::DISABLE => false,
            INTENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTENR {
        match value {
            false => INTENR::DISABLE,
            true => INTENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == INTENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == INTENR::ENABLE
    }
}
#[doc = "Possible values of the field `RESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESENR {
    #[doc = "Disable watchdog reset"]
    DISABLE,
    #[doc = "Enable watchdog reset"]
    ENABLE,
}
impl RESENR {
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
            RESENR::DISABLE => false,
            RESENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESENR {
        match value {
            false => RESENR::DISABLE,
            true => RESENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RESENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RESENR::ENABLE
    }
}
#[doc = "Values that can be written to the field `INTEN`"]
pub enum INTENW {
    #[doc = "Disable watchdog interrupt"]
    DISABLE,
    #[doc = "Enable watchdog interrupt"]
    ENABLE,
}
impl INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTENW::DISABLE => false,
            INTENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable watchdog interrupt"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTENW::DISABLE)
    }
    #[doc = "Enable watchdog interrupt"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(INTENW::ENABLE)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESEN`"]
pub enum RESENW {
    #[doc = "Disable watchdog reset"]
    DISABLE,
    #[doc = "Enable watchdog reset"]
    ENABLE,
}
impl RESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESENW::DISABLE => false,
            RESENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESENW<'a> {
    w: &'a mut W,
}
impl<'a> _RESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable watchdog reset"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RESENW::DISABLE)
    }
    #[doc = "Enable watchdog reset"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RESENW::ENABLE)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Watchdog interrupt enable. Enable the interrupt event:<ul><li>0: watchdog interrupt is disabled.</li><li>1: watchdog interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn inten(&self) -> INTENR {
        INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Watchdog reset enable. Enable the watchdog reset output:<ul><li>0: watchdog reset is disabled.</li><li>1: watchdog reset is enabled.</li></ul>"]
    #[inline]
    pub fn resen(&self) -> RESENR {
        RESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Watchdog interrupt enable. Enable the interrupt event:<ul><li>0: watchdog interrupt is disabled.</li><li>1: watchdog interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn inten(&mut self) -> _INTENW {
        _INTENW { w: self }
    }
    #[doc = "Bit 1 - Watchdog reset enable. Enable the watchdog reset output:<ul><li>0: watchdog reset is disabled.</li><li>1: watchdog reset is enabled.</li></ul>"]
    #[inline]
    pub fn resen(&mut self) -> _RESENW {
        _RESENW { w: self }
    }
}
