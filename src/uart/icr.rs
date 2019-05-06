#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::ICR {
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
}
#[doc = r" Proxy"]
pub struct _CTSMICW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSMICW<'a> {
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
pub struct _RXICW<'a> {
    w: &'a mut W,
}
impl<'a> _RXICW<'a> {
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
pub struct _TXICW<'a> {
    w: &'a mut W,
}
impl<'a> _TXICW<'a> {
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
pub struct _RTICW<'a> {
    w: &'a mut W,
}
impl<'a> _RTICW<'a> {
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
pub struct _FEICW<'a> {
    w: &'a mut W,
}
impl<'a> _FEICW<'a> {
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
pub struct _PEICW<'a> {
    w: &'a mut W,
}
impl<'a> _PEICW<'a> {
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
pub struct _BEICW<'a> {
    w: &'a mut W,
}
impl<'a> _BEICW<'a> {
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
pub struct _OEICW<'a> {
    w: &'a mut W,
}
impl<'a> _OEICW<'a> {
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
pub struct _XOFFICW<'a> {
    w: &'a mut W,
}
impl<'a> _XOFFICW<'a> {
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
pub struct _TXFEICW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFEICW<'a> {
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
    #[doc = "Bit 1 - Clear to send modem interrupt clear.<ul><li>0: No effect.</li><li>1: Clears the interrupt.</li></ul>"]
    #[inline]
    pub fn ctsmic(&mut self) -> _CTSMICW {
        _CTSMICW { w: self }
    }
    #[doc = "Bit 4 - Receive interrupt clear.<ul><li>0: No effect.</li><li>1: Clears the interrupt.</li></ul>"]
    #[inline]
    pub fn rxic(&mut self) -> _RXICW {
        _RXICW { w: self }
    }
    #[doc = "Bit 5 - Transmit interrupt clear.<ul><li>0: No effect.</li><li>1: Clears the interrupt.</li></ul>"]
    #[inline]
    pub fn txic(&mut self) -> _TXICW {
        _TXICW { w: self }
    }
    #[doc = "Bit 6 - Receive timeout interrupt clear.<ul><li>0: No effect.</li><li>1: Clears the interrupt.</li></ul>"]
    #[inline]
    pub fn rtic(&mut self) -> _RTICW {
        _RTICW { w: self }
    }
    #[doc = "Bit 7 - Framing error interrupt clear.<ul><li>0: No effect.</li><li>1: Clears the interrupt.</li></ul>"]
    #[inline]
    pub fn feic(&mut self) -> _FEICW {
        _FEICW { w: self }
    }
    #[doc = "Bit 8 - Parity error interrupt clear.<ul><li>0: No effect.</li><li>1: Clears the interrupt.</li></ul>"]
    #[inline]
    pub fn peic(&mut self) -> _PEICW {
        _PEICW { w: self }
    }
    #[doc = "Bit 9 - Break error interrupt clear.<ul><li>0: No effect.</li><li>1: Clears the interrupt.</li></ul>"]
    #[inline]
    pub fn beic(&mut self) -> _BEICW {
        _BEICW { w: self }
    }
    #[doc = "Bit 10 - Overrun error interrupt clear.<ul><li>0: No effect.</li><li>1: Clears the interrupt.</li></ul>"]
    #[inline]
    pub fn oeic(&mut self) -> _OEICW {
        _OEICW { w: self }
    }
    #[doc = "Bit 11 - XOFF interrupt clear.<ul><li>0: No effect.</li><li>1: Clears the interrupt.</li></ul>"]
    #[inline]
    pub fn xoffic(&mut self) -> _XOFFICW {
        _XOFFICW { w: self }
    }
    #[doc = "Bit 12 - TX FIFO empty interrupt clear.<ul><li>0: No effect.</li><li>1: Clears the interrupt.</li></ul>"]
    #[inline]
    pub fn txfeic(&mut self) -> _TXFEICW {
        _TXFEICW { w: self }
    }
}
