#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::DMACR {
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
#[doc = "Possible values of the field `RXDMASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMASER {
    #[doc = "Single transfer DMA in receive disable"]
    RX_DMA_DISABLE,
    #[doc = "Single transfer DMA in receive enable"]
    RX_DMA_ENABLE,
}
impl RXDMASER {
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
            RXDMASER::RX_DMA_DISABLE => false,
            RXDMASER::RX_DMA_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDMASER {
        match value {
            false => RXDMASER::RX_DMA_DISABLE,
            true => RXDMASER::RX_DMA_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `RX_DMA_DISABLE`"]
    #[inline]
    pub fn is_rx_dma_disable(&self) -> bool {
        *self == RXDMASER::RX_DMA_DISABLE
    }
    #[doc = "Checks if the value of the field is `RX_DMA_ENABLE`"]
    #[inline]
    pub fn is_rx_dma_enable(&self) -> bool {
        *self == RXDMASER::RX_DMA_ENABLE
    }
}
#[doc = "Possible values of the field `TXDMASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMASER {
    #[doc = "Single transfer DMA in transmit disable"]
    TX_DMA_DISABLE,
    #[doc = "Single transfer DMA in transmit enable"]
    TX_DMA_ENABLE,
}
impl TXDMASER {
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
            TXDMASER::TX_DMA_DISABLE => false,
            TXDMASER::TX_DMA_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDMASER {
        match value {
            false => TXDMASER::TX_DMA_DISABLE,
            true => TXDMASER::TX_DMA_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `TX_DMA_DISABLE`"]
    #[inline]
    pub fn is_tx_dma_disable(&self) -> bool {
        *self == TXDMASER::TX_DMA_DISABLE
    }
    #[doc = "Checks if the value of the field is `TX_DMA_ENABLE`"]
    #[inline]
    pub fn is_tx_dma_enable(&self) -> bool {
        *self == TXDMASER::TX_DMA_ENABLE
    }
}
#[doc = "Values that can be written to the field `RXDMASE`"]
pub enum RXDMASEW {
    #[doc = "Single transfer DMA in receive disable"]
    RX_DMA_DISABLE,
    #[doc = "Single transfer DMA in receive enable"]
    RX_DMA_ENABLE,
}
impl RXDMASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDMASEW::RX_DMA_DISABLE => false,
            RXDMASEW::RX_DMA_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDMASEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDMASEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDMASEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single transfer DMA in receive disable"]
    #[inline]
    pub fn rx_dma_disable(self) -> &'a mut W {
        self.variant(RXDMASEW::RX_DMA_DISABLE)
    }
    #[doc = "Single transfer DMA in receive enable"]
    #[inline]
    pub fn rx_dma_enable(self) -> &'a mut W {
        self.variant(RXDMASEW::RX_DMA_ENABLE)
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
#[doc = "Values that can be written to the field `TXDMASE`"]
pub enum TXDMASEW {
    #[doc = "Single transfer DMA in transmit disable"]
    TX_DMA_DISABLE,
    #[doc = "Single transfer DMA in transmit enable"]
    TX_DMA_ENABLE,
}
impl TXDMASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDMASEW::TX_DMA_DISABLE => false,
            TXDMASEW::TX_DMA_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDMASEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDMASEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDMASEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single transfer DMA in transmit disable"]
    #[inline]
    pub fn tx_dma_disable(self) -> &'a mut W {
        self.variant(TXDMASEW::TX_DMA_DISABLE)
    }
    #[doc = "Single transfer DMA in transmit enable"]
    #[inline]
    pub fn tx_dma_enable(self) -> &'a mut W {
        self.variant(TXDMASEW::TX_DMA_ENABLE)
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
    #[doc = "Bit 0 - Single receive DMA request.<ul><li>0: Single transfer DMA in receive disable.</li><li>1: Single transfer DMA in receive enable.</li></ul>"]
    #[inline]
    pub fn rxdmase(&self) -> RXDMASER {
        RXDMASER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Signle transmit DMA request.<ul><li>0: Single transfer DMA in transmit disable.</li><li>1: Single transfer DMA in transmit enable.</li></ul>"]
    #[inline]
    pub fn txdmase(&self) -> TXDMASER {
        TXDMASER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Single receive DMA request.<ul><li>0: Single transfer DMA in receive disable.</li><li>1: Single transfer DMA in receive enable.</li></ul>"]
    #[inline]
    pub fn rxdmase(&mut self) -> _RXDMASEW {
        _RXDMASEW { w: self }
    }
    #[doc = "Bit 2 - Signle transmit DMA request.<ul><li>0: Single transfer DMA in transmit disable.</li><li>1: Single transfer DMA in transmit enable.</li></ul>"]
    #[inline]
    pub fn txdmase(&mut self) -> _TXDMASEW {
        _TXDMASEW { w: self }
    }
}
