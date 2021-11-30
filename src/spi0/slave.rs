#[doc = "Register `SLAVE` reader"]
pub struct R(crate::R<SLAVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE` writer"]
pub struct W(crate::W<SLAVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE_SPEC>;
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
impl From<crate::W<SLAVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_RD_BUF_DONE` reader - The interrupt raw bit for the completion of read-buffer operation in the slave mode."]
pub struct SLV_RD_BUF_DONE_R(crate::FieldReader<bool, bool>);
impl SLV_RD_BUF_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RD_BUF_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RD_BUF_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RD_BUF_DONE` writer - The interrupt raw bit for the completion of read-buffer operation in the slave mode."]
pub struct SLV_RD_BUF_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RD_BUF_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SLV_WR_BUF_DONE` reader - The interrupt raw bit for the completion of write-buffer operation in the slave mode."]
pub struct SLV_WR_BUF_DONE_R(crate::FieldReader<bool, bool>);
impl SLV_WR_BUF_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WR_BUF_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WR_BUF_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WR_BUF_DONE` writer - The interrupt raw bit for the completion of write-buffer operation in the slave mode."]
pub struct SLV_WR_BUF_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_BUF_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SLV_RD_STA_DONE` reader - The interrupt raw bit for the completion of read-status operation in the slave mode."]
pub struct SLV_RD_STA_DONE_R(crate::FieldReader<bool, bool>);
impl SLV_RD_STA_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RD_STA_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RD_STA_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RD_STA_DONE` writer - The interrupt raw bit for the completion of read-status operation in the slave mode."]
pub struct SLV_RD_STA_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RD_STA_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SLV_WR_STA_DONE` reader - The interrupt raw bit for the completion of write-status operation in the slave mode."]
pub struct SLV_WR_STA_DONE_R(crate::FieldReader<bool, bool>);
impl SLV_WR_STA_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WR_STA_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WR_STA_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WR_STA_DONE` writer - The interrupt raw bit for the completion of write-status operation in the slave mode."]
pub struct SLV_WR_STA_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_STA_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TRANS_DONE` reader - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode."]
pub struct TRANS_DONE_R(crate::FieldReader<bool, bool>);
impl TRANS_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_DONE` writer - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode."]
pub struct TRANS_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `INT_EN` reader - Interrupt enable bits for the below 5 sources"]
pub struct INT_EN_R(crate::FieldReader<u8, u8>);
impl INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_EN` writer - Interrupt enable bits for the below 5 sources"]
pub struct INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `CS_I_MODE` reader - In the slave mode this bits used to synchronize the input spi cs signal and eliminate spi cs jitter."]
pub struct CS_I_MODE_R(crate::FieldReader<u8, u8>);
impl CS_I_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CS_I_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_I_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_I_MODE` writer - In the slave mode this bits used to synchronize the input spi cs signal and eliminate spi cs jitter."]
pub struct CS_I_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_I_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `SLV_LAST_COMMAND` reader - In the slave mode it is the value of command."]
pub struct SLV_LAST_COMMAND_R(crate::FieldReader<u8, u8>);
impl SLV_LAST_COMMAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_LAST_COMMAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_LAST_COMMAND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_LAST_STATE` reader - In the slave mode it is the state of spi state machine."]
pub struct SLV_LAST_STATE_R(crate::FieldReader<u8, u8>);
impl SLV_LAST_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_LAST_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_LAST_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_CNT` reader - The operations counter in both the master mode and the slave mode. 4: read-status"]
pub struct TRANS_CNT_R(crate::FieldReader<u8, u8>);
impl TRANS_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRANS_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD_DEFINE` reader - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as: 1: write-status 2: write-buffer and 3: read-buffer."]
pub struct SLV_CMD_DEFINE_R(crate::FieldReader<bool, bool>);
impl SLV_CMD_DEFINE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD_DEFINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD_DEFINE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD_DEFINE` writer - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as: 1: write-status 2: write-buffer and 3: read-buffer."]
pub struct SLV_CMD_DEFINE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD_DEFINE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `SLV_WR_RD_STA_EN` reader - write and read status enable in the slave mode"]
pub struct SLV_WR_RD_STA_EN_R(crate::FieldReader<bool, bool>);
impl SLV_WR_RD_STA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WR_RD_STA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WR_RD_STA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WR_RD_STA_EN` writer - write and read status enable in the slave mode"]
pub struct SLV_WR_RD_STA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_RD_STA_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `SLV_WR_RD_BUF_EN` reader - write and read buffer enable in the slave mode"]
pub struct SLV_WR_RD_BUF_EN_R(crate::FieldReader<bool, bool>);
impl SLV_WR_RD_BUF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WR_RD_BUF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WR_RD_BUF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WR_RD_BUF_EN` writer - write and read buffer enable in the slave mode"]
pub struct SLV_WR_RD_BUF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_RD_BUF_EN_W<'a> {
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
#[doc = "Field `MODE` reader - 1: slave mode 0: master mode."]
pub struct MODE_R(crate::FieldReader<bool, bool>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - 1: slave mode 0: master mode."]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `SYNC_RESET` reader - Software reset enable, reset the spi clock line cs line and data lines."]
pub struct SYNC_RESET_R(crate::FieldReader<bool, bool>);
impl SYNC_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNC_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNC_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC_RESET` writer - Software reset enable, reset the spi clock line cs line and data lines."]
pub struct SYNC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_RESET_W<'a> {
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
    #[doc = "Bit 0 - The interrupt raw bit for the completion of read-buffer operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&self) -> SLV_RD_BUF_DONE_R {
        SLV_RD_BUF_DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for the completion of write-buffer operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&self) -> SLV_WR_BUF_DONE_R {
        SLV_WR_BUF_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for the completion of read-status operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_sta_done(&self) -> SLV_RD_STA_DONE_R {
        SLV_RD_STA_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for the completion of write-status operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_sta_done(&self) -> SLV_WR_STA_DONE_R {
        SLV_WR_STA_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode."]
    #[inline(always)]
    pub fn trans_done(&self) -> TRANS_DONE_R {
        TRANS_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:9 - Interrupt enable bits for the below 5 sources"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - In the slave mode this bits used to synchronize the input spi cs signal and eliminate spi cs jitter."]
    #[inline(always)]
    pub fn cs_i_mode(&self) -> CS_I_MODE_R {
        CS_I_MODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 17:19 - In the slave mode it is the value of command."]
    #[inline(always)]
    pub fn slv_last_command(&self) -> SLV_LAST_COMMAND_R {
        SLV_LAST_COMMAND_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - In the slave mode it is the state of spi state machine."]
    #[inline(always)]
    pub fn slv_last_state(&self) -> SLV_LAST_STATE_R {
        SLV_LAST_STATE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:26 - The operations counter in both the master mode and the slave mode. 4: read-status"]
    #[inline(always)]
    pub fn trans_cnt(&self) -> TRANS_CNT_R {
        TRANS_CNT_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as: 1: write-status 2: write-buffer and 3: read-buffer."]
    #[inline(always)]
    pub fn slv_cmd_define(&self) -> SLV_CMD_DEFINE_R {
        SLV_CMD_DEFINE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - write and read status enable in the slave mode"]
    #[inline(always)]
    pub fn slv_wr_rd_sta_en(&self) -> SLV_WR_RD_STA_EN_R {
        SLV_WR_RD_STA_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - write and read buffer enable in the slave mode"]
    #[inline(always)]
    pub fn slv_wr_rd_buf_en(&self) -> SLV_WR_RD_BUF_EN_R {
        SLV_WR_RD_BUF_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Software reset enable, reset the spi clock line cs line and data lines."]
    #[inline(always)]
    pub fn sync_reset(&self) -> SYNC_RESET_R {
        SYNC_RESET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt raw bit for the completion of read-buffer operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&mut self) -> SLV_RD_BUF_DONE_W {
        SLV_RD_BUF_DONE_W { w: self }
    }
    #[doc = "Bit 1 - The interrupt raw bit for the completion of write-buffer operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&mut self) -> SLV_WR_BUF_DONE_W {
        SLV_WR_BUF_DONE_W { w: self }
    }
    #[doc = "Bit 2 - The interrupt raw bit for the completion of read-status operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_sta_done(&mut self) -> SLV_RD_STA_DONE_W {
        SLV_RD_STA_DONE_W { w: self }
    }
    #[doc = "Bit 3 - The interrupt raw bit for the completion of write-status operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_sta_done(&mut self) -> SLV_WR_STA_DONE_W {
        SLV_WR_STA_DONE_W { w: self }
    }
    #[doc = "Bit 4 - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode."]
    #[inline(always)]
    pub fn trans_done(&mut self) -> TRANS_DONE_W {
        TRANS_DONE_W { w: self }
    }
    #[doc = "Bits 5:9 - Interrupt enable bits for the below 5 sources"]
    #[inline(always)]
    pub fn int_en(&mut self) -> INT_EN_W {
        INT_EN_W { w: self }
    }
    #[doc = "Bits 10:11 - In the slave mode this bits used to synchronize the input spi cs signal and eliminate spi cs jitter."]
    #[inline(always)]
    pub fn cs_i_mode(&mut self) -> CS_I_MODE_W {
        CS_I_MODE_W { w: self }
    }
    #[doc = "Bit 27 - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as: 1: write-status 2: write-buffer and 3: read-buffer."]
    #[inline(always)]
    pub fn slv_cmd_define(&mut self) -> SLV_CMD_DEFINE_W {
        SLV_CMD_DEFINE_W { w: self }
    }
    #[doc = "Bit 28 - write and read status enable in the slave mode"]
    #[inline(always)]
    pub fn slv_wr_rd_sta_en(&mut self) -> SLV_WR_RD_STA_EN_W {
        SLV_WR_RD_STA_EN_W { w: self }
    }
    #[doc = "Bit 29 - write and read buffer enable in the slave mode"]
    #[inline(always)]
    pub fn slv_wr_rd_buf_en(&mut self) -> SLV_WR_RD_BUF_EN_W {
        SLV_WR_RD_BUF_EN_W { w: self }
    }
    #[doc = "Bit 30 - 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 31 - Software reset enable, reset the spi clock line cs line and data lines."]
    #[inline(always)]
    pub fn sync_reset(&mut self) -> SYNC_RESET_W {
        SYNC_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave](index.html) module"]
pub struct SLAVE_SPEC;
impl crate::RegisterSpec for SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave::R](R) reader structure"]
impl crate::Readable for SLAVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave::W](W) writer structure"]
impl crate::Writable for SLAVE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLAVE to value 0x20"]
impl crate::Resettable for SLAVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
