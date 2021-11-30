#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INDEX` reader - Command index."]
pub struct INDEX_R(crate::FieldReader<u8, u8>);
impl INDEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INDEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INDEX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INDEX` writer - Command index."]
pub struct INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `RESPONSE_EXPECT` reader - 0: No response expected from card; 1: Response expected from card."]
pub struct RESPONSE_EXPECT_R(crate::FieldReader<bool, bool>);
impl RESPONSE_EXPECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESPONSE_EXPECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESPONSE_EXPECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESPONSE_EXPECT` writer - 0: No response expected from card; 1: Response expected from card."]
pub struct RESPONSE_EXPECT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPONSE_EXPECT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RESPONSE_LENGTH` reader - 0: Short response expected from card; 1: Long response expected from card."]
pub struct RESPONSE_LENGTH_R(crate::FieldReader<bool, bool>);
impl RESPONSE_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESPONSE_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESPONSE_LENGTH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESPONSE_LENGTH` writer - 0: Short response expected from card; 1: Long response expected from card."]
pub struct RESPONSE_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPONSE_LENGTH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CHECK_RESPONSE_CRC` reader - 0: Do not check; 1: Check response CRC. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
pub struct CHECK_RESPONSE_CRC_R(crate::FieldReader<bool, bool>);
impl CHECK_RESPONSE_CRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHECK_RESPONSE_CRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHECK_RESPONSE_CRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHECK_RESPONSE_CRC` writer - 0: Do not check; 1: Check response CRC. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
pub struct CHECK_RESPONSE_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CHECK_RESPONSE_CRC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DATA_EXPECTED` reader - 0: No data transfer expected; 1: Data transfer expected."]
pub struct DATA_EXPECTED_R(crate::FieldReader<bool, bool>);
impl DATA_EXPECTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA_EXPECTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_EXPECTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_EXPECTED` writer - 0: No data transfer expected; 1: Data transfer expected."]
pub struct DATA_EXPECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_EXPECTED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `READ_WRITE` reader - 0: Read from card; 1: Write to card. Don't care if no data is expected from card."]
pub struct READ_WRITE_R(crate::FieldReader<bool, bool>);
impl READ_WRITE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READ_WRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_WRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_WRITE` writer - 0: Read from card; 1: Write to card. Don't care if no data is expected from card."]
pub struct READ_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_WRITE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `TRANSFER_MODE` reader - Block data transfer command; 1: Stream data transfer command. Don't care if no data expected."]
pub struct TRANSFER_MODE_R(crate::FieldReader<bool, bool>);
impl TRANSFER_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANSFER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANSFER_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANSFER_MODE` writer - Block data transfer command; 1: Stream data transfer command. Don't care if no data expected."]
pub struct TRANSFER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSFER_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SEND_AUTO_STOP` reader - 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer."]
pub struct SEND_AUTO_STOP_R(crate::FieldReader<bool, bool>);
impl SEND_AUTO_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_AUTO_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_AUTO_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_AUTO_STOP` writer - 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer."]
pub struct SEND_AUTO_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_AUTO_STOP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `WAIT_PRVDATA_COMPLETE` reader - 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command. The SDHOST_WAIT_PRVDATA_COMPLETE\\]
= 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command."]
pub struct WAIT_PRVDATA_COMPLETE_R(crate::FieldReader<bool, bool>);
impl WAIT_PRVDATA_COMPLETE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAIT_PRVDATA_COMPLETE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAIT_PRVDATA_COMPLETE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT_PRVDATA_COMPLETE` writer - 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command. The SDHOST_WAIT_PRVDATA_COMPLETE\\]
= 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command."]
pub struct WAIT_PRVDATA_COMPLETE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_PRVDATA_COMPLETE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `STOP_ABORT_CMD` reader - 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state."]
pub struct STOP_ABORT_CMD_R(crate::FieldReader<bool, bool>);
impl STOP_ABORT_CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP_ABORT_CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_ABORT_CMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP_ABORT_CMD` writer - 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state."]
pub struct STOP_ABORT_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_ABORT_CMD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SEND_INITIALIZATION` reader - 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command. After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card."]
pub struct SEND_INITIALIZATION_R(crate::FieldReader<bool, bool>);
impl SEND_INITIALIZATION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_INITIALIZATION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_INITIALIZATION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_INITIALIZATION` writer - 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command. After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card."]
pub struct SEND_INITIALIZATION_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_INITIALIZATION_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CARD_NUMBER` reader - Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported."]
pub struct CARD_NUMBER_R(crate::FieldReader<u8, u8>);
impl CARD_NUMBER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CARD_NUMBER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_NUMBER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_NUMBER` writer - Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported."]
pub struct CARD_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` reader - 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain. Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
pub struct UPDATE_CLOCK_REGISTERS_ONLY_R(crate::FieldReader<bool, bool>);
impl UPDATE_CLOCK_REGISTERS_ONLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPDATE_CLOCK_REGISTERS_ONLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPDATE_CLOCK_REGISTERS_ONLY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` writer - 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain. Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
pub struct UPDATE_CLOCK_REGISTERS_ONLY_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATE_CLOCK_REGISTERS_ONLY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `READ_CEATA_DEVICE` reader - Read access flag. 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device; 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device."]
pub struct READ_CEATA_DEVICE_R(crate::FieldReader<bool, bool>);
impl READ_CEATA_DEVICE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READ_CEATA_DEVICE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_CEATA_DEVICE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_CEATA_DEVICE` writer - Read access flag. 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device; 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device."]
pub struct READ_CEATA_DEVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_CEATA_DEVICE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CCS_EXPECTED` reader - Expected Command Completion Signal (CCS) configuration. 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device; 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
pub struct CCS_EXPECTED_R(crate::FieldReader<bool, bool>);
impl CCS_EXPECTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCS_EXPECTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCS_EXPECTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCS_EXPECTED` writer - Expected Command Completion Signal (CCS) configuration. 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device; 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
pub struct CCS_EXPECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> CCS_EXPECTED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `USE_HOLE` reader - Use Hold Register. 0: CMD and DATA sent to card bypassing HOLD Register; 1: CMD and DATA sent to card through the HOLD Register."]
pub struct USE_HOLE_R(crate::FieldReader<bool, bool>);
impl USE_HOLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USE_HOLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USE_HOLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USE_HOLE` writer - Use Hold Register. 0: CMD and DATA sent to card bypassing HOLD Register; 1: CMD and DATA sent to card through the HOLD Register."]
pub struct USE_HOLE_W<'a> {
    w: &'a mut W,
}
impl<'a> USE_HOLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `START_CMD` reader - Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register."]
pub struct START_CMD_R(crate::FieldReader<bool, bool>);
impl START_CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_CMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_CMD` writer - Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register."]
pub struct START_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> START_CMD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 0: No response expected from card; 1: Response expected from card."]
    #[inline(always)]
    pub fn response_expect(&self) -> RESPONSE_EXPECT_R {
        RESPONSE_EXPECT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 0: Short response expected from card; 1: Long response expected from card."]
    #[inline(always)]
    pub fn response_length(&self) -> RESPONSE_LENGTH_R {
        RESPONSE_LENGTH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 0: Do not check; 1: Check response CRC. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
    #[inline(always)]
    pub fn check_response_crc(&self) -> CHECK_RESPONSE_CRC_R {
        CHECK_RESPONSE_CRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 0: No data transfer expected; 1: Data transfer expected."]
    #[inline(always)]
    pub fn data_expected(&self) -> DATA_EXPECTED_R {
        DATA_EXPECTED_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 0: Read from card; 1: Write to card. Don't care if no data is expected from card."]
    #[inline(always)]
    pub fn read_write(&self) -> READ_WRITE_R {
        READ_WRITE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Block data transfer command; 1: Stream data transfer command. Don't care if no data expected."]
    #[inline(always)]
    pub fn transfer_mode(&self) -> TRANSFER_MODE_R {
        TRANSFER_MODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer."]
    #[inline(always)]
    pub fn send_auto_stop(&self) -> SEND_AUTO_STOP_R {
        SEND_AUTO_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command. The SDHOST_WAIT_PRVDATA_COMPLETE\\]
= 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command."]
    #[inline(always)]
    pub fn wait_prvdata_complete(&self) -> WAIT_PRVDATA_COMPLETE_R {
        WAIT_PRVDATA_COMPLETE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state."]
    #[inline(always)]
    pub fn stop_abort_cmd(&self) -> STOP_ABORT_CMD_R {
        STOP_ABORT_CMD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command. After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card."]
    #[inline(always)]
    pub fn send_initialization(&self) -> SEND_INITIALIZATION_R {
        SEND_INITIALIZATION_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported."]
    #[inline(always)]
    pub fn card_number(&self) -> CARD_NUMBER_R {
        CARD_NUMBER_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain. Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
    #[inline(always)]
    pub fn update_clock_registers_only(&self) -> UPDATE_CLOCK_REGISTERS_ONLY_R {
        UPDATE_CLOCK_REGISTERS_ONLY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Read access flag. 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device; 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device."]
    #[inline(always)]
    pub fn read_ceata_device(&self) -> READ_CEATA_DEVICE_R {
        READ_CEATA_DEVICE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Expected Command Completion Signal (CCS) configuration. 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device; 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
    #[inline(always)]
    pub fn ccs_expected(&self) -> CCS_EXPECTED_R {
        CCS_EXPECTED_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Use Hold Register. 0: CMD and DATA sent to card bypassing HOLD Register; 1: CMD and DATA sent to card through the HOLD Register."]
    #[inline(always)]
    pub fn use_hole(&self) -> USE_HOLE_R {
        USE_HOLE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register."]
    #[inline(always)]
    pub fn start_cmd(&self) -> START_CMD_R {
        START_CMD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn index(&mut self) -> INDEX_W {
        INDEX_W { w: self }
    }
    #[doc = "Bit 6 - 0: No response expected from card; 1: Response expected from card."]
    #[inline(always)]
    pub fn response_expect(&mut self) -> RESPONSE_EXPECT_W {
        RESPONSE_EXPECT_W { w: self }
    }
    #[doc = "Bit 7 - 0: Short response expected from card; 1: Long response expected from card."]
    #[inline(always)]
    pub fn response_length(&mut self) -> RESPONSE_LENGTH_W {
        RESPONSE_LENGTH_W { w: self }
    }
    #[doc = "Bit 8 - 0: Do not check; 1: Check response CRC. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
    #[inline(always)]
    pub fn check_response_crc(&mut self) -> CHECK_RESPONSE_CRC_W {
        CHECK_RESPONSE_CRC_W { w: self }
    }
    #[doc = "Bit 9 - 0: No data transfer expected; 1: Data transfer expected."]
    #[inline(always)]
    pub fn data_expected(&mut self) -> DATA_EXPECTED_W {
        DATA_EXPECTED_W { w: self }
    }
    #[doc = "Bit 10 - 0: Read from card; 1: Write to card. Don't care if no data is expected from card."]
    #[inline(always)]
    pub fn read_write(&mut self) -> READ_WRITE_W {
        READ_WRITE_W { w: self }
    }
    #[doc = "Bit 11 - Block data transfer command; 1: Stream data transfer command. Don't care if no data expected."]
    #[inline(always)]
    pub fn transfer_mode(&mut self) -> TRANSFER_MODE_W {
        TRANSFER_MODE_W { w: self }
    }
    #[doc = "Bit 12 - 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer."]
    #[inline(always)]
    pub fn send_auto_stop(&mut self) -> SEND_AUTO_STOP_W {
        SEND_AUTO_STOP_W { w: self }
    }
    #[doc = "Bit 13 - 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command. The SDHOST_WAIT_PRVDATA_COMPLETE\\]
= 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command."]
    #[inline(always)]
    pub fn wait_prvdata_complete(&mut self) -> WAIT_PRVDATA_COMPLETE_W {
        WAIT_PRVDATA_COMPLETE_W { w: self }
    }
    #[doc = "Bit 14 - 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state."]
    #[inline(always)]
    pub fn stop_abort_cmd(&mut self) -> STOP_ABORT_CMD_W {
        STOP_ABORT_CMD_W { w: self }
    }
    #[doc = "Bit 15 - 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command. After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card."]
    #[inline(always)]
    pub fn send_initialization(&mut self) -> SEND_INITIALIZATION_W {
        SEND_INITIALIZATION_W { w: self }
    }
    #[doc = "Bits 16:20 - Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported."]
    #[inline(always)]
    pub fn card_number(&mut self) -> CARD_NUMBER_W {
        CARD_NUMBER_W { w: self }
    }
    #[doc = "Bit 21 - 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain. Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
    #[inline(always)]
    pub fn update_clock_registers_only(&mut self) -> UPDATE_CLOCK_REGISTERS_ONLY_W {
        UPDATE_CLOCK_REGISTERS_ONLY_W { w: self }
    }
    #[doc = "Bit 22 - Read access flag. 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device; 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device."]
    #[inline(always)]
    pub fn read_ceata_device(&mut self) -> READ_CEATA_DEVICE_W {
        READ_CEATA_DEVICE_W { w: self }
    }
    #[doc = "Bit 23 - Expected Command Completion Signal (CCS) configuration. 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device; 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
    #[inline(always)]
    pub fn ccs_expected(&mut self) -> CCS_EXPECTED_W {
        CCS_EXPECTED_W { w: self }
    }
    #[doc = "Bit 29 - Use Hold Register. 0: CMD and DATA sent to card bypassing HOLD Register; 1: CMD and DATA sent to card through the HOLD Register."]
    #[inline(always)]
    pub fn use_hole(&mut self) -> USE_HOLE_W {
        USE_HOLE_W { w: self }
    }
    #[doc = "Bit 31 - Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register."]
    #[inline(always)]
    pub fn start_cmd(&mut self) -> START_CMD_W {
        START_CMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command and boot configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0x2000_0000"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_0000
    }
}
