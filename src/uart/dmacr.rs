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
#[doc = "Possible values of the field `RXDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAER {
    #[doc = "DMA mode for reception enable"]
    DMA_MODE_RX_ENABLE,
    #[doc = "DMA mode for reception disable"]
    DMA_MODE_RX_DISABLE,
}
impl RXDMAER {
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
            RXDMAER::DMA_MODE_RX_ENABLE => true,
            RXDMAER::DMA_MODE_RX_DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDMAER {
        match value {
            true => RXDMAER::DMA_MODE_RX_ENABLE,
            false => RXDMAER::DMA_MODE_RX_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_MODE_RX_ENABLE`"]
    #[inline]
    pub fn is_dma_mode_rx_enable(&self) -> bool {
        *self == RXDMAER::DMA_MODE_RX_ENABLE
    }
    #[doc = "Checks if the value of the field is `DMA_MODE_RX_DISABLE`"]
    #[inline]
    pub fn is_dma_mode_rx_disable(&self) -> bool {
        *self == RXDMAER::DMA_MODE_RX_DISABLE
    }
}
#[doc = "Possible values of the field `TXDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAER {
    #[doc = "DMA mode for transmission enable"]
    DMA_MODE_TX_ENABLE,
    #[doc = "DMA mode for transmission disable"]
    DMA_MODE_TX_DISABLE,
}
impl TXDMAER {
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
            TXDMAER::DMA_MODE_TX_ENABLE => true,
            TXDMAER::DMA_MODE_TX_DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDMAER {
        match value {
            true => TXDMAER::DMA_MODE_TX_ENABLE,
            false => TXDMAER::DMA_MODE_TX_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_MODE_TX_ENABLE`"]
    #[inline]
    pub fn is_dma_mode_tx_enable(&self) -> bool {
        *self == TXDMAER::DMA_MODE_TX_ENABLE
    }
    #[doc = "Checks if the value of the field is `DMA_MODE_TX_DISABLE`"]
    #[inline]
    pub fn is_dma_mode_tx_disable(&self) -> bool {
        *self == TXDMAER::DMA_MODE_TX_DISABLE
    }
}
#[doc = "Possible values of the field `DMAONERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAONERRR {
    #[doc = "DMA receive requests are disabled when the UART error interrupt is asserted"]
    DMA_ON_ERR_ENABLE,
    #[doc = "UART error interrupt status has no impact in receive DMA mode"]
    DMA_ON_ERR_DISABLE,
}
impl DMAONERRR {
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
            DMAONERRR::DMA_ON_ERR_ENABLE => true,
            DMAONERRR::DMA_ON_ERR_DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAONERRR {
        match value {
            true => DMAONERRR::DMA_ON_ERR_ENABLE,
            false => DMAONERRR::DMA_ON_ERR_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_ON_ERR_ENABLE`"]
    #[inline]
    pub fn is_dma_on_err_enable(&self) -> bool {
        *self == DMAONERRR::DMA_ON_ERR_ENABLE
    }
    #[doc = "Checks if the value of the field is `DMA_ON_ERR_DISABLE`"]
    #[inline]
    pub fn is_dma_on_err_disable(&self) -> bool {
        *self == DMAONERRR::DMA_ON_ERR_DISABLE
    }
}
#[doc = "Values that can be written to the field `RXDMAE`"]
pub enum RXDMAEW {
    #[doc = "DMA mode for reception enable"]
    DMA_MODE_RX_ENABLE,
    #[doc = "DMA mode for reception disable"]
    DMA_MODE_RX_DISABLE,
}
impl RXDMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDMAEW::DMA_MODE_RX_ENABLE => true,
            RXDMAEW::DMA_MODE_RX_DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA mode for reception enable"]
    #[inline]
    pub fn dma_mode_rx_enable(self) -> &'a mut W {
        self.variant(RXDMAEW::DMA_MODE_RX_ENABLE)
    }
    #[doc = "DMA mode for reception disable"]
    #[inline]
    pub fn dma_mode_rx_disable(self) -> &'a mut W {
        self.variant(RXDMAEW::DMA_MODE_RX_DISABLE)
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
#[doc = "Values that can be written to the field `TXDMAE`"]
pub enum TXDMAEW {
    #[doc = "DMA mode for transmission enable"]
    DMA_MODE_TX_ENABLE,
    #[doc = "DMA mode for transmission disable"]
    DMA_MODE_TX_DISABLE,
}
impl TXDMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDMAEW::DMA_MODE_TX_ENABLE => true,
            TXDMAEW::DMA_MODE_TX_DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA mode for transmission enable"]
    #[inline]
    pub fn dma_mode_tx_enable(self) -> &'a mut W {
        self.variant(TXDMAEW::DMA_MODE_TX_ENABLE)
    }
    #[doc = "DMA mode for transmission disable"]
    #[inline]
    pub fn dma_mode_tx_disable(self) -> &'a mut W {
        self.variant(TXDMAEW::DMA_MODE_TX_DISABLE)
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
#[doc = "Values that can be written to the field `DMAONERR`"]
pub enum DMAONERRW {
    #[doc = "DMA receive requests are disabled when the UART error interrupt is asserted"]
    DMA_ON_ERR_ENABLE,
    #[doc = "UART error interrupt status has no impact in receive DMA mode"]
    DMA_ON_ERR_DISABLE,
}
impl DMAONERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAONERRW::DMA_ON_ERR_ENABLE => true,
            DMAONERRW::DMA_ON_ERR_DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAONERRW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAONERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAONERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA receive requests are disabled when the UART error interrupt is asserted"]
    #[inline]
    pub fn dma_on_err_enable(self) -> &'a mut W {
        self.variant(DMAONERRW::DMA_ON_ERR_ENABLE)
    }
    #[doc = "UART error interrupt status has no impact in receive DMA mode"]
    #[inline]
    pub fn dma_on_err_disable(self) -> &'a mut W {
        self.variant(DMAONERRW::DMA_ON_ERR_DISABLE)
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
    #[doc = "Bit 0 - Receive DMA enable bit.<ul><li>0: DMA mode is disabled for reception.</li><li>1: DMA mode is enabled for reception.</li></ul>"]
    #[inline]
    pub fn rxdmae(&self) -> RXDMAER {
        RXDMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Transmit DMA enable bit.<ul><li>0: DMA mode is disabled for transmit.</li><li>1: DMA mode is enabled for transmit.</li></ul>"]
    #[inline]
    pub fn txdmae(&self) -> TXDMAER {
        TXDMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - DMA on error.<ul><li>0: UART error interrupt status has no impact in receive DMA mode.</li><li>1: DMA receive requests are disabled when the UART error interrupt is asserted.</li></ul>"]
    #[inline]
    pub fn dmaonerr(&self) -> DMAONERRR {
        DMAONERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Receive DMA enable bit.<ul><li>0: DMA mode is disabled for reception.</li><li>1: DMA mode is enabled for reception.</li></ul>"]
    #[inline]
    pub fn rxdmae(&mut self) -> _RXDMAEW {
        _RXDMAEW { w: self }
    }
    #[doc = "Bit 1 - Transmit DMA enable bit.<ul><li>0: DMA mode is disabled for transmit.</li><li>1: DMA mode is enabled for transmit.</li></ul>"]
    #[inline]
    pub fn txdmae(&mut self) -> _TXDMAEW {
        _TXDMAEW { w: self }
    }
    #[doc = "Bit 3 - DMA on error.<ul><li>0: UART error interrupt status has no impact in receive DMA mode.</li><li>1: DMA receive requests are disabled when the UART error interrupt is asserted.</li></ul>"]
    #[inline]
    pub fn dmaonerr(&mut self) -> _DMAONERRW {
        _DMAONERRW { w: self }
    }
}
