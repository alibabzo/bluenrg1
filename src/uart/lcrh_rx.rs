#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::LCRH_RX {
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
#[doc = "Possible values of the field `PEN_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN_RXR {
    #[doc = "Parity Enable"]
    PARITY_ENABLE,
    #[doc = "Parity Disable"]
    PARITY_DISABLE,
}
impl PEN_RXR {
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
            PEN_RXR::PARITY_ENABLE => true,
            PEN_RXR::PARITY_DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN_RXR {
        match value {
            true => PEN_RXR::PARITY_ENABLE,
            false => PEN_RXR::PARITY_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PARITY_ENABLE`"]
    #[inline]
    pub fn is_parity_enable(&self) -> bool {
        *self == PEN_RXR::PARITY_ENABLE
    }
    #[doc = "Checks if the value of the field is `PARITY_DISABLE`"]
    #[inline]
    pub fn is_parity_disable(&self) -> bool {
        *self == PEN_RXR::PARITY_DISABLE
    }
}
#[doc = "Possible values of the field `EPS_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPS_RXR {
    #[doc = "Odd parity generation and checking is performed during reception, which check for an odd number of 1s in data and parity bits"]
    ODD,
    #[doc = "Even parity generation and checking is performed during reception, which check for an even number of 1s in data and parity bits"]
    EVEN,
}
impl EPS_RXR {
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
            EPS_RXR::ODD => false,
            EPS_RXR::EVEN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPS_RXR {
        match value {
            false => EPS_RXR::ODD,
            true => EPS_RXR::EVEN,
        }
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == EPS_RXR::ODD
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == EPS_RXR::EVEN
    }
}
#[doc = "Possible values of the field `STP2_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STP2_RXR {
    #[doc = "1 stop bit received"]
    STOP_BIT1,
    #[doc = "2 stop bits received"]
    STOP_BITS2,
}
impl STP2_RXR {
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
            STP2_RXR::STOP_BIT1 => false,
            STP2_RXR::STOP_BITS2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STP2_RXR {
        match value {
            false => STP2_RXR::STOP_BIT1,
            true => STP2_RXR::STOP_BITS2,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_BIT1`"]
    #[inline]
    pub fn is_stop_bit1(&self) -> bool {
        *self == STP2_RXR::STOP_BIT1
    }
    #[doc = "Checks if the value of the field is `STOP_BITS2`"]
    #[inline]
    pub fn is_stop_bits2(&self) -> bool {
        *self == STP2_RXR::STOP_BITS2
    }
}
#[doc = "Possible values of the field `FEN_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEN_RXR {
    #[doc = "RX FIFO is disabled"]
    RXFIFO_DISABLED,
    #[doc = "RX FIFO is enabled"]
    RXFIFO_ENABLED,
}
impl FEN_RXR {
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
            FEN_RXR::RXFIFO_DISABLED => false,
            FEN_RXR::RXFIFO_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEN_RXR {
        match value {
            false => FEN_RXR::RXFIFO_DISABLED,
            true => FEN_RXR::RXFIFO_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFO_DISABLED`"]
    #[inline]
    pub fn is_rxfifo_disabled(&self) -> bool {
        *self == FEN_RXR::RXFIFO_DISABLED
    }
    #[doc = "Checks if the value of the field is `RXFIFO_ENABLED`"]
    #[inline]
    pub fn is_rxfifo_enabled(&self) -> bool {
        *self == FEN_RXR::RXFIFO_ENABLED
    }
}
#[doc = "Possible values of the field `WLEN_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLEN_RXR {
    #[doc = "5 bits"]
    BIT5,
    #[doc = "6 bits"]
    BIT6,
    #[doc = "7 bits"]
    BIT7,
    #[doc = "8 bits"]
    BIT8,
}
impl WLEN_RXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WLEN_RXR::BIT5 => 0,
            WLEN_RXR::BIT6 => 1,
            WLEN_RXR::BIT7 => 2,
            WLEN_RXR::BIT8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WLEN_RXR {
        match value {
            0 => WLEN_RXR::BIT5,
            1 => WLEN_RXR::BIT6,
            2 => WLEN_RXR::BIT7,
            3 => WLEN_RXR::BIT8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIT5`"]
    #[inline]
    pub fn is_bit5(&self) -> bool {
        *self == WLEN_RXR::BIT5
    }
    #[doc = "Checks if the value of the field is `BIT6`"]
    #[inline]
    pub fn is_bit6(&self) -> bool {
        *self == WLEN_RXR::BIT6
    }
    #[doc = "Checks if the value of the field is `BIT7`"]
    #[inline]
    pub fn is_bit7(&self) -> bool {
        *self == WLEN_RXR::BIT7
    }
    #[doc = "Checks if the value of the field is `BIT8`"]
    #[inline]
    pub fn is_bit8(&self) -> bool {
        *self == WLEN_RXR::BIT8
    }
}
#[doc = "Possible values of the field `SPS_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPS_RXR {
    #[doc = "stick parity disable"]
    STICK_PARITY_DISABLE,
    #[doc = "stick parity enable"]
    STICK_PARITY_ENABLE,
}
impl SPS_RXR {
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
            SPS_RXR::STICK_PARITY_DISABLE => false,
            SPS_RXR::STICK_PARITY_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPS_RXR {
        match value {
            false => SPS_RXR::STICK_PARITY_DISABLE,
            true => SPS_RXR::STICK_PARITY_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `STICK_PARITY_DISABLE`"]
    #[inline]
    pub fn is_stick_parity_disable(&self) -> bool {
        *self == SPS_RXR::STICK_PARITY_DISABLE
    }
    #[doc = "Checks if the value of the field is `STICK_PARITY_ENABLE`"]
    #[inline]
    pub fn is_stick_parity_enable(&self) -> bool {
        *self == SPS_RXR::STICK_PARITY_ENABLE
    }
}
#[doc = "Values that can be written to the field `PEN_RX`"]
pub enum PEN_RXW {
    #[doc = "Parity Enable"]
    PARITY_ENABLE,
    #[doc = "Parity Disable"]
    PARITY_DISABLE,
}
impl PEN_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN_RXW::PARITY_ENABLE => true,
            PEN_RXW::PARITY_DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _PEN_RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN_RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Parity Enable"]
    #[inline]
    pub fn parity_enable(self) -> &'a mut W {
        self.variant(PEN_RXW::PARITY_ENABLE)
    }
    #[doc = "Parity Disable"]
    #[inline]
    pub fn parity_disable(self) -> &'a mut W {
        self.variant(PEN_RXW::PARITY_DISABLE)
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
#[doc = "Values that can be written to the field `EPS_RX`"]
pub enum EPS_RXW {
    #[doc = "Odd parity generation and checking is performed during reception, which check for an odd number of 1s in data and parity bits"]
    ODD,
    #[doc = "Even parity generation and checking is performed during reception, which check for an even number of 1s in data and parity bits"]
    EVEN,
}
impl EPS_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPS_RXW::ODD => false,
            EPS_RXW::EVEN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPS_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _EPS_RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPS_RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Odd parity generation and checking is performed during reception, which check for an odd number of 1s in data and parity bits"]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(EPS_RXW::ODD)
    }
    #[doc = "Even parity generation and checking is performed during reception, which check for an even number of 1s in data and parity bits"]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(EPS_RXW::EVEN)
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
#[doc = "Values that can be written to the field `STP2_RX`"]
pub enum STP2_RXW {
    #[doc = "1 stop bit received"]
    STOP_BIT1,
    #[doc = "2 stop bits received"]
    STOP_BITS2,
}
impl STP2_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STP2_RXW::STOP_BIT1 => false,
            STP2_RXW::STOP_BITS2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STP2_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _STP2_RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STP2_RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1 stop bit received"]
    #[inline]
    pub fn stop_bit1(self) -> &'a mut W {
        self.variant(STP2_RXW::STOP_BIT1)
    }
    #[doc = "2 stop bits received"]
    #[inline]
    pub fn stop_bits2(self) -> &'a mut W {
        self.variant(STP2_RXW::STOP_BITS2)
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
#[doc = "Values that can be written to the field `FEN_RX`"]
pub enum FEN_RXW {
    #[doc = "RX FIFO is disabled"]
    RXFIFO_DISABLED,
    #[doc = "RX FIFO is enabled"]
    RXFIFO_ENABLED,
}
impl FEN_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEN_RXW::RXFIFO_DISABLED => false,
            FEN_RXW::RXFIFO_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEN_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _FEN_RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEN_RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX FIFO is disabled"]
    #[inline]
    pub fn rxfifo_disabled(self) -> &'a mut W {
        self.variant(FEN_RXW::RXFIFO_DISABLED)
    }
    #[doc = "RX FIFO is enabled"]
    #[inline]
    pub fn rxfifo_enabled(self) -> &'a mut W {
        self.variant(FEN_RXW::RXFIFO_ENABLED)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WLEN_RX`"]
pub enum WLEN_RXW {
    #[doc = "5 bits"]
    BIT5,
    #[doc = "6 bits"]
    BIT6,
    #[doc = "7 bits"]
    BIT7,
    #[doc = "8 bits"]
    BIT8,
}
impl WLEN_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WLEN_RXW::BIT5 => 0,
            WLEN_RXW::BIT6 => 1,
            WLEN_RXW::BIT7 => 2,
            WLEN_RXW::BIT8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WLEN_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _WLEN_RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WLEN_RXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "5 bits"]
    #[inline]
    pub fn bit5(self) -> &'a mut W {
        self.variant(WLEN_RXW::BIT5)
    }
    #[doc = "6 bits"]
    #[inline]
    pub fn bit6(self) -> &'a mut W {
        self.variant(WLEN_RXW::BIT6)
    }
    #[doc = "7 bits"]
    #[inline]
    pub fn bit7(self) -> &'a mut W {
        self.variant(WLEN_RXW::BIT7)
    }
    #[doc = "8 bits"]
    #[inline]
    pub fn bit8(self) -> &'a mut W {
        self.variant(WLEN_RXW::BIT8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPS_RX`"]
pub enum SPS_RXW {
    #[doc = "stick parity disable"]
    STICK_PARITY_DISABLE,
    #[doc = "stick parity enable"]
    STICK_PARITY_ENABLE,
}
impl SPS_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPS_RXW::STICK_PARITY_DISABLE => false,
            SPS_RXW::STICK_PARITY_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPS_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _SPS_RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPS_RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stick parity disable"]
    #[inline]
    pub fn stick_parity_disable(self) -> &'a mut W {
        self.variant(SPS_RXW::STICK_PARITY_DISABLE)
    }
    #[doc = "stick parity enable"]
    #[inline]
    pub fn stick_parity_enable(self) -> &'a mut W {
        self.variant(SPS_RXW::STICK_PARITY_ENABLE)
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 1 - RX parity enable:<ul><li>0: Parity disabled.</li><li>1: Parity enabled.</li></ul>"]
    #[inline]
    pub fn pen_rx(&self) -> PEN_RXR {
        PEN_RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - RX even parity selection, when the parity is enabled.<ul><li>0: Odd parity generation and checking is performed during reception, which check for an odd number of 1s in data and parity bits.</li><li>1: Even parity generation and checking is performed during reception, which check for an even number of 1s in data and parity bits.</li></ul>"]
    #[inline]
    pub fn eps_rx(&self) -> EPS_RXR {
        EPS_RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - RX two stop bits select. This bit enables the check for two stop bits being received:<ul><li>0: 1 stop bit received.</li><li>1: 2 stop bits received.</li></ul>"]
    #[inline]
    pub fn stp2_rx(&self) -> STP2_RXR {
        STP2_RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - RX enable FIFOs. This bit enables/disables the receive RX FIFO buffer:<ul><li>0: RX FIFO is disabled (character mode).</li><li>1: RX FIFO is enabled.</li></ul>"]
    #[inline]
    pub fn fen_rx(&self) -> FEN_RXR {
        FEN_RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 5:6 - RX Word length. This bit field indicates the number of data bits received in a frame as follows:<ul><li>00b: 5 bits.</li><li>01b: 6 bits.</li><li>10b: 7 bits.</li><li>11b: 8 bits.</li></ul>"]
    #[inline]
    pub fn wlen_rx(&self) -> WLEN_RXR {
        WLEN_RXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 7 - RX stick parity select:<ul><li>0: stick parity is disabled.</li><li>1: when PEN_RX = 1b (parity enabled) and EPS_RX = 1b (even parity), the parity is checked as a 0. When PEN_RX = 1b and EPS_RX = 0b (odd parity), the parity bit is checked as a 1.</li></ul>"]
    #[inline]
    pub fn sps_rx(&self) -> SPS_RXR {
        SPS_RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 1 - RX parity enable:<ul><li>0: Parity disabled.</li><li>1: Parity enabled.</li></ul>"]
    #[inline]
    pub fn pen_rx(&mut self) -> _PEN_RXW {
        _PEN_RXW { w: self }
    }
    #[doc = "Bit 2 - RX even parity selection, when the parity is enabled.<ul><li>0: Odd parity generation and checking is performed during reception, which check for an odd number of 1s in data and parity bits.</li><li>1: Even parity generation and checking is performed during reception, which check for an even number of 1s in data and parity bits.</li></ul>"]
    #[inline]
    pub fn eps_rx(&mut self) -> _EPS_RXW {
        _EPS_RXW { w: self }
    }
    #[doc = "Bit 3 - RX two stop bits select. This bit enables the check for two stop bits being received:<ul><li>0: 1 stop bit received.</li><li>1: 2 stop bits received.</li></ul>"]
    #[inline]
    pub fn stp2_rx(&mut self) -> _STP2_RXW {
        _STP2_RXW { w: self }
    }
    #[doc = "Bit 4 - RX enable FIFOs. This bit enables/disables the receive RX FIFO buffer:<ul><li>0: RX FIFO is disabled (character mode).</li><li>1: RX FIFO is enabled.</li></ul>"]
    #[inline]
    pub fn fen_rx(&mut self) -> _FEN_RXW {
        _FEN_RXW { w: self }
    }
    #[doc = "Bits 5:6 - RX Word length. This bit field indicates the number of data bits received in a frame as follows:<ul><li>00b: 5 bits.</li><li>01b: 6 bits.</li><li>10b: 7 bits.</li><li>11b: 8 bits.</li></ul>"]
    #[inline]
    pub fn wlen_rx(&mut self) -> _WLEN_RXW {
        _WLEN_RXW { w: self }
    }
    #[doc = "Bit 7 - RX stick parity select:<ul><li>0: stick parity is disabled.</li><li>1: when PEN_RX = 1b (parity enabled) and EPS_RX = 1b (even parity), the parity is checked as a 0. When PEN_RX = 1b and EPS_RX = 0b (odd parity), the parity bit is checked as a 1.</li></ul>"]
    #[inline]
    pub fn sps_rx(&mut self) -> _SPS_RXW {
        _SPS_RXW { w: self }
    }
}
