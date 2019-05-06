#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MISR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TXFEMISR {
    bits: bool,
}
impl TXFEMISR {
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
pub struct TXFNEMISR {
    bits: bool,
}
impl TXFNEMISR {
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
pub struct TXFFMISR {
    bits: bool,
}
impl TXFFMISR {
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
pub struct TXFOVRMISR {
    bits: bool,
}
impl TXFOVRMISR {
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
pub struct RXFEMISR {
    bits: bool,
}
impl RXFEMISR {
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
pub struct RXFNFMISR {
    bits: bool,
}
impl RXFNFMISR {
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
pub struct RXFFMISR {
    bits: bool,
}
impl RXFFMISR {
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
pub struct LBRMISR {
    bits: bool,
}
impl LBRMISR {
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
pub struct RFSRMISR {
    bits: bool,
}
impl RFSRMISR {
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
pub struct RFSEMISR {
    bits: bool,
}
impl RFSEMISR {
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
pub struct WTSRMISR {
    bits: bool,
}
impl WTSRMISR {
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
pub struct MTDMISR {
    bits: bool,
}
impl MTDMISR {
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
pub struct STDMISR {
    bits: bool,
}
impl STDMISR {
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
pub struct SALMISR {
    bits: bool,
}
impl SALMISR {
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
pub struct MALMISR {
    bits: bool,
}
impl MALMISR {
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
pub struct BERRMISR {
    bits: bool,
}
impl BERRMISR {
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
pub struct MTDWSMISR {
    bits: bool,
}
impl MTDWSMISR {
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
pub struct TIMEOUTMISR {
    bits: bool,
}
impl TIMEOUTMISR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - TX FIFO empty masked interrupt status.<ul><li>0: TX FIFO is not empty.</li><li>1: TX FIFO is empty.</li></ul>"]
    #[inline]
    pub fn txfemis(&self) -> TXFEMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFEMISR { bits }
    }
    #[doc = "Bit 1 - TX FIFO nearly empty masked interrupt status.<ul><li>0: Number of entries in TX FIFO greater than the TFTR:THRESHOLD_TX register.</li><li>1: Number of entries in TX FIFO less than or equal to the TFTR:THRESHOLD_TX register.</li></ul>"]
    #[inline]
    pub fn txfnemis(&self) -> TXFNEMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFNEMISR { bits }
    }
    #[doc = "Bit 2 - Tx FIFO full masked interrupt status.<ul><li>0: TX FIFO is not full.</li><li>1: TX FIFO is full.</li></ul>"]
    #[inline]
    pub fn txffmis(&self) -> TXFFMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFFMISR { bits }
    }
    #[doc = "Bit 3 - Tx FIFO overrun masked interrupt status.<ul><li>0: No overrun condition occurred in TX FIFO.</li><li>1: Overrun condition occurred in TX FIFO.</li></ul>"]
    #[inline]
    pub fn txfovrmis(&self) -> TXFOVRMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFOVRMISR { bits }
    }
    #[doc = "Bit 4 - RX FIFO empty masked interrupt status.<ul><li>0: RX FIFO is not empty.</li><li>1: RX FIFO is empty..</li></ul>"]
    #[inline]
    pub fn rxfemis(&self) -> RXFEMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFEMISR { bits }
    }
    #[doc = "Bit 5 - RX FIFO nearly full masked interrupt status.<ul><li>0: Number of entries in the RX FIFO less than the RFTR:THRESHOLD_RX register.</li><li>1: Number of entries in the RX FIFO greater than or equal to the RFTR:THRESHOLD_RX register.</li></ul>"]
    #[inline]
    pub fn rxfnfmis(&self) -> RXFNFMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFNFMISR { bits }
    }
    #[doc = "Bit 6 - RX FIFO full masked interrupt status.<ul><li>0: RX FIFO is not full.</li><li>1: RX FIFO is full.</li></ul>"]
    #[inline]
    pub fn rxffmis(&self) -> RXFFMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFFMISR { bits }
    }
    #[doc = "Bit 15 - Length number of bytes received masked interrupt status.<ul><li>0: Length number of bytes is not received.</li><li>1: Length number of bytes is received.</li></ul>"]
    #[inline]
    pub fn lbrmis(&self) -> LBRMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LBRMISR { bits }
    }
    #[doc = "Bit 16 - Read-from-Slave request masked interrupt status.<ul><li>0: Read-from-slave request has been served.</li><li>1: Read-from-slave request is pending.</li></ul>"]
    #[inline]
    pub fn rfsrmis(&self) -> RFSRMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFSRMISR { bits }
    }
    #[doc = "Bit 17 - Read-from-Slave empty masked interrupt status.<ul><li>0: TX FIFO is not empty.</li><li>1: TX FIFO is empty with the read-from-slave operation in progress.</li></ul>"]
    #[inline]
    pub fn rfsemis(&self) -> RFSEMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFSEMISR { bits }
    }
    #[doc = "Bit 18 - Write-to-Slave request masked interrupt status.<ul><li>0: No write-to-slave request pending.</li><li>1: Write-to-slave request is pending.</li></ul>"]
    #[inline]
    pub fn wtsrmis(&self) -> WTSRMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WTSRMISR { bits }
    }
    #[doc = "Bit 19 - Master Transaction done masked interrupt status.<ul><li>0: Master transaction acknowledged.</li><li>1: Master transaction done (ready for acknowledgment).</li></ul>"]
    #[inline]
    pub fn mtdmis(&self) -> MTDMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MTDMISR { bits }
    }
    #[doc = "Bit 20 - Slave Transaction done masked interrupt status.<ul><li>0: Slave transaction acknowledged.</li><li>1: Slave transaction done (ready for acknowledgment).</li></ul>"]
    #[inline]
    pub fn stdmis(&self) -> STDMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STDMISR { bits }
    }
    #[doc = "Bit 23 - Slave Arbitration lost masked interrupt status.<ul><li>0: No slave arbitration lost.</li><li>1: Slave arbitration lost.</li></ul>"]
    #[inline]
    pub fn salmis(&self) -> SALMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SALMISR { bits }
    }
    #[doc = "Bit 24 - Master Arbitration lost masked interrupt status.<ul><li>0: No master arbitration lost.</li><li>1: Master arbitration lost.</li></ul>"]
    #[inline]
    pub fn malmis(&self) -> MALMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MALMISR { bits }
    }
    #[doc = "Bit 25 - Bus Error masked interrupt status.<ul><li>0: No bus error detection.</li><li>1: Bus error detection.</li></ul>"]
    #[inline]
    pub fn berrmis(&self) -> BERRMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BERRMISR { bits }
    }
    #[doc = "Bit 28 - Master Transaction done without stop masked interrupt status.<ul><li>0: Master transaction acknowledged.</li><li>1: Master transaction done (ready for acknowledgment) and stop is not applied into the I2C bus.</li></ul>"]
    #[inline]
    pub fn mtdwsmis(&self) -> MTDWSMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MTDWSMISR { bits }
    }
    #[doc = "Bit 30 - Timeout or Tlow error masked interrupt status.<ul><li>0: No timeout error.</li><li>1: SCL remained LOW for 25 ms (Timeout).</li></ul>"]
    #[inline]
    pub fn timeoutmis(&self) -> TIMEOUTMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIMEOUTMISR { bits }
    }
}
