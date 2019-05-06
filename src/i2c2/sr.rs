#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `OP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPR {
    #[doc = "Master write operation"]
    MW,
    #[doc = "Master read operation"]
    MR,
    #[doc = "Write to slave operation"]
    WTS,
    #[doc = "Read from slave operation"]
    RFS,
}
impl OPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPR::MW => 0,
            OPR::MR => 1,
            OPR::WTS => 2,
            OPR::RFS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPR {
        match value {
            0 => OPR::MW,
            1 => OPR::MR,
            2 => OPR::WTS,
            3 => OPR::RFS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MW`"]
    #[inline]
    pub fn is_mw(&self) -> bool {
        *self == OPR::MW
    }
    #[doc = "Checks if the value of the field is `MR`"]
    #[inline]
    pub fn is_mr(&self) -> bool {
        *self == OPR::MR
    }
    #[doc = "Checks if the value of the field is `WTS`"]
    #[inline]
    pub fn is_wts(&self) -> bool {
        *self == OPR::WTS
    }
    #[doc = "Checks if the value of the field is `RFS`"]
    #[inline]
    pub fn is_rfs(&self) -> bool {
        *self == OPR::RFS
    }
}
#[doc = "Possible values of the field `STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUSR {
    #[doc = "No operation is in progress"]
    NOP,
    #[doc = "An operation is ongoing"]
    ON_GOING,
    #[doc = "The operation (OP field) has been completed successfully"]
    OK,
    #[doc = "The operation (OP field) has been aborted due to the occurrence of the event descried in the CAUSE field"]
    ABORT,
}
impl STATUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATUSR::NOP => 0,
            STATUSR::ON_GOING => 1,
            STATUSR::OK => 2,
            STATUSR::ABORT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATUSR {
        match value {
            0 => STATUSR::NOP,
            1 => STATUSR::ON_GOING,
            2 => STATUSR::OK,
            3 => STATUSR::ABORT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline]
    pub fn is_nop(&self) -> bool {
        *self == STATUSR::NOP
    }
    #[doc = "Checks if the value of the field is `ON_GOING`"]
    #[inline]
    pub fn is_on_going(&self) -> bool {
        *self == STATUSR::ON_GOING
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline]
    pub fn is_ok(&self) -> bool {
        *self == STATUSR::OK
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline]
    pub fn is_abort(&self) -> bool {
        *self == STATUSR::ABORT
    }
}
#[doc = "Possible values of the field `CAUSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAUSER {
    #[doc = "The master receives a not-acknowledge after the transmission of the address"]
    NACK_ADDR,
    #[doc = "The master receives a not-acknowledge during the data phase of a MW operation"]
    NACK_DATA,
    #[doc = "The master loses the arbitration during a MW or MR operation"]
    ARB_LOST,
    #[doc = "Slave restarts, a START Condition occurs while the byte transfer is not terminated"]
    BERR_START,
    #[doc = "Slave reset, a STOP Condition while the byte transfer is not terminated"]
    BERR_STOP,
    #[doc = "The slave receives a frame related to the WTS operation longer than the maximum size = 2047 bytes"]
    OVFL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CAUSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAUSER::NACK_ADDR => 0,
            CAUSER::NACK_DATA => 1,
            CAUSER::ARB_LOST => 3,
            CAUSER::BERR_START => 4,
            CAUSER::BERR_STOP => 5,
            CAUSER::OVFL => 6,
            CAUSER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAUSER {
        match value {
            0 => CAUSER::NACK_ADDR,
            1 => CAUSER::NACK_DATA,
            3 => CAUSER::ARB_LOST,
            4 => CAUSER::BERR_START,
            5 => CAUSER::BERR_STOP,
            6 => CAUSER::OVFL,
            i => CAUSER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NACK_ADDR`"]
    #[inline]
    pub fn is_nack_addr(&self) -> bool {
        *self == CAUSER::NACK_ADDR
    }
    #[doc = "Checks if the value of the field is `NACK_DATA`"]
    #[inline]
    pub fn is_nack_data(&self) -> bool {
        *self == CAUSER::NACK_DATA
    }
    #[doc = "Checks if the value of the field is `ARB_LOST`"]
    #[inline]
    pub fn is_arb_lost(&self) -> bool {
        *self == CAUSER::ARB_LOST
    }
    #[doc = "Checks if the value of the field is `BERR_START`"]
    #[inline]
    pub fn is_berr_start(&self) -> bool {
        *self == CAUSER::BERR_START
    }
    #[doc = "Checks if the value of the field is `BERR_STOP`"]
    #[inline]
    pub fn is_berr_stop(&self) -> bool {
        *self == CAUSER::BERR_STOP
    }
    #[doc = "Checks if the value of the field is `OVFL`"]
    #[inline]
    pub fn is_ovfl(&self) -> bool {
        *self == CAUSER::OVFL
    }
}
#[doc = "Possible values of the field `TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPER {
    #[doc = "The slave has received a normal frame"]
    FRAME,
    #[doc = "The slave has received a general call"]
    GCALL,
    #[doc = "The slave has received a hardware general call"]
    HW_GCALL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TYPER::FRAME => 0,
            TYPER::GCALL => 1,
            TYPER::HW_GCALL => 2,
            TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TYPER {
        match value {
            0 => TYPER::FRAME,
            1 => TYPER::GCALL,
            2 => TYPER::HW_GCALL,
            i => TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRAME`"]
    #[inline]
    pub fn is_frame(&self) -> bool {
        *self == TYPER::FRAME
    }
    #[doc = "Checks if the value of the field is `GCALL`"]
    #[inline]
    pub fn is_gcall(&self) -> bool {
        *self == TYPER::GCALL
    }
    #[doc = "Checks if the value of the field is `HW_GCALL`"]
    #[inline]
    pub fn is_hw_gcall(&self) -> bool {
        *self == TYPER::HW_GCALL
    }
}
#[doc = r" Value of the field"]
pub struct LENGTHR {
    bits: u16,
}
impl LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DUALF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUALFR {
    #[doc = "Received address matched with slave address (SA7)"]
    DUAL_SLAVE_ADDR_OFF,
    #[doc = "Received address matched with dual slave address (DSA7)"]
    DUAL_SLAVE_ADDR_ON,
}
impl DUALFR {
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
            DUALFR::DUAL_SLAVE_ADDR_OFF => false,
            DUALFR::DUAL_SLAVE_ADDR_ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DUALFR {
        match value {
            false => DUALFR::DUAL_SLAVE_ADDR_OFF,
            true => DUALFR::DUAL_SLAVE_ADDR_ON,
        }
    }
    #[doc = "Checks if the value of the field is `DUAL_SLAVE_ADDR_OFF`"]
    #[inline]
    pub fn is_dual_slave_addr_off(&self) -> bool {
        *self == DUALFR::DUAL_SLAVE_ADDR_OFF
    }
    #[doc = "Checks if the value of the field is `DUAL_SLAVE_ADDR_ON`"]
    #[inline]
    pub fn is_dual_slave_addr_on(&self) -> bool {
        *self == DUALFR::DUAL_SLAVE_ADDR_ON
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Operation:<ul><li>00b: MW: master write operation.</li><li>01b: MR: master read operation.</li><li>10b: WTS: write-to-slave operation.</li><li>11b: RFS: read-from-slave operation.</li></ul>"]
    #[inline]
    pub fn op(&self) -> OPR {
        OPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Controller status. Valid for the operations MW, MR, WTS RFS:<ul><li>00b: NOP: No operation is in progress.</li><li>01b: ON_GOING: An operation is ongoing.</li><li>10b: OK: The operation (OP field) has been completed successfully.</li><li>11b: ABORT: The operation (OP field) has been aborted due to the occurrence of the event described in the CAUSE field.</li></ul>"]
    #[inline]
    pub fn status(&self) -> STATUSR {
        STATUSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Abort cause. This field is valid only when the STATUS field contains the ABORT tag. Others: RESERVED. <ul><li>000b: NACK_ADDR: The master receives a not-acknowledge after the transmission of the address. Valid for the operation MW, MR.</li><li>001b: NACK_DATA: The master receives a not-acknowledge during the data phase of a MW operation. Valid for the operation MW.</li><li>011b: ARB_LOST: The master loses the arbitration during a MW or MR operation. Valid for the operation MW, MR.</li><li>100b: BERR_START: Slave restarts, a START Condition occurs while the byte transfer is not terminated.</li><li>101b: BERR_STOP: Slave reset, a STOP Condition while the byte transfer is not terminated.</li><li>110b: OVFL: The slave receives a frame related to the WTS operation longer than the maximum size = 2047 bytes. In this case the slave device returns a NACK to complete the data transfer. Valid for WTS operation</li></ul>"]
    #[inline]
    pub fn cause(&self) -> CAUSER {
        CAUSER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:8 - Receive type. Valid only for the operation WTS:<ul><li>00b: FRAME: The slave has received a normal frame.</li><li>01b: GCALL: The slave has received a general call. If the it I2C_CR:SGCM is set to 1, the general call is directly executed without software intervention and only the control code word is reported in FIFO (LENGTH =0).</li><li>10b: HW_GCALL: The slave has received a hardware general call.</li></ul>"]
    #[inline]
    pub fn type_(&self) -> TYPER {
        TYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:18 - Transfer length. For an MR, WTS operation the LENGTH field defines the actual size of the subsequent payload, in terms of number of bytes. For an MW, RFS operation the LENGTH field defines the actual number of bytes transferred by the master/slave device. For a WTS operation if the transfer length exceeds 2047 bytes, the operation is stopped by the slave returning a NACK handshake and the flag OVFL is set. For an RFS operation if the transfer length exceeds 2047 bytes, the operation continues normally but the LENGTH field is reset to 0."]
    #[inline]
    pub fn length(&self) -> LENGTHR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LENGTHR { bits }
    }
    #[doc = "Bit 29 - Dual flag (slave mode):<ul><li>0: Received address matched with slave address (SA7).</li><li>1: Received address matched with dual slave address (DSA7).</li></ul>Cleared by hardware after a Stop condition or repeated Start condition, bus error or when PE=0."]
    #[inline]
    pub fn dualf(&self) -> DUALFR {
        DUALFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
