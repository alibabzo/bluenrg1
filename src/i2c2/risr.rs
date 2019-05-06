#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RISR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TXFER {
    bits: bool,
}
impl TXFER {
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
pub struct TXFNER {
    bits: bool,
}
impl TXFNER {
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
pub struct TXFFR {
    bits: bool,
}
impl TXFFR {
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
pub struct TXFOVRR {
    bits: bool,
}
impl TXFOVRR {
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
pub struct RXFER {
    bits: bool,
}
impl RXFER {
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
pub struct RXFNFR {
    bits: bool,
}
impl RXFNFR {
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
pub struct RXFFR {
    bits: bool,
}
impl RXFFR {
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
pub struct LBRR {
    bits: bool,
}
impl LBRR {
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
pub struct RFSRR {
    bits: bool,
}
impl RFSRR {
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
pub struct RFSER {
    bits: bool,
}
impl RFSER {
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
pub struct WTSRR {
    bits: bool,
}
impl WTSRR {
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
pub struct MTDR {
    bits: bool,
}
impl MTDR {
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
pub struct STDR {
    bits: bool,
}
impl STDR {
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
pub struct SALR {
    bits: bool,
}
impl SALR {
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
pub struct MALR {
    bits: bool,
}
impl MALR {
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
pub struct BERRR {
    bits: bool,
}
impl BERRR {
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
pub struct MTDWSR {
    bits: bool,
}
impl MTDWSR {
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
    #[doc = "Bit 0 - TX FIFO empty. TXFE is set when TX FIFO is empty. This bit is self-cleared by writing in TX FIFO.<ul><li>0: TX FIFO is not empty.</li><li>1: TX FIFO is empty.</li></ul>"]
    #[inline]
    pub fn txfe(&self) -> TXFER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFER { bits }
    }
    #[doc = "Bit 1 - TX FIFO nearly empty. TXFNE is set when the number of entries in TX FIFO is less than or equal to the threshold value programmed in the I2C_TFTR:THRESHOLD_TX register. It is self-cleared when the threshold level is over the programmed threshold.<ul><li>0: Number of entries in TX FIFO greater than the TFTR:THRESHOLD_TX register.</li><li>1: Number of entries in TX FIFO less than or equal to the TFTR:THRESHOLD_TX register.</li></ul>"]
    #[inline]
    pub fn txfne(&self) -> TXFNER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFNER { bits }
    }
    #[doc = "Bit 2 - TX FIFO full. TXFF is set when a full condition occurs in TX FIFO. This bit is self-cleared when the TX FIFO is not full:<ul><li>0: TX FIFO is not full.</li><li>1: TX FIFO is full.</li></ul>"]
    #[inline]
    pub fn txff(&self) -> TXFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFFR { bits }
    }
    #[doc = "Bit 3 - TX FIFO overrun. TXFOVR is set when a write operation in TX FIFO is performed and TX FIFO is full. The application must avoid an overflow condition by a proper data flow control. Anyway in case of overrun, the application shall flush the transmitter (CR:FTX bit to set) because the TX FIFO content is corrupted (at least one word has been lost in FIFO). This interrupt is cleared by setting the related bit of the ICR register:<ul><li>0: No overrun condition occurred in TX FIFO.</li><li>1: Overrun condition occurred in TX FIFO.</li></ul>"]
    #[inline]
    pub fn txfovr(&self) -> TXFOVRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFOVRR { bits }
    }
    #[doc = "Bit 4 - RX FIFO empty. RXFE is set when the RX FIFO is empty. This bit is self-cleared when the slave RX FIFO is not empty:<ul><li>0: RX FIFO is not empty..</li><li>1: RX FIFO is empty..</li></ul>"]
    #[inline]
    pub fn rxfe(&self) -> RXFER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFER { bits }
    }
    #[doc = "Bit 5 - RX FIFO nearly full. RXFNF is set when the number of entries in RX FIFO is greater than or equal to the threshold value programmed in the RFTR:THRESHOLD_RX register. Its self-cleared when the threshold level is under the programmed threshold:<ul><li>0: Number of entries in the RX FIFO less than the RFTR:THRESHOLD_RX register.</li><li>1: Number of entries in the RX FIFO greater than or equal to the RFTR:THRESHOLD_RX register.</li></ul>"]
    #[inline]
    pub fn rxfnf(&self) -> RXFNFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFNFR { bits }
    }
    #[doc = "Bit 6 - RX FIFO full. RXFF is set when a full condition occurs in RX FIFO. This bit is self-cleared when the data are read from the RX FIFO.<ul><li>0: RX FIFO is not full.</li><li>1: RX FIFO is full.</li></ul>"]
    #[inline]
    pub fn rxff(&self) -> RXFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFFR { bits }
    }
    #[doc = "Bit 15 - Length number of bytes received. LBR is set in case of MR or WTS and when the number of bytes received is equal to the transaction length programmed in the MCR:LENGTH (master mode) or SMB_SCR:LENGTH (slave mode). On the assertion of this interrupt and when the bit CR:FRC_STRTCH is set, the hardware starts clock stretching, the CPU shall download the data byte (Command code, Byte Count, Data...) from RX FIFO, re-set the expected length of the transaction in SMB_SCR:LENGTH and clear the interrupt. When clearing this interrupt the hardware continues the transfer. This interrupt is cleared by setting the related bit of the ICR register.<ul><li>0: Length number of bytes is not received.</li><li>1: Length number of bytes is received.</li></ul>"]
    #[inline]
    pub fn lbr(&self) -> LBRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LBRR { bits }
    }
    #[doc = "Bit 16 - Read-from-slave request. RFSR is set when a read-from-slave \"Slavetransmitter\" request is received (I2C slave is addressed) from the I2C line. On the assertion of this interrupt the TX FIFO is flushed (pending data are cleared) and the CPU shall put the data in TX FIFO. This bit is self-cleared by writing data in FIFO. In case the FIFO is empty before the completion of the read operation, the RISR:RFSE interrupt bit is set.This interrupt is cleared by setting the related bit of the ICR register.<ul><li>0: Read-from-slave request has been served.</li><li>1: Read-from-slave request is pending.</li></ul>"]
    #[inline]
    pub fn rfsr(&self) -> RFSRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFSRR { bits }
    }
    #[doc = "Bit 17 - Read-from-Slave empty. RFSE is set when a read-from-slave operation is in progress and TX FIFO is empty. On the assertion of this interrupt, the CPU shall download in TX FIFO the data required for the slave operation. This bit is self-cleared by writing in TX FIFO. At the end of the read-from-slave operation this bit is cleared although the TX FIFO is empty.<ul><li>0: TX FIFO is not empty.</li><li>1: TX FIFO is empty with the read-from-slave operation in progress.</li></ul>"]
    #[inline]
    pub fn rfse(&self) -> RFSER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFSER { bits }
    }
    #[doc = "Bit 18 - Write-to-Slave request. WTSR is set when a write-to-slave operation is received (I2C slave is addressed) from the I2C line. This notification can be used by the application to program the DMA descriptor when required. This interrupt is cleared by setting the related bit of the ICR register:<ul><li>0: No write-to-slave request pending.</li><li>1: Write-to-slave request is pending.</li></ul>"]
    #[inline]
    pub fn wtsr(&self) -> WTSRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WTSRR { bits }
    }
    #[doc = "Bit 19 - Master Transaction done. MTD is set when a master operation (master write or master read) has been executed after a stop condition. The application shall read the related transaction status (SR register), the pending data in the RX FIFO (only for a master read operation) and clear this interrupt (transaction acknowledgment). A subsequent master operation can be issued (writing the MCR register) after the clearing of this interrupt. A subsequent slave operation will be notified (RISR:WTSR and RISR:RFSR interrupt bits assertion) after clearing this interrupt, meanwhile the I2C clock line will be stretched low. This interrupt is cleared by setting the related bit of the ICR register.<ul><li>0: Master transaction acknowledged.</li><li>1: Master transaction done (ready for acknowledgment).</li></ul>"]
    #[inline]
    pub fn mtd(&self) -> MTDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MTDR { bits }
    }
    #[doc = "Bit 20 - Slave Transaction done. STD is set when a slave operation (write-to-slave or read-from-slave) has been executed. The application shall read the related transaction status (SR register), the pending data in the RX FIFO (only for a write-to-slave operation) and clear this interrupt (transaction acknowledgment). A subsequent slave operation will be notified (RISR:WTSR and RISR:RFSR interrupt bits assertion) after clearing this interrupt, meanwhile the I2C clock line will be stretched low. A subsequent master operation can be issued (by writing the MCR register) after clearing this interrupt. This interrupt is cleared by setting the related bit of the ICR register:<ul><li>0: Slave transaction acknowledged.</li><li>1: Slave transaction done (ready for acknowledgment).</li></ul>"]
    #[inline]
    pub fn std(&self) -> STDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STDR { bits }
    }
    #[doc = "Bit 23 - Slave Arbitration lost. SAL is set when the slave loses the arbitration during the data phase. A collision occurs when 2 devices transmit simultaneously 2 opposite values on the serial data line. The device that is pulling up the line, identifies the collision reading a 0 value on the sda_in signal, stops the transmission, releases the bus and waits for the idle state (STOP condition received) on the bus line. The device which transmits the first unique zero wins the bus arbitration. This interrupt is cleared by setting the related bit of the ICR register.<ul><li>0: No slave arbitration lost.</li><li>1: Slave arbitration lost.</li></ul>"]
    #[inline]
    pub fn sal(&self) -> SALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SALR { bits }
    }
    #[doc = "Bit 24 - Master arbitration lost. MAL is set when the master loses the arbitration. The status code word in the SR contains a specific error tag (CAUSE field) for this error condition. A collision occurs when 2 stations transmit simultaneously 2 opposite values on the serial line. The station that is pulling up the line, identifies the collision reading a 0 value on the sda_in signal, stops the transmission, leaves the bus and waits for the idle state (STOP condition received) on the bus line before retrying the same transaction. The station which transmits the first unique zero wins the bus arbitration. This interrupt is cleared by setting the related bit of the ICR register.<ul><li>0: No master arbitration lost.</li><li>1: Master arbitration lost.</li></ul>"]
    #[inline]
    pub fn mal(&self) -> MALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MALR { bits }
    }
    #[doc = "Bit 25 - Bus Error. BERR is set when an unexpected Start/Stop condition occurs during a transaction. The related actions are different, depending on the type of operation in progress.The status code word in the SR contains a specific error tag (CAUSE field) for this error condition. This interrupt is cleared by setting the related bit of the ICR register.<ul><li>0: No bus error detection.</li><li>1: Bus error detection.</li></ul>"]
    #[inline]
    pub fn berr(&self) -> BERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BERRR { bits }
    }
    #[doc = "Bit 28 - Master transaction done without stop. MTDWS is set when a master operation (write or read) has been executed and a stop (MCR:P field) is not programmed. The application shall read the related transaction status (SR register), the pending data in the RX FIFO (only for a master read operation) and clear this interrupt (transaction acknowledgment). A subsequent master operation can be issued (by writing the MCR register) after clearing this interrupt. A subsequent slave operation will be notified (RISR:WTSR and RISR:RFSR interrupt bits assertion) after clearing this interrupt, meanwhile the I2C clock line will be stretched low. This interrupt is cleared by setting the related bit of the ICR register:<ul><li>0: Master transaction acknowledged.</li><li>1: Master transaction done (ready for acknowledgment) and stop is not applied into the I2C bus.</li></ul>"]
    #[inline]
    pub fn mtdws(&self) -> MTDWSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MTDWSR { bits }
    }
}
