#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFO_RX_WATERMARK` reader - FIFO reached Receive watermark level, not qualified with data transfer."]
pub struct FIFO_RX_WATERMARK_R(crate::FieldReader<bool, bool>);
impl FIFO_RX_WATERMARK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_RX_WATERMARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_RX_WATERMARK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_TX_WATERMARK` reader - FIFO reached Transmit watermark level, not qualified with data transfer."]
pub struct FIFO_TX_WATERMARK_R(crate::FieldReader<bool, bool>);
impl FIFO_TX_WATERMARK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_TX_WATERMARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_TX_WATERMARK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_EMPTY` reader - FIFO is empty status."]
pub struct FIFO_EMPTY_R(crate::FieldReader<bool, bool>);
impl FIFO_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_FULL` reader - FIFO is full status."]
pub struct FIFO_FULL_R(crate::FieldReader<bool, bool>);
impl FIFO_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND_FSM_STATES` reader - Command FSM states. 0: Idle; 1: Send init sequence; 2: Send cmd start bit; 3: Send cmd tx bit; 4: Send cmd index + arg; 5: Send cmd crc7; 6: Send cmd end bit; 7: Receive resp start bit; 8: Receive resp IRQ response; 9: Receive resp tx bit; 10: Receive resp cmd idx; 11: Receive resp data; 12: Receive resp crc7; 13: Receive resp end bit; 14: Cmd path wait NCC; 15: Wait, cmd-to-response turnaround."]
pub struct COMMAND_FSM_STATES_R(crate::FieldReader<u8, u8>);
impl COMMAND_FSM_STATES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COMMAND_FSM_STATES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND_FSM_STATES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_3_STATUS` reader - Raw selected sdhost_card_data\\[3\\], checks whether card is present. 0: card not present; 1: card present."]
pub struct DATA_3_STATUS_R(crate::FieldReader<bool, bool>);
impl DATA_3_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA_3_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_3_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_BUSY` reader - Inverted version of raw selected sdhost_card_data\\[0\\]. 0: Card data not busy; 1: Card data busy."]
pub struct DATA_BUSY_R(crate::FieldReader<bool, bool>);
impl DATA_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_STATE_MC_BUSY` reader - Data transmit or receive state-machine is busy."]
pub struct DATA_STATE_MC_BUSY_R(crate::FieldReader<bool, bool>);
impl DATA_STATE_MC_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA_STATE_MC_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_STATE_MC_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESPONSE_INDEX` reader - Index of previous response, including any auto-stop sent by core."]
pub struct RESPONSE_INDEX_R(crate::FieldReader<u8, u8>);
impl RESPONSE_INDEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RESPONSE_INDEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESPONSE_INDEX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_COUNT` reader - FIFO count, number of filled locations in FIFO."]
pub struct FIFO_COUNT_R(crate::FieldReader<u16, u16>);
impl FIFO_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FIFO_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - FIFO reached Receive watermark level, not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_rx_watermark(&self) -> FIFO_RX_WATERMARK_R {
        FIFO_RX_WATERMARK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level, not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_tx_watermark(&self) -> FIFO_TX_WATERMARK_R {
        FIFO_TX_WATERMARK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO is empty status."]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO is full status."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Command FSM states. 0: Idle; 1: Send init sequence; 2: Send cmd start bit; 3: Send cmd tx bit; 4: Send cmd index + arg; 5: Send cmd crc7; 6: Send cmd end bit; 7: Receive resp start bit; 8: Receive resp IRQ response; 9: Receive resp tx bit; 10: Receive resp cmd idx; 11: Receive resp data; 12: Receive resp crc7; 13: Receive resp end bit; 14: Cmd path wait NCC; 15: Wait, cmd-to-response turnaround."]
    #[inline(always)]
    pub fn command_fsm_states(&self) -> COMMAND_FSM_STATES_R {
        COMMAND_FSM_STATES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Raw selected sdhost_card_data\\[3\\], checks whether card is present. 0: card not present; 1: card present."]
    #[inline(always)]
    pub fn data_3_status(&self) -> DATA_3_STATUS_R {
        DATA_3_STATUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Inverted version of raw selected sdhost_card_data\\[0\\]. 0: Card data not busy; 1: Card data busy."]
    #[inline(always)]
    pub fn data_busy(&self) -> DATA_BUSY_R {
        DATA_BUSY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data transmit or receive state-machine is busy."]
    #[inline(always)]
    pub fn data_state_mc_busy(&self) -> DATA_STATE_MC_BUSY_R {
        DATA_STATE_MC_BUSY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:16 - Index of previous response, including any auto-stop sent by core."]
    #[inline(always)]
    pub fn response_index(&self) -> RESPONSE_INDEX_R {
        RESPONSE_INDEX_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:29 - FIFO count, number of filled locations in FIFO."]
    #[inline(always)]
    pub fn fifo_count(&self) -> FIFO_COUNT_R {
        FIFO_COUNT_R::new(((self.bits >> 17) & 0x1fff) as u16)
    }
}
#[doc = "SD/MMC status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x0716"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0716
    }
}
