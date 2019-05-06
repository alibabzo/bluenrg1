#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
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
#[doc = "Possible values of the field `MHZ32_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MHZ32_SELR {
    #[doc = "16 MHz is selected"]
    MHZ16,
    #[doc = "32 MHz is selected"]
    MHZ32,
}
impl MHZ32_SELR {
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
            MHZ32_SELR::MHZ16 => false,
            MHZ32_SELR::MHZ32 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MHZ32_SELR {
        match value {
            false => MHZ32_SELR::MHZ16,
            true => MHZ32_SELR::MHZ32,
        }
    }
    #[doc = "Checks if the value of the field is `MHZ16`"]
    #[inline]
    pub fn is_mhz16(&self) -> bool {
        *self == MHZ32_SELR::MHZ16
    }
    #[doc = "Checks if the value of the field is `MHZ32`"]
    #[inline]
    pub fn is_mhz32(&self) -> bool {
        *self == MHZ32_SELR::MHZ32
    }
}
#[doc = "Values that can be written to the field `MHZ32_SEL`"]
pub enum MHZ32_SELW {
    #[doc = "16 MHz is selected"]
    MHZ16,
    #[doc = "32 MHz is selected"]
    MHZ32,
}
impl MHZ32_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MHZ32_SELW::MHZ16 => false,
            MHZ32_SELW::MHZ32 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MHZ32_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _MHZ32_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MHZ32_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "16 MHz is selected"]
    #[inline]
    pub fn mhz16(self) -> &'a mut W {
        self.variant(MHZ32_SELW::MHZ16)
    }
    #[doc = "32 MHz is selected"]
    #[inline]
    pub fn mhz32(self) -> &'a mut W {
        self.variant(MHZ32_SELW::MHZ32)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Indicates the crystal frequency used in the application.<ul><li>0: The 16 MHz is selected.</li><li>1: The 32 MHz is selected.</li></ul>"]
    #[inline]
    pub fn mhz32_sel(&self) -> MHZ32_SELR {
        MHZ32_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - Indicates the crystal frequency used in the application.<ul><li>0: The 16 MHz is selected.</li><li>1: The 32 MHz is selected.</li></ul>"]
    #[inline]
    pub fn mhz32_sel(&mut self) -> _MHZ32_SELW {
        _MHZ32_SELW { w: self }
    }
}
