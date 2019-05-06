#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::IMSC {
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
#[doc = "Possible values of the field `CTSMIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSMIMR {
    #[doc = "set the interrupt mask"]
    SET_MASK,
    #[doc = "clear the interrupt mask"]
    CLEAR_MASK,
}
impl CTSMIMR {
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
            CTSMIMR::SET_MASK => true,
            CTSMIMR::CLEAR_MASK => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSMIMR {
        match value {
            true => CTSMIMR::SET_MASK,
            false => CTSMIMR::CLEAR_MASK,
        }
    }
    #[doc = "Checks if the value of the field is `SET_MASK`"]
    #[inline]
    pub fn is_set_mask(&self) -> bool {
        *self == CTSMIMR::SET_MASK
    }
    #[doc = "Checks if the value of the field is `CLEAR_MASK`"]
    #[inline]
    pub fn is_clear_mask(&self) -> bool {
        *self == CTSMIMR::CLEAR_MASK
    }
}
#[doc = r" Value of the field"]
pub struct RXIMR {
    bits: bool,
}
impl RXIMR {
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
pub struct TXIMR {
    bits: bool,
}
impl TXIMR {
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
pub struct RTIMR {
    bits: bool,
}
impl RTIMR {
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
pub struct FEIMR {
    bits: bool,
}
impl FEIMR {
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
pub struct PEIMR {
    bits: bool,
}
impl PEIMR {
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
pub struct BEIMR {
    bits: bool,
}
impl BEIMR {
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
pub struct OEIMR {
    bits: bool,
}
impl OEIMR {
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
pub struct XOFFIMR {
    bits: bool,
}
impl XOFFIMR {
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
pub struct TXFEIMR {
    bits: bool,
}
impl TXFEIMR {
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
#[doc = "Values that can be written to the field `CTSMIM`"]
pub enum CTSMIMW {
    #[doc = "set the interrupt mask"]
    SET_MASK,
    #[doc = "clear the interrupt mask"]
    CLEAR_MASK,
}
impl CTSMIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSMIMW::SET_MASK => true,
            CTSMIMW::CLEAR_MASK => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSMIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSMIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "set the interrupt mask"]
    #[inline]
    pub fn set_mask(self) -> &'a mut W {
        self.variant(CTSMIMW::SET_MASK)
    }
    #[doc = "clear the interrupt mask"]
    #[inline]
    pub fn clear_mask(self) -> &'a mut W {
        self.variant(CTSMIMW::CLEAR_MASK)
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
#[doc = r" Proxy"]
pub struct _RXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIMW<'a> {
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
pub struct _TXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIMW<'a> {
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
#[doc = r" Proxy"]
pub struct _RTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RTIMW<'a> {
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
#[doc = r" Proxy"]
pub struct _FEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _FEIMW<'a> {
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
pub struct _PEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _PEIMW<'a> {
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
pub struct _BEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BEIMW<'a> {
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
pub struct _OEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _OEIMW<'a> {
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
#[doc = r" Proxy"]
pub struct _XOFFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _XOFFIMW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXFEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFEIMW<'a> {
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 1 - Clear to send modem interrupt mask. On a read, the current mask for the CTSMIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn ctsmim(&self) -> CTSMIMR {
        CTSMIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Receive interrupt mask. On a read, the current mask for the RXIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn rxim(&self) -> RXIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RXIMR { bits }
    }
    #[doc = "Bit 5 - Transmit interrupt mask. On a read, the current mask for the TXIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn txim(&self) -> TXIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TXIMR { bits }
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask. On a read, the current mask for the RTIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn rtim(&self) -> RTIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RTIMR { bits }
    }
    #[doc = "Bit 7 - Framing error interrupt mask. On a read, the current mask for the FEIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn feim(&self) -> FEIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        FEIMR { bits }
    }
    #[doc = "Bit 8 - Parity error interrupt mask. On a read, the current mask for the PEIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn peim(&self) -> PEIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        PEIMR { bits }
    }
    #[doc = "Bit 9 - Break error interrupt mask. On a read, the current mask for the BEIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn beim(&self) -> BEIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        BEIMR { bits }
    }
    #[doc = "Bit 10 - Overrun error interrupt mask. On a read, the current mask for the OEIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn oeim(&self) -> OEIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        OEIMR { bits }
    }
    #[doc = "Bit 11 - XOFF interrupt mask. On a read, the current mask for the XOFFIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn xoffim(&self) -> XOFFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        XOFFIMR { bits }
    }
    #[doc = "Bit 12 - TX FIFO empty interrupt mask. On a read, the current mask for the TXFEIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn txfeim(&self) -> TXFEIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TXFEIMR { bits }
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
    #[doc = "Bit 1 - Clear to send modem interrupt mask. On a read, the current mask for the CTSMIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn ctsmim(&mut self) -> _CTSMIMW {
        _CTSMIMW { w: self }
    }
    #[doc = "Bit 4 - Receive interrupt mask. On a read, the current mask for the RXIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn rxim(&mut self) -> _RXIMW {
        _RXIMW { w: self }
    }
    #[doc = "Bit 5 - Transmit interrupt mask. On a read, the current mask for the TXIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn txim(&mut self) -> _TXIMW {
        _TXIMW { w: self }
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask. On a read, the current mask for the RTIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn rtim(&mut self) -> _RTIMW {
        _RTIMW { w: self }
    }
    #[doc = "Bit 7 - Framing error interrupt mask. On a read, the current mask for the FEIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn feim(&mut self) -> _FEIMW {
        _FEIMW { w: self }
    }
    #[doc = "Bit 8 - Parity error interrupt mask. On a read, the current mask for the PEIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn peim(&mut self) -> _PEIMW {
        _PEIMW { w: self }
    }
    #[doc = "Bit 9 - Break error interrupt mask. On a read, the current mask for the BEIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn beim(&mut self) -> _BEIMW {
        _BEIMW { w: self }
    }
    #[doc = "Bit 10 - Overrun error interrupt mask. On a read, the current mask for the OEIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn oeim(&mut self) -> _OEIMW {
        _OEIMW { w: self }
    }
    #[doc = "Bit 11 - XOFF interrupt mask. On a read, the current mask for the XOFFIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn xoffim(&mut self) -> _XOFFIMW {
        _XOFFIMW { w: self }
    }
    #[doc = "Bit 12 - TX FIFO empty interrupt mask. On a read, the current mask for the TXFEIM interrupt is returned.<ul><li>0: Clears the mask (interrupt is disabled).</li><li>1: Sets the mask (interrupt is enabled).</li></ul>"]
    #[inline]
    pub fn txfeim(&mut self) -> _TXFEIMW {
        _TXFEIMW { w: self }
    }
}
