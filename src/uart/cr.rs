#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
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
#[doc = "Possible values of the field `UARTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTENR {
    #[doc = "Enable the UART"]
    ENABLE,
    #[doc = "Disable the UART"]
    DISABLE,
}
impl UARTENR {
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
            UARTENR::ENABLE => true,
            UARTENR::DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UARTENR {
        match value {
            true => UARTENR::ENABLE,
            false => UARTENR::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == UARTENR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == UARTENR::DISABLE
    }
}
#[doc = "Possible values of the field `OVSFACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVSFACTR {
    #[doc = "16 UARTCLK clock cycles"]
    CYCLES_16,
    #[doc = "8 UARTCLK clock cycles"]
    CYCLES_8,
}
impl OVSFACTR {
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
            OVSFACTR::CYCLES_16 => false,
            OVSFACTR::CYCLES_8 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVSFACTR {
        match value {
            false => OVSFACTR::CYCLES_16,
            true => OVSFACTR::CYCLES_8,
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES_16`"]
    #[inline]
    pub fn is_cycles_16(&self) -> bool {
        *self == OVSFACTR::CYCLES_16
    }
    #[doc = "Checks if the value of the field is `CYCLES_8`"]
    #[inline]
    pub fn is_cycles_8(&self) -> bool {
        *self == OVSFACTR::CYCLES_8
    }
}
#[doc = "Possible values of the field `TXE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXER {
    #[doc = "Enable the TX UART"]
    TX_ENABLE,
    #[doc = "Disable the TX UART"]
    TX_DISABLE,
}
impl TXER {
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
            TXER::TX_ENABLE => true,
            TXER::TX_DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXER {
        match value {
            true => TXER::TX_ENABLE,
            false => TXER::TX_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `TX_ENABLE`"]
    #[inline]
    pub fn is_tx_enable(&self) -> bool {
        *self == TXER::TX_ENABLE
    }
    #[doc = "Checks if the value of the field is `TX_DISABLE`"]
    #[inline]
    pub fn is_tx_disable(&self) -> bool {
        *self == TXER::TX_DISABLE
    }
}
#[doc = "Possible values of the field `RXE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXER {
    #[doc = "Enable the RX UART"]
    RX_ENABLE,
    #[doc = "Disable the RX UART"]
    RX_DISABLE,
}
impl RXER {
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
            RXER::RX_ENABLE => true,
            RXER::RX_DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXER {
        match value {
            true => RXER::RX_ENABLE,
            false => RXER::RX_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `RX_ENABLE`"]
    #[inline]
    pub fn is_rx_enable(&self) -> bool {
        *self == RXER::RX_ENABLE
    }
    #[doc = "Checks if the value of the field is `RX_DISABLE`"]
    #[inline]
    pub fn is_rx_disable(&self) -> bool {
        *self == RXER::RX_DISABLE
    }
}
#[doc = "Possible values of the field `RTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSR {
    #[doc = "request to send high"]
    REQUEST_TO_SEND_HIGH,
    #[doc = "request to send low"]
    REQUEST_TO_SEND_LOW,
}
impl RTSR {
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
            RTSR::REQUEST_TO_SEND_HIGH => true,
            RTSR::REQUEST_TO_SEND_LOW => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTSR {
        match value {
            true => RTSR::REQUEST_TO_SEND_HIGH,
            false => RTSR::REQUEST_TO_SEND_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `REQUEST_TO_SEND_HIGH`"]
    #[inline]
    pub fn is_request_to_send_high(&self) -> bool {
        *self == RTSR::REQUEST_TO_SEND_HIGH
    }
    #[doc = "Checks if the value of the field is `REQUEST_TO_SEND_LOW`"]
    #[inline]
    pub fn is_request_to_send_low(&self) -> bool {
        *self == RTSR::REQUEST_TO_SEND_LOW
    }
}
#[doc = "Possible values of the field `RTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSENR {
    #[doc = "RTS hardware flow control disable"]
    RTS_DISABLE,
    #[doc = "RTS hardware flow control enable"]
    RTS_ENABLE,
}
impl RTSENR {
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
            RTSENR::RTS_DISABLE => false,
            RTSENR::RTS_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTSENR {
        match value {
            false => RTSENR::RTS_DISABLE,
            true => RTSENR::RTS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `RTS_DISABLE`"]
    #[inline]
    pub fn is_rts_disable(&self) -> bool {
        *self == RTSENR::RTS_DISABLE
    }
    #[doc = "Checks if the value of the field is `RTS_ENABLE`"]
    #[inline]
    pub fn is_rts_enable(&self) -> bool {
        *self == RTSENR::RTS_ENABLE
    }
}
#[doc = "Possible values of the field `CTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSENR {
    #[doc = "CTS hardware flow control disable"]
    CTS_DISABLE,
    #[doc = "CTS hardware flow control enable"]
    CTS_ENABLE,
}
impl CTSENR {
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
            CTSENR::CTS_DISABLE => false,
            CTSENR::CTS_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSENR {
        match value {
            false => CTSENR::CTS_DISABLE,
            true => CTSENR::CTS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CTS_DISABLE`"]
    #[inline]
    pub fn is_cts_disable(&self) -> bool {
        *self == CTSENR::CTS_DISABLE
    }
    #[doc = "Checks if the value of the field is `CTS_ENABLE`"]
    #[inline]
    pub fn is_cts_enable(&self) -> bool {
        *self == CTSENR::CTS_ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct STA_B_DURATIONR {
    bits: u8,
}
impl STA_B_DURATIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `UARTEN`"]
pub enum UARTENW {
    #[doc = "Enable the UART"]
    ENABLE,
    #[doc = "Disable the UART"]
    DISABLE,
}
impl UARTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UARTENW::ENABLE => true,
            UARTENW::DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UARTENW<'a> {
    w: &'a mut W,
}
impl<'a> _UARTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UARTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the UART"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(UARTENW::ENABLE)
    }
    #[doc = "Disable the UART"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(UARTENW::DISABLE)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OVSFACT`"]
pub enum OVSFACTW {
    #[doc = "16 UARTCLK clock cycles"]
    CYCLES_16,
    #[doc = "8 UARTCLK clock cycles"]
    CYCLES_8,
}
impl OVSFACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVSFACTW::CYCLES_16 => false,
            OVSFACTW::CYCLES_8 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVSFACTW<'a> {
    w: &'a mut W,
}
impl<'a> _OVSFACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVSFACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "16 UARTCLK clock cycles"]
    #[inline]
    pub fn cycles_16(self) -> &'a mut W {
        self.variant(OVSFACTW::CYCLES_16)
    }
    #[doc = "8 UARTCLK clock cycles"]
    #[inline]
    pub fn cycles_8(self) -> &'a mut W {
        self.variant(OVSFACTW::CYCLES_8)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXE`"]
pub enum TXEW {
    #[doc = "Enable the TX UART"]
    TX_ENABLE,
    #[doc = "Disable the TX UART"]
    TX_DISABLE,
}
impl TXEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXEW::TX_ENABLE => true,
            TXEW::TX_DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the TX UART"]
    #[inline]
    pub fn tx_enable(self) -> &'a mut W {
        self.variant(TXEW::TX_ENABLE)
    }
    #[doc = "Disable the TX UART"]
    #[inline]
    pub fn tx_disable(self) -> &'a mut W {
        self.variant(TXEW::TX_DISABLE)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXE`"]
pub enum RXEW {
    #[doc = "Enable the RX UART"]
    RX_ENABLE,
    #[doc = "Disable the RX UART"]
    RX_DISABLE,
}
impl RXEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXEW::RX_ENABLE => true,
            RXEW::RX_DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the RX UART"]
    #[inline]
    pub fn rx_enable(self) -> &'a mut W {
        self.variant(RXEW::RX_ENABLE)
    }
    #[doc = "Disable the RX UART"]
    #[inline]
    pub fn rx_disable(self) -> &'a mut W {
        self.variant(RXEW::RX_DISABLE)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTS`"]
pub enum RTSW {
    #[doc = "request to send high"]
    REQUEST_TO_SEND_HIGH,
    #[doc = "request to send low"]
    REQUEST_TO_SEND_LOW,
}
impl RTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTSW::REQUEST_TO_SEND_HIGH => true,
            RTSW::REQUEST_TO_SEND_LOW => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTSW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "request to send high"]
    #[inline]
    pub fn request_to_send_high(self) -> &'a mut W {
        self.variant(RTSW::REQUEST_TO_SEND_HIGH)
    }
    #[doc = "request to send low"]
    #[inline]
    pub fn request_to_send_low(self) -> &'a mut W {
        self.variant(RTSW::REQUEST_TO_SEND_LOW)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTSEN`"]
pub enum RTSENW {
    #[doc = "RTS hardware flow control disable"]
    RTS_DISABLE,
    #[doc = "RTS hardware flow control enable"]
    RTS_ENABLE,
}
impl RTSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTSENW::RTS_DISABLE => false,
            RTSENW::RTS_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTS hardware flow control disable"]
    #[inline]
    pub fn rts_disable(self) -> &'a mut W {
        self.variant(RTSENW::RTS_DISABLE)
    }
    #[doc = "RTS hardware flow control enable"]
    #[inline]
    pub fn rts_enable(self) -> &'a mut W {
        self.variant(RTSENW::RTS_ENABLE)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTSEN`"]
pub enum CTSENW {
    #[doc = "CTS hardware flow control disable"]
    CTS_DISABLE,
    #[doc = "CTS hardware flow control enable"]
    CTS_ENABLE,
}
impl CTSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSENW::CTS_DISABLE => false,
            CTSENW::CTS_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CTS hardware flow control disable"]
    #[inline]
    pub fn cts_disable(self) -> &'a mut W {
        self.variant(CTSENW::CTS_DISABLE)
    }
    #[doc = "CTS hardware flow control enable"]
    #[inline]
    pub fn cts_enable(self) -> &'a mut W {
        self.variant(CTSENW::CTS_ENABLE)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STA_B_DURATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _STA_B_DURATIONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - UART enable. This bit enables the UART.<ul><li>0: UART is disabled.</li><li>1: UART is enabled. Data transmission and reception can occur. When the UART is disabled in the middle of transmission or reception, it completes the current character before stopping.</li></ul>"]
    #[inline]
    pub fn uarten(&self) -> UARTENR {
        UARTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - UART oversampling factor.This bit enables the UART oversampling factor. If UARTCLK is 16 MHz thus max. baud-rate is 1 Mbaud when OVSFACT = 0b, and 2 Mbaud when OVSFACT = 1b.<ul><li>0: UART it is 16 UARTCLK clock cycles.</li><li>1: UART it is 8 UARTCLK clock cycles.</li></ul>"]
    #[inline]
    pub fn ovsfact(&self) -> OVSFACTR {
        OVSFACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Transmit enable.<ul><li>0b: UART TX disabled.</li><li>1b: UART TX enabled.</li></ul>"]
    #[inline]
    pub fn txe(&self) -> TXER {
        TXER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Receive enable.<ul><li>0b: UART RX disabled.</li><li>1b: UART RX enabled.</li></ul>"]
    #[inline]
    pub fn rxe(&self) -> RXER {
        RXER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Request to send.<ul><li>0: RTS is high.</li><li>1: RTS is low.</li></ul>"]
    #[inline]
    pub fn rts(&self) -> RTSR {
        RTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - RTS hardware flow control enable.<ul><li>0b: RTS disabled.</li><li>1b: RTS enabled. Data is only requested when there is space in the receive FIFO for it to be received.</li></ul>"]
    #[inline]
    pub fn rtsen(&self) -> RTSENR {
        RTSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - CTS hardware flow control enable.<ul><li>0b: CTS disabled.</li><li>1b: CTS enabled. Data is only transmitted when the CTS is asserted.</li></ul>"]
    #[inline]
    pub fn ctsen(&self) -> CTSENR {
        CTSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - START bit duration Receiver state. These bits can be used to configure the START bit duration (in clock cycles) to get the bit sampled in the middle of the UART receiver. These bits can be used only when using high baud rates (IBRD = 1, FBRD >= 0 and OVSFACT = 1). Below the formula to calculate the START bit duration receiver state:<p>STA_B_DURATION = Integer(Fuartclk/(2* BAUD RATE)) - 1</p>Example: when UARTCLK = 16 MHz and BAUD RATE = 2.0 Mbps then STA_B_DURATION = 4 - 1 = 3. STA_B_DURATION field should be configured with 4'b0011."]
    #[inline]
    pub fn sta_b_duration(&self) -> STA_B_DURATIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STA_B_DURATIONR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 262912 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - UART enable. This bit enables the UART.<ul><li>0: UART is disabled.</li><li>1: UART is enabled. Data transmission and reception can occur. When the UART is disabled in the middle of transmission or reception, it completes the current character before stopping.</li></ul>"]
    #[inline]
    pub fn uarten(&mut self) -> _UARTENW {
        _UARTENW { w: self }
    }
    #[doc = "Bit 3 - UART oversampling factor.This bit enables the UART oversampling factor. If UARTCLK is 16 MHz thus max. baud-rate is 1 Mbaud when OVSFACT = 0b, and 2 Mbaud when OVSFACT = 1b.<ul><li>0: UART it is 16 UARTCLK clock cycles.</li><li>1: UART it is 8 UARTCLK clock cycles.</li></ul>"]
    #[inline]
    pub fn ovsfact(&mut self) -> _OVSFACTW {
        _OVSFACTW { w: self }
    }
    #[doc = "Bit 8 - Transmit enable.<ul><li>0b: UART TX disabled.</li><li>1b: UART TX enabled.</li></ul>"]
    #[inline]
    pub fn txe(&mut self) -> _TXEW {
        _TXEW { w: self }
    }
    #[doc = "Bit 9 - Receive enable.<ul><li>0b: UART RX disabled.</li><li>1b: UART RX enabled.</li></ul>"]
    #[inline]
    pub fn rxe(&mut self) -> _RXEW {
        _RXEW { w: self }
    }
    #[doc = "Bit 11 - Request to send.<ul><li>0: RTS is high.</li><li>1: RTS is low.</li></ul>"]
    #[inline]
    pub fn rts(&mut self) -> _RTSW {
        _RTSW { w: self }
    }
    #[doc = "Bit 14 - RTS hardware flow control enable.<ul><li>0b: RTS disabled.</li><li>1b: RTS enabled. Data is only requested when there is space in the receive FIFO for it to be received.</li></ul>"]
    #[inline]
    pub fn rtsen(&mut self) -> _RTSENW {
        _RTSENW { w: self }
    }
    #[doc = "Bit 15 - CTS hardware flow control enable.<ul><li>0b: CTS disabled.</li><li>1b: CTS enabled. Data is only transmitted when the CTS is asserted.</li></ul>"]
    #[inline]
    pub fn ctsen(&mut self) -> _CTSENW {
        _CTSENW { w: self }
    }
    #[doc = "Bits 16:19 - START bit duration Receiver state. These bits can be used to configure the START bit duration (in clock cycles) to get the bit sampled in the middle of the UART receiver. These bits can be used only when using high baud rates (IBRD = 1, FBRD >= 0 and OVSFACT = 1). Below the formula to calculate the START bit duration receiver state:<p>STA_B_DURATION = Integer(Fuartclk/(2* BAUD RATE)) - 1</p>Example: when UARTCLK = 16 MHz and BAUD RATE = 2.0 Mbps then STA_B_DURATION = 4 - 1 = 3. STA_B_DURATION field should be configured with 4'b0011."]
    #[inline]
    pub fn sta_b_duration(&mut self) -> _STA_B_DURATIONW {
        _STA_B_DURATIONW { w: self }
    }
}
