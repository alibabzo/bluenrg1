#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::RFR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RDATAR {
    bits: u8,
}
impl RDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:7 - Receive data. RDATA contains the received payload, related to a master read or write-to-slave operation, to be read from the Rx FIFO. The RDATA(0) is the first LSB bit received over the I2C line. In case the FIFO is full, the I2C controller stretches automatically the I2C clock line until a new entry is available.<p>For a write-to-slave operation, when the slave is addressed, the interrupt I2C_RISR:WTSR bit is asserted for notification to the CPU. In CPU mode the FIFO management shall be based on the assertion of the interrupt bit I2C_RISR:RXFNF, related to the nearly-full threshold.</p><p>In DMA mode, the single requests are automatically executed based on the number of entries contained in the Rx FIFO.</p>"]
    #[inline]
    pub fn rdata(&self) -> RDATAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        RDATAR { bits }
    }
}
