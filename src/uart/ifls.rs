#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::IFLS {
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
#[doc = "Possible values of the field `TXIFLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIFLSELR {
    #[doc = "Interrupt when FIFO >= 1/64 empty"]
    TXFIFO_1_64,
    #[doc = "Interrupt when FIFO >= 1/32 empty"]
    TXFIFO_1_32,
    #[doc = "Interrupt when FIFO >= 1/16 empty"]
    TXFIFO_1_16,
    #[doc = "Interrupt when FIFO >= 1/8 empty"]
    TXFIFO_1_8,
    #[doc = "Interrupt when FIFO >= 1/4 empty"]
    TXFIFO_1_4,
    #[doc = "Interrupt when FIFO >= 1/2 empty"]
    TXFIFO_1_2,
    #[doc = "Interrupt when FIFO >= 3/4 empty"]
    TXFIFO_3_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXIFLSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXIFLSELR::TXFIFO_1_64 => 0,
            TXIFLSELR::TXFIFO_1_32 => 1,
            TXIFLSELR::TXFIFO_1_16 => 2,
            TXIFLSELR::TXFIFO_1_8 => 3,
            TXIFLSELR::TXFIFO_1_4 => 4,
            TXIFLSELR::TXFIFO_1_2 => 5,
            TXIFLSELR::TXFIFO_3_4 => 6,
            TXIFLSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXIFLSELR {
        match value {
            0 => TXIFLSELR::TXFIFO_1_64,
            1 => TXIFLSELR::TXFIFO_1_32,
            2 => TXIFLSELR::TXFIFO_1_16,
            3 => TXIFLSELR::TXFIFO_1_8,
            4 => TXIFLSELR::TXFIFO_1_4,
            5 => TXIFLSELR::TXFIFO_1_2,
            6 => TXIFLSELR::TXFIFO_3_4,
            i => TXIFLSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXFIFO_1_64`"]
    #[inline]
    pub fn is_txfifo_1_64(&self) -> bool {
        *self == TXIFLSELR::TXFIFO_1_64
    }
    #[doc = "Checks if the value of the field is `TXFIFO_1_32`"]
    #[inline]
    pub fn is_txfifo_1_32(&self) -> bool {
        *self == TXIFLSELR::TXFIFO_1_32
    }
    #[doc = "Checks if the value of the field is `TXFIFO_1_16`"]
    #[inline]
    pub fn is_txfifo_1_16(&self) -> bool {
        *self == TXIFLSELR::TXFIFO_1_16
    }
    #[doc = "Checks if the value of the field is `TXFIFO_1_8`"]
    #[inline]
    pub fn is_txfifo_1_8(&self) -> bool {
        *self == TXIFLSELR::TXFIFO_1_8
    }
    #[doc = "Checks if the value of the field is `TXFIFO_1_4`"]
    #[inline]
    pub fn is_txfifo_1_4(&self) -> bool {
        *self == TXIFLSELR::TXFIFO_1_4
    }
    #[doc = "Checks if the value of the field is `TXFIFO_1_2`"]
    #[inline]
    pub fn is_txfifo_1_2(&self) -> bool {
        *self == TXIFLSELR::TXFIFO_1_2
    }
    #[doc = "Checks if the value of the field is `TXFIFO_3_4`"]
    #[inline]
    pub fn is_txfifo_3_4(&self) -> bool {
        *self == TXIFLSELR::TXFIFO_3_4
    }
}
#[doc = "Possible values of the field `RXIFLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIFLSELR {
    #[doc = "Interrupt when FIFO >= 1/64 full"]
    RXFIFO_1_64,
    #[doc = "Interrupt when FIFO >= 1/32 full"]
    RXFIFO_1_32,
    #[doc = "Interrupt when FIFO >= 1/16 full"]
    RXFIFO_1_16,
    #[doc = "Interrupt when FIFO >= 1/8 full"]
    RXFIFO_1_8,
    #[doc = "Interrupt when FIFO >= 1/4 full"]
    RXFIFO_1_4,
    #[doc = "Interrupt when FIFO >= 1/2 full"]
    RXFIFO_1_2,
    #[doc = "Interrupt when FIFO >= 3/4 full"]
    RXFIFO_3_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXIFLSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXIFLSELR::RXFIFO_1_64 => 0,
            RXIFLSELR::RXFIFO_1_32 => 1,
            RXIFLSELR::RXFIFO_1_16 => 2,
            RXIFLSELR::RXFIFO_1_8 => 3,
            RXIFLSELR::RXFIFO_1_4 => 4,
            RXIFLSELR::RXFIFO_1_2 => 5,
            RXIFLSELR::RXFIFO_3_4 => 6,
            RXIFLSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXIFLSELR {
        match value {
            0 => RXIFLSELR::RXFIFO_1_64,
            1 => RXIFLSELR::RXFIFO_1_32,
            2 => RXIFLSELR::RXFIFO_1_16,
            3 => RXIFLSELR::RXFIFO_1_8,
            4 => RXIFLSELR::RXFIFO_1_4,
            5 => RXIFLSELR::RXFIFO_1_2,
            6 => RXIFLSELR::RXFIFO_3_4,
            i => RXIFLSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFO_1_64`"]
    #[inline]
    pub fn is_rxfifo_1_64(&self) -> bool {
        *self == RXIFLSELR::RXFIFO_1_64
    }
    #[doc = "Checks if the value of the field is `RXFIFO_1_32`"]
    #[inline]
    pub fn is_rxfifo_1_32(&self) -> bool {
        *self == RXIFLSELR::RXFIFO_1_32
    }
    #[doc = "Checks if the value of the field is `RXFIFO_1_16`"]
    #[inline]
    pub fn is_rxfifo_1_16(&self) -> bool {
        *self == RXIFLSELR::RXFIFO_1_16
    }
    #[doc = "Checks if the value of the field is `RXFIFO_1_8`"]
    #[inline]
    pub fn is_rxfifo_1_8(&self) -> bool {
        *self == RXIFLSELR::RXFIFO_1_8
    }
    #[doc = "Checks if the value of the field is `RXFIFO_1_4`"]
    #[inline]
    pub fn is_rxfifo_1_4(&self) -> bool {
        *self == RXIFLSELR::RXFIFO_1_4
    }
    #[doc = "Checks if the value of the field is `RXFIFO_1_2`"]
    #[inline]
    pub fn is_rxfifo_1_2(&self) -> bool {
        *self == RXIFLSELR::RXFIFO_1_2
    }
    #[doc = "Checks if the value of the field is `RXFIFO_3_4`"]
    #[inline]
    pub fn is_rxfifo_3_4(&self) -> bool {
        *self == RXIFLSELR::RXFIFO_3_4
    }
}
#[doc = "Values that can be written to the field `TXIFLSEL`"]
pub enum TXIFLSELW {
    #[doc = "Interrupt when FIFO >= 1/64 empty"]
    TXFIFO_1_64,
    #[doc = "Interrupt when FIFO >= 1/32 empty"]
    TXFIFO_1_32,
    #[doc = "Interrupt when FIFO >= 1/16 empty"]
    TXFIFO_1_16,
    #[doc = "Interrupt when FIFO >= 1/8 empty"]
    TXFIFO_1_8,
    #[doc = "Interrupt when FIFO >= 1/4 empty"]
    TXFIFO_1_4,
    #[doc = "Interrupt when FIFO >= 1/2 empty"]
    TXFIFO_1_2,
    #[doc = "Interrupt when FIFO >= 3/4 empty"]
    TXFIFO_3_4,
}
impl TXIFLSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXIFLSELW::TXFIFO_1_64 => 0,
            TXIFLSELW::TXFIFO_1_32 => 1,
            TXIFLSELW::TXFIFO_1_16 => 2,
            TXIFLSELW::TXFIFO_1_8 => 3,
            TXIFLSELW::TXFIFO_1_4 => 4,
            TXIFLSELW::TXFIFO_1_2 => 5,
            TXIFLSELW::TXFIFO_3_4 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXIFLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIFLSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXIFLSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Interrupt when FIFO >= 1/64 empty"]
    #[inline]
    pub fn txfifo_1_64(self) -> &'a mut W {
        self.variant(TXIFLSELW::TXFIFO_1_64)
    }
    #[doc = "Interrupt when FIFO >= 1/32 empty"]
    #[inline]
    pub fn txfifo_1_32(self) -> &'a mut W {
        self.variant(TXIFLSELW::TXFIFO_1_32)
    }
    #[doc = "Interrupt when FIFO >= 1/16 empty"]
    #[inline]
    pub fn txfifo_1_16(self) -> &'a mut W {
        self.variant(TXIFLSELW::TXFIFO_1_16)
    }
    #[doc = "Interrupt when FIFO >= 1/8 empty"]
    #[inline]
    pub fn txfifo_1_8(self) -> &'a mut W {
        self.variant(TXIFLSELW::TXFIFO_1_8)
    }
    #[doc = "Interrupt when FIFO >= 1/4 empty"]
    #[inline]
    pub fn txfifo_1_4(self) -> &'a mut W {
        self.variant(TXIFLSELW::TXFIFO_1_4)
    }
    #[doc = "Interrupt when FIFO >= 1/2 empty"]
    #[inline]
    pub fn txfifo_1_2(self) -> &'a mut W {
        self.variant(TXIFLSELW::TXFIFO_1_2)
    }
    #[doc = "Interrupt when FIFO >= 3/4 empty"]
    #[inline]
    pub fn txfifo_3_4(self) -> &'a mut W {
        self.variant(TXIFLSELW::TXFIFO_3_4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXIFLSEL`"]
pub enum RXIFLSELW {
    #[doc = "Interrupt when FIFO >= 1/64 full"]
    RXFIFO_1_64,
    #[doc = "Interrupt when FIFO >= 1/32 full"]
    RXFIFO_1_32,
    #[doc = "Interrupt when FIFO >= 1/16 full"]
    RXFIFO_1_16,
    #[doc = "Interrupt when FIFO >= 1/8 full"]
    RXFIFO_1_8,
    #[doc = "Interrupt when FIFO >= 1/4 full"]
    RXFIFO_1_4,
    #[doc = "Interrupt when FIFO >= 1/2 full"]
    RXFIFO_1_2,
    #[doc = "Interrupt when FIFO >= 3/4 full"]
    RXFIFO_3_4,
}
impl RXIFLSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXIFLSELW::RXFIFO_1_64 => 0,
            RXIFLSELW::RXFIFO_1_32 => 1,
            RXIFLSELW::RXFIFO_1_16 => 2,
            RXIFLSELW::RXFIFO_1_8 => 3,
            RXIFLSELW::RXFIFO_1_4 => 4,
            RXIFLSELW::RXFIFO_1_2 => 5,
            RXIFLSELW::RXFIFO_3_4 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIFLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIFLSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIFLSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Interrupt when FIFO >= 1/64 full"]
    #[inline]
    pub fn rxfifo_1_64(self) -> &'a mut W {
        self.variant(RXIFLSELW::RXFIFO_1_64)
    }
    #[doc = "Interrupt when FIFO >= 1/32 full"]
    #[inline]
    pub fn rxfifo_1_32(self) -> &'a mut W {
        self.variant(RXIFLSELW::RXFIFO_1_32)
    }
    #[doc = "Interrupt when FIFO >= 1/16 full"]
    #[inline]
    pub fn rxfifo_1_16(self) -> &'a mut W {
        self.variant(RXIFLSELW::RXFIFO_1_16)
    }
    #[doc = "Interrupt when FIFO >= 1/8 full"]
    #[inline]
    pub fn rxfifo_1_8(self) -> &'a mut W {
        self.variant(RXIFLSELW::RXFIFO_1_8)
    }
    #[doc = "Interrupt when FIFO >= 1/4 full"]
    #[inline]
    pub fn rxfifo_1_4(self) -> &'a mut W {
        self.variant(RXIFLSELW::RXFIFO_1_4)
    }
    #[doc = "Interrupt when FIFO >= 1/2 full"]
    #[inline]
    pub fn rxfifo_1_2(self) -> &'a mut W {
        self.variant(RXIFLSELW::RXFIFO_1_2)
    }
    #[doc = "Interrupt when FIFO >= 3/4 full"]
    #[inline]
    pub fn rxfifo_3_4(self) -> &'a mut W {
        self.variant(RXIFLSELW::RXFIFO_3_4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
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
    #[doc = "Bits 0:2 - Transmit interrupt FIFO level select. This bit field selects the trigger points for TX FIFO interrupt:<ul><li>000b: Interrupt when FIFO >= 1/64 empty.</li><li>001b: Interrupt when FIFO >= 1/32 empty.</li><li>010b: Interrupt when FIFO >= 1/16 empty.</li><li>011b: Interrupt when FIFO >= 1/8 empty.</li><li>100b: Interrupt when FIFO >= 1/4 empty.</li><li>101b: Interrupt when FIFO >= 1/2 empty.</li><li>110b: Interrupt when FIFO >= 3/4 empty.</li></ul>"]
    #[inline]
    pub fn txiflsel(&self) -> TXIFLSELR {
        TXIFLSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 3:5 - Receive interrupt FIFO level select. This bit field selects the trigger points for RX FIFO interrupt:<ul><li>000b: Interrupt when FIFO >= 1/64 full.</li><li>001b: Interrupt when FIFO >= 1/32 full.</li><li>010b: Interrupt when FIFO >= 1/16 full.</li><li>011b: Interrupt when FIFO >= 1/8 full.</li><li>100b: Interrupt when FIFO >= 1/4 full.</li><li>101b: Interrupt when FIFO >= 1/2 full.</li><li>110b: Interrupt when FIFO >= 3/4 full.</li></ul>"]
    #[inline]
    pub fn rxiflsel(&self) -> RXIFLSELR {
        RXIFLSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 18 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Transmit interrupt FIFO level select. This bit field selects the trigger points for TX FIFO interrupt:<ul><li>000b: Interrupt when FIFO >= 1/64 empty.</li><li>001b: Interrupt when FIFO >= 1/32 empty.</li><li>010b: Interrupt when FIFO >= 1/16 empty.</li><li>011b: Interrupt when FIFO >= 1/8 empty.</li><li>100b: Interrupt when FIFO >= 1/4 empty.</li><li>101b: Interrupt when FIFO >= 1/2 empty.</li><li>110b: Interrupt when FIFO >= 3/4 empty.</li></ul>"]
    #[inline]
    pub fn txiflsel(&mut self) -> _TXIFLSELW {
        _TXIFLSELW { w: self }
    }
    #[doc = "Bits 3:5 - Receive interrupt FIFO level select. This bit field selects the trigger points for RX FIFO interrupt:<ul><li>000b: Interrupt when FIFO >= 1/64 full.</li><li>001b: Interrupt when FIFO >= 1/32 full.</li><li>010b: Interrupt when FIFO >= 1/16 full.</li><li>011b: Interrupt when FIFO >= 1/8 full.</li><li>100b: Interrupt when FIFO >= 1/4 full.</li><li>101b: Interrupt when FIFO >= 1/2 full.</li><li>110b: Interrupt when FIFO >= 3/4 full.</li></ul>"]
    #[inline]
    pub fn rxiflsel(&mut self) -> _RXIFLSELW {
        _RXIFLSELW { w: self }
    }
}
