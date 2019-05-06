#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::IRQRAW {
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
#[doc = "Possible values of the field `CMDDONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDDONER {
    #[doc = "Irq pending"]
    IRQ_PENDING,
    #[doc = "Irq not pending"]
    IRQ_NOT_PENDING,
}
impl CMDDONER {
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
            CMDDONER::IRQ_PENDING => true,
            CMDDONER::IRQ_NOT_PENDING => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDDONER {
        match value {
            true => CMDDONER::IRQ_PENDING,
            false => CMDDONER::IRQ_NOT_PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `IRQ_PENDING`"]
    #[inline]
    pub fn is_irq_pending(&self) -> bool {
        *self == CMDDONER::IRQ_PENDING
    }
    #[doc = "Checks if the value of the field is `IRQ_NOT_PENDING`"]
    #[inline]
    pub fn is_irq_not_pending(&self) -> bool {
        *self == CMDDONER::IRQ_NOT_PENDING
    }
}
#[doc = r" Value of the field"]
pub struct CMDSTARTR {
    bits: bool,
}
impl CMDSTARTR {
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
pub struct CMDERRR {
    bits: bool,
}
impl CMDERRR {
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
pub struct ILLCMDR {
    bits: bool,
}
impl ILLCMDR {
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
pub struct READOKR {
    bits: bool,
}
impl READOKR {
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
pub struct FLNREADYR {
    bits: bool,
}
impl FLNREADYR {
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
#[doc = "Values that can be written to the field `CMDDONE`"]
pub enum CMDDONEW {
    #[doc = "Irq pending"]
    IRQ_PENDING,
    #[doc = "Irq not pending"]
    IRQ_NOT_PENDING,
}
impl CMDDONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDDONEW::IRQ_PENDING => true,
            CMDDONEW::IRQ_NOT_PENDING => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDDONEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDDONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDDONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Irq pending"]
    #[inline]
    pub fn irq_pending(self) -> &'a mut W {
        self.variant(CMDDONEW::IRQ_PENDING)
    }
    #[doc = "Irq not pending"]
    #[inline]
    pub fn irq_not_pending(self) -> &'a mut W {
        self.variant(CMDDONEW::IRQ_NOT_PENDING)
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
#[doc = r" Proxy"]
pub struct _CMDSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDSTARTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CMDERRW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDERRW<'a> {
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
pub struct _ILLCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _ILLCMDW<'a> {
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
#[doc = r" Proxy"]
pub struct _READOKW<'a> {
    w: &'a mut W,
}
impl<'a> _READOKW<'a> {
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
pub struct _FLNREADYW<'a> {
    w: &'a mut W,
}
impl<'a> _FLNREADYW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Command is done."]
    #[inline]
    pub fn cmddone(&self) -> CMDDONER {
        CMDDONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Command is started."]
    #[inline]
    pub fn cmdstart(&self) -> CMDSTARTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CMDSTARTR { bits }
    }
    #[doc = "Bit 2 - Command written while BUSY"]
    #[inline]
    pub fn cmderr(&self) -> CMDERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CMDERRR { bits }
    }
    #[doc = "Bit 3 - Illegal command written"]
    #[inline]
    pub fn illcmd(&self) -> ILLCMDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        ILLCMDR { bits }
    }
    #[doc = "Bit 4 - Mass read was OK."]
    #[inline]
    pub fn readok(&self) -> READOKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        READOKR { bits }
    }
    #[doc = "Bit 5 - Flash not ready (sleep)."]
    #[inline]
    pub fn flnready(&self) -> FLNREADYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        FLNREADYR { bits }
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
    #[doc = "Bit 0 - Command is done."]
    #[inline]
    pub fn cmddone(&mut self) -> _CMDDONEW {
        _CMDDONEW { w: self }
    }
    #[doc = "Bit 1 - Command is started."]
    #[inline]
    pub fn cmdstart(&mut self) -> _CMDSTARTW {
        _CMDSTARTW { w: self }
    }
    #[doc = "Bit 2 - Command written while BUSY"]
    #[inline]
    pub fn cmderr(&mut self) -> _CMDERRW {
        _CMDERRW { w: self }
    }
    #[doc = "Bit 3 - Illegal command written"]
    #[inline]
    pub fn illcmd(&mut self) -> _ILLCMDW {
        _ILLCMDW { w: self }
    }
    #[doc = "Bit 4 - Mass read was OK."]
    #[inline]
    pub fn readok(&mut self) -> _READOKW {
        _READOKW { w: self }
    }
    #[doc = "Bit 5 - Flash not ready (sleep)."]
    #[inline]
    pub fn flnready(&mut self) -> _FLNREADYW {
        _FLNREADYW { w: self }
    }
}
