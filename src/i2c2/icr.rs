#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICR {
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
#[doc = r" Value of the field"]
pub struct TXFOVRICR {
    bits: bool,
}
impl TXFOVRICR {
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
pub struct LBRICR {
    bits: bool,
}
impl LBRICR {
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
pub struct RFSRICR {
    bits: bool,
}
impl RFSRICR {
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
pub struct RFSEICR {
    bits: bool,
}
impl RFSEICR {
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
pub struct WTSRICR {
    bits: bool,
}
impl WTSRICR {
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
pub struct MTDICR {
    bits: bool,
}
impl MTDICR {
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
pub struct STDICR {
    bits: bool,
}
impl STDICR {
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
pub struct SALICR {
    bits: bool,
}
impl SALICR {
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
pub struct MALICR {
    bits: bool,
}
impl MALICR {
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
pub struct BERRICR {
    bits: bool,
}
impl BERRICR {
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
pub struct MTDWSICR {
    bits: bool,
}
impl MTDWSICR {
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
pub struct TIMEOUTICR {
    bits: bool,
}
impl TIMEOUTICR {
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
#[doc = r" Proxy"]
pub struct _TXFOVRICW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFOVRICW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LBRICW<'a> {
    w: &'a mut W,
}
impl<'a> _LBRICW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFSRICW<'a> {
    w: &'a mut W,
}
impl<'a> _RFSRICW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFSEICW<'a> {
    w: &'a mut W,
}
impl<'a> _RFSEICW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WTSRICW<'a> {
    w: &'a mut W,
}
impl<'a> _WTSRICW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MTDICW<'a> {
    w: &'a mut W,
}
impl<'a> _MTDICW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STDICW<'a> {
    w: &'a mut W,
}
impl<'a> _STDICW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SALICW<'a> {
    w: &'a mut W,
}
impl<'a> _SALICW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MALICW<'a> {
    w: &'a mut W,
}
impl<'a> _MALICW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BERRICW<'a> {
    w: &'a mut W,
}
impl<'a> _BERRICW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MTDWSICW<'a> {
    w: &'a mut W,
}
impl<'a> _MTDWSICW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMEOUTICW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUTICW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 3 - Tx FIFO overrun interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn txfovric(&self) -> TXFOVRICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFOVRICR { bits }
    }
    #[doc = "Bit 15 - Length number of bytes received interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn lbric(&self) -> LBRICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LBRICR { bits }
    }
    #[doc = "Bit 16 - Read-from-Slave request interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn rfsric(&self) -> RFSRICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFSRICR { bits }
    }
    #[doc = "Bit 17 - Read-from-Slave empty interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn rfseic(&self) -> RFSEICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFSEICR { bits }
    }
    #[doc = "Bit 18 - Write-to-Slave request interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn wtsric(&self) -> WTSRICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WTSRICR { bits }
    }
    #[doc = "Bit 19 - Master Transaction done interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn mtdic(&self) -> MTDICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MTDICR { bits }
    }
    #[doc = "Bit 20 - Slave Transaction done interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn stdic(&self) -> STDICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STDICR { bits }
    }
    #[doc = "Bit 23 - Slave Arbitration lost interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn salic(&self) -> SALICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SALICR { bits }
    }
    #[doc = "Bit 24 - Master Arbitration lost interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn malic(&self) -> MALICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MALICR { bits }
    }
    #[doc = "Bit 25 - Bus Error interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn berric(&self) -> BERRICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BERRICR { bits }
    }
    #[doc = "Bit 28 - Master Transaction done without stop interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn mtdwsic(&self) -> MTDWSICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MTDWSICR { bits }
    }
    #[doc = "Bit 30 - Timeout or Tlow error interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn timeoutic(&self) -> TIMEOUTICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIMEOUTICR { bits }
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - Tx FIFO overrun interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn txfovric(&mut self) -> _TXFOVRICW {
        _TXFOVRICW { w: self }
    }
    #[doc = "Bit 15 - Length number of bytes received interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn lbric(&mut self) -> _LBRICW {
        _LBRICW { w: self }
    }
    #[doc = "Bit 16 - Read-from-Slave request interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn rfsric(&mut self) -> _RFSRICW {
        _RFSRICW { w: self }
    }
    #[doc = "Bit 17 - Read-from-Slave empty interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn rfseic(&mut self) -> _RFSEICW {
        _RFSEICW { w: self }
    }
    #[doc = "Bit 18 - Write-to-Slave request interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn wtsric(&mut self) -> _WTSRICW {
        _WTSRICW { w: self }
    }
    #[doc = "Bit 19 - Master Transaction done interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn mtdic(&mut self) -> _MTDICW {
        _MTDICW { w: self }
    }
    #[doc = "Bit 20 - Slave Transaction done interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn stdic(&mut self) -> _STDICW {
        _STDICW { w: self }
    }
    #[doc = "Bit 23 - Slave Arbitration lost interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn salic(&mut self) -> _SALICW {
        _SALICW { w: self }
    }
    #[doc = "Bit 24 - Master Arbitration lost interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn malic(&mut self) -> _MALICW {
        _MALICW { w: self }
    }
    #[doc = "Bit 25 - Bus Error interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn berric(&mut self) -> _BERRICW {
        _BERRICW { w: self }
    }
    #[doc = "Bit 28 - Master Transaction done without stop interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn mtdwsic(&mut self) -> _MTDWSICW {
        _MTDWSICW { w: self }
    }
    #[doc = "Bit 30 - Timeout or Tlow error interrupt clear.<ul><li>0: Has no effect.</li><li>1: Clears interrupt pending.</li></ul>"]
    #[inline]
    pub fn timeoutic(&mut self) -> _TIMEOUTICW {
        _TIMEOUTICW { w: self }
    }
}
