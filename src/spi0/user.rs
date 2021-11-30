#[doc = "Register `USER` reader"]
pub struct R(crate::R<USER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER` writer"]
pub struct W(crate::W<USER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER_SPEC>;
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
impl From<crate::W<USER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUTDIN` reader - Set the bit to enable full duplex communication. 1: enable 0: disable."]
pub struct DOUTDIN_R(crate::FieldReader<bool, bool>);
impl DOUTDIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOUTDIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUTDIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUTDIN` writer - Set the bit to enable full duplex communication. 1: enable 0: disable."]
pub struct DOUTDIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUTDIN_W<'a> {
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
#[doc = "Field `CS_HOLD` reader - spi cs keep low when spi is in ¡°done¡± phase. 1: enable 0: disable."]
pub struct CS_HOLD_R(crate::FieldReader<bool, bool>);
impl CS_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_HOLD` writer - spi cs keep low when spi is in ¡°done¡± phase. 1: enable 0: disable."]
pub struct CS_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_HOLD_W<'a> {
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
#[doc = "Field `CS_SETUP` reader - spi cs is enable when spi is in ¡°prepare¡± phase. 1: enable 0: disable."]
pub struct CS_SETUP_R(crate::FieldReader<bool, bool>);
impl CS_SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_SETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_SETUP` writer - spi cs is enable when spi is in ¡°prepare¡± phase. 1: enable 0: disable."]
pub struct CS_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_SETUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CK_I_EDGE` reader - In the slave mode the bit is same as spi_ck_out_edge in master mode. It is combined with spi_miso_delay_mode bits."]
pub struct CK_I_EDGE_R(crate::FieldReader<bool, bool>);
impl CK_I_EDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK_I_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK_I_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK_I_EDGE` writer - In the slave mode the bit is same as spi_ck_out_edge in master mode. It is combined with spi_miso_delay_mode bits."]
pub struct CK_I_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CK_I_EDGE_W<'a> {
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
#[doc = "Field `CK_OUT_EDGE` reader - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode."]
pub struct CK_OUT_EDGE_R(crate::FieldReader<bool, bool>);
impl CK_OUT_EDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK_OUT_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK_OUT_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK_OUT_EDGE` writer - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode."]
pub struct CK_OUT_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CK_OUT_EDGE_W<'a> {
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
#[doc = "Field `RD_BYTE_ORDER` reader - In read-data (MISO) phase 1: big-endian 0: little_endian"]
pub struct RD_BYTE_ORDER_R(crate::FieldReader<bool, bool>);
impl RD_BYTE_ORDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_BYTE_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_BYTE_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_BYTE_ORDER` writer - In read-data (MISO) phase 1: big-endian 0: little_endian"]
pub struct RD_BYTE_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_BYTE_ORDER_W<'a> {
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
#[doc = "Field `WR_BYTE_ORDER` reader - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian"]
pub struct WR_BYTE_ORDER_R(crate::FieldReader<bool, bool>);
impl WR_BYTE_ORDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WR_BYTE_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_BYTE_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_BYTE_ORDER` writer - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian"]
pub struct WR_BYTE_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_BYTE_ORDER_W<'a> {
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
#[doc = "Field `FWRITE_DUAL` reader - In the write operations read-data phase apply 2 signals"]
pub struct FWRITE_DUAL_R(crate::FieldReader<bool, bool>);
impl FWRITE_DUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FWRITE_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWRITE_DUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWRITE_DUAL` writer - In the write operations read-data phase apply 2 signals"]
pub struct FWRITE_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FWRITE_DUAL_W<'a> {
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
#[doc = "Field `FWRITE_QUAD` reader - In the write operations read-data phase apply 4 signals"]
pub struct FWRITE_QUAD_R(crate::FieldReader<bool, bool>);
impl FWRITE_QUAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FWRITE_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWRITE_QUAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWRITE_QUAD` writer - In the write operations read-data phase apply 4 signals"]
pub struct FWRITE_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FWRITE_QUAD_W<'a> {
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
#[doc = "Field `FWRITE_DIO` reader - In the write operations address phase and read-data phase apply 2 signals."]
pub struct FWRITE_DIO_R(crate::FieldReader<bool, bool>);
impl FWRITE_DIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FWRITE_DIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWRITE_DIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWRITE_DIO` writer - In the write operations address phase and read-data phase apply 2 signals."]
pub struct FWRITE_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> FWRITE_DIO_W<'a> {
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
#[doc = "Field `FWRITE_QIO` reader - In the write operations address phase and read-data phase apply 4 signals."]
pub struct FWRITE_QIO_R(crate::FieldReader<bool, bool>);
impl FWRITE_QIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FWRITE_QIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWRITE_QIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWRITE_QIO` writer - In the write operations address phase and read-data phase apply 4 signals."]
pub struct FWRITE_QIO_W<'a> {
    w: &'a mut W,
}
impl<'a> FWRITE_QIO_W<'a> {
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
#[doc = "Field `SIO` reader - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable."]
pub struct SIO_R(crate::FieldReader<bool, bool>);
impl SIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIO` writer - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable."]
pub struct SIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `USR_HOLD_POL` reader - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low"]
pub struct USR_HOLD_POL_R(crate::FieldReader<bool, bool>);
impl USR_HOLD_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_HOLD_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_HOLD_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_HOLD_POL` writer - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low"]
pub struct USR_HOLD_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_HOLD_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `USR_DOUT_HOLD` reader - spi is hold at data out state the bit combined with spi_usr_hold_pol bit."]
pub struct USR_DOUT_HOLD_R(crate::FieldReader<bool, bool>);
impl USR_DOUT_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_DOUT_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_DOUT_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_DOUT_HOLD` writer - spi is hold at data out state the bit combined with spi_usr_hold_pol bit."]
pub struct USR_DOUT_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DOUT_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `USR_DIN_HOLD` reader - spi is hold at data in state the bit combined with spi_usr_hold_pol bit."]
pub struct USR_DIN_HOLD_R(crate::FieldReader<bool, bool>);
impl USR_DIN_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_DIN_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_DIN_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_DIN_HOLD` writer - spi is hold at data in state the bit combined with spi_usr_hold_pol bit."]
pub struct USR_DIN_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DIN_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `USR_DUMMY_HOLD` reader - spi is hold at dummy state the bit combined with spi_usr_hold_pol bit."]
pub struct USR_DUMMY_HOLD_R(crate::FieldReader<bool, bool>);
impl USR_DUMMY_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_DUMMY_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_DUMMY_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_DUMMY_HOLD` writer - spi is hold at dummy state the bit combined with spi_usr_hold_pol bit."]
pub struct USR_DUMMY_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DUMMY_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `USR_ADDR_HOLD` reader - spi is hold at address state the bit combined with spi_usr_hold_pol bit."]
pub struct USR_ADDR_HOLD_R(crate::FieldReader<bool, bool>);
impl USR_ADDR_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_ADDR_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_ADDR_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_ADDR_HOLD` writer - spi is hold at address state the bit combined with spi_usr_hold_pol bit."]
pub struct USR_ADDR_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_ADDR_HOLD_W<'a> {
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
#[doc = "Field `USR_CMD_HOLD` reader - spi is hold at command state the bit combined with spi_usr_hold_pol bit."]
pub struct USR_CMD_HOLD_R(crate::FieldReader<bool, bool>);
impl USR_CMD_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_CMD_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_CMD_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_CMD_HOLD` writer - spi is hold at command state the bit combined with spi_usr_hold_pol bit."]
pub struct USR_CMD_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_CMD_HOLD_W<'a> {
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
#[doc = "Field `USR_PREP_HOLD` reader - spi is hold at prepare state the bit combined with spi_usr_hold_pol bit."]
pub struct USR_PREP_HOLD_R(crate::FieldReader<bool, bool>);
impl USR_PREP_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_PREP_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_PREP_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_PREP_HOLD` writer - spi is hold at prepare state the bit combined with spi_usr_hold_pol bit."]
pub struct USR_PREP_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_PREP_HOLD_W<'a> {
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
#[doc = "Field `USR_MISO_HIGHPART` reader - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
pub struct USR_MISO_HIGHPART_R(crate::FieldReader<bool, bool>);
impl USR_MISO_HIGHPART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_MISO_HIGHPART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_MISO_HIGHPART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_MISO_HIGHPART` writer - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
pub struct USR_MISO_HIGHPART_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MISO_HIGHPART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `USR_MOSI_HIGHPART` reader - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
pub struct USR_MOSI_HIGHPART_R(crate::FieldReader<bool, bool>);
impl USR_MOSI_HIGHPART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_MOSI_HIGHPART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_MOSI_HIGHPART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_MOSI_HIGHPART` writer - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
pub struct USR_MOSI_HIGHPART_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MOSI_HIGHPART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `USR_DUMMY_IDLE` reader - spi clock is disable in dummy phase when the bit is enable."]
pub struct USR_DUMMY_IDLE_R(crate::FieldReader<bool, bool>);
impl USR_DUMMY_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_DUMMY_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_DUMMY_IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_DUMMY_IDLE` writer - spi clock is disable in dummy phase when the bit is enable."]
pub struct USR_DUMMY_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DUMMY_IDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `USR_MOSI` reader - This bit enable the write-data phase of an operation."]
pub struct USR_MOSI_R(crate::FieldReader<bool, bool>);
impl USR_MOSI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_MOSI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_MOSI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_MOSI` writer - This bit enable the write-data phase of an operation."]
pub struct USR_MOSI_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MOSI_W<'a> {
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
#[doc = "Field `USR_MISO` reader - This bit enable the read-data phase of an operation."]
pub struct USR_MISO_R(crate::FieldReader<bool, bool>);
impl USR_MISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_MISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_MISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_MISO` writer - This bit enable the read-data phase of an operation."]
pub struct USR_MISO_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MISO_W<'a> {
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
#[doc = "Field `USR_DUMMY` reader - This bit enable the dummy phase of an operation."]
pub struct USR_DUMMY_R(crate::FieldReader<bool, bool>);
impl USR_DUMMY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_DUMMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_DUMMY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_DUMMY` writer - This bit enable the dummy phase of an operation."]
pub struct USR_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DUMMY_W<'a> {
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
#[doc = "Field `USR_ADDR` reader - This bit enable the address phase of an operation."]
pub struct USR_ADDR_R(crate::FieldReader<bool, bool>);
impl USR_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_ADDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_ADDR` writer - This bit enable the address phase of an operation."]
pub struct USR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_ADDR_W<'a> {
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
#[doc = "Field `USR_COMMAND` reader - This bit enable the command phase of an operation."]
pub struct USR_COMMAND_R(crate::FieldReader<bool, bool>);
impl USR_COMMAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_COMMAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_COMMAND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_COMMAND` writer - This bit enable the command phase of an operation."]
pub struct USR_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_COMMAND_W<'a> {
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
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable."]
    #[inline(always)]
    pub fn doutdin(&self) -> DOUTDIN_R {
        DOUTDIN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - spi cs keep low when spi is in ¡°done¡± phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_hold(&self) -> CS_HOLD_R {
        CS_HOLD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - spi cs is enable when spi is in ¡°prepare¡± phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_setup(&self) -> CS_SETUP_R {
        CS_SETUP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - In the slave mode the bit is same as spi_ck_out_edge in master mode. It is combined with spi_miso_delay_mode bits."]
    #[inline(always)]
    pub fn ck_i_edge(&self) -> CK_I_EDGE_R {
        CK_I_EDGE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode."]
    #[inline(always)]
    pub fn ck_out_edge(&self) -> CK_OUT_EDGE_R {
        CK_OUT_EDGE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - In read-data (MISO) phase 1: big-endian 0: little_endian"]
    #[inline(always)]
    pub fn rd_byte_order(&self) -> RD_BYTE_ORDER_R {
        RD_BYTE_ORDER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian"]
    #[inline(always)]
    pub fn wr_byte_order(&self) -> WR_BYTE_ORDER_R {
        WR_BYTE_ORDER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals"]
    #[inline(always)]
    pub fn fwrite_dual(&self) -> FWRITE_DUAL_R {
        FWRITE_DUAL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals"]
    #[inline(always)]
    pub fn fwrite_quad(&self) -> FWRITE_QUAD_R {
        FWRITE_QUAD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - In the write operations address phase and read-data phase apply 2 signals."]
    #[inline(always)]
    pub fn fwrite_dio(&self) -> FWRITE_DIO_R {
        FWRITE_DIO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - In the write operations address phase and read-data phase apply 4 signals."]
    #[inline(always)]
    pub fn fwrite_qio(&self) -> FWRITE_QIO_R {
        FWRITE_QIO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sio(&self) -> SIO_R {
        SIO_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low"]
    #[inline(always)]
    pub fn usr_hold_pol(&self) -> USR_HOLD_POL_R {
        USR_HOLD_POL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - spi is hold at data out state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_dout_hold(&self) -> USR_DOUT_HOLD_R {
        USR_DOUT_HOLD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - spi is hold at data in state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_din_hold(&self) -> USR_DIN_HOLD_R {
        USR_DIN_HOLD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - spi is hold at dummy state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_dummy_hold(&self) -> USR_DUMMY_HOLD_R {
        USR_DUMMY_HOLD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - spi is hold at address state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_addr_hold(&self) -> USR_ADDR_HOLD_R {
        USR_ADDR_HOLD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - spi is hold at command state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_cmd_hold(&self) -> USR_CMD_HOLD_R {
        USR_CMD_HOLD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - spi is hold at prepare state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_prep_hold(&self) -> USR_PREP_HOLD_R {
        USR_PREP_HOLD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_miso_highpart(&self) -> USR_MISO_HIGHPART_R {
        USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_mosi_highpart(&self) -> USR_MOSI_HIGHPART_R {
        USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn usr_dummy_idle(&self) -> USR_DUMMY_IDLE_R {
        USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation."]
    #[inline(always)]
    pub fn usr_mosi(&self) -> USR_MOSI_R {
        USR_MOSI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation."]
    #[inline(always)]
    pub fn usr_miso(&self) -> USR_MISO_R {
        USR_MISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn usr_dummy(&self) -> USR_DUMMY_R {
        USR_DUMMY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation."]
    #[inline(always)]
    pub fn usr_addr(&self) -> USR_ADDR_R {
        USR_ADDR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation."]
    #[inline(always)]
    pub fn usr_command(&self) -> USR_COMMAND_R {
        USR_COMMAND_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable."]
    #[inline(always)]
    pub fn doutdin(&mut self) -> DOUTDIN_W {
        DOUTDIN_W { w: self }
    }
    #[doc = "Bit 4 - spi cs keep low when spi is in ¡°done¡± phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_hold(&mut self) -> CS_HOLD_W {
        CS_HOLD_W { w: self }
    }
    #[doc = "Bit 5 - spi cs is enable when spi is in ¡°prepare¡± phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_setup(&mut self) -> CS_SETUP_W {
        CS_SETUP_W { w: self }
    }
    #[doc = "Bit 6 - In the slave mode the bit is same as spi_ck_out_edge in master mode. It is combined with spi_miso_delay_mode bits."]
    #[inline(always)]
    pub fn ck_i_edge(&mut self) -> CK_I_EDGE_W {
        CK_I_EDGE_W { w: self }
    }
    #[doc = "Bit 7 - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode."]
    #[inline(always)]
    pub fn ck_out_edge(&mut self) -> CK_OUT_EDGE_W {
        CK_OUT_EDGE_W { w: self }
    }
    #[doc = "Bit 10 - In read-data (MISO) phase 1: big-endian 0: little_endian"]
    #[inline(always)]
    pub fn rd_byte_order(&mut self) -> RD_BYTE_ORDER_W {
        RD_BYTE_ORDER_W { w: self }
    }
    #[doc = "Bit 11 - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian"]
    #[inline(always)]
    pub fn wr_byte_order(&mut self) -> WR_BYTE_ORDER_W {
        WR_BYTE_ORDER_W { w: self }
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals"]
    #[inline(always)]
    pub fn fwrite_dual(&mut self) -> FWRITE_DUAL_W {
        FWRITE_DUAL_W { w: self }
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals"]
    #[inline(always)]
    pub fn fwrite_quad(&mut self) -> FWRITE_QUAD_W {
        FWRITE_QUAD_W { w: self }
    }
    #[doc = "Bit 14 - In the write operations address phase and read-data phase apply 2 signals."]
    #[inline(always)]
    pub fn fwrite_dio(&mut self) -> FWRITE_DIO_W {
        FWRITE_DIO_W { w: self }
    }
    #[doc = "Bit 15 - In the write operations address phase and read-data phase apply 4 signals."]
    #[inline(always)]
    pub fn fwrite_qio(&mut self) -> FWRITE_QIO_W {
        FWRITE_QIO_W { w: self }
    }
    #[doc = "Bit 16 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sio(&mut self) -> SIO_W {
        SIO_W { w: self }
    }
    #[doc = "Bit 17 - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low"]
    #[inline(always)]
    pub fn usr_hold_pol(&mut self) -> USR_HOLD_POL_W {
        USR_HOLD_POL_W { w: self }
    }
    #[doc = "Bit 18 - spi is hold at data out state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_dout_hold(&mut self) -> USR_DOUT_HOLD_W {
        USR_DOUT_HOLD_W { w: self }
    }
    #[doc = "Bit 19 - spi is hold at data in state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_din_hold(&mut self) -> USR_DIN_HOLD_W {
        USR_DIN_HOLD_W { w: self }
    }
    #[doc = "Bit 20 - spi is hold at dummy state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_dummy_hold(&mut self) -> USR_DUMMY_HOLD_W {
        USR_DUMMY_HOLD_W { w: self }
    }
    #[doc = "Bit 21 - spi is hold at address state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_addr_hold(&mut self) -> USR_ADDR_HOLD_W {
        USR_ADDR_HOLD_W { w: self }
    }
    #[doc = "Bit 22 - spi is hold at command state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_cmd_hold(&mut self) -> USR_CMD_HOLD_W {
        USR_CMD_HOLD_W { w: self }
    }
    #[doc = "Bit 23 - spi is hold at prepare state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_prep_hold(&mut self) -> USR_PREP_HOLD_W {
        USR_PREP_HOLD_W { w: self }
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_miso_highpart(&mut self) -> USR_MISO_HIGHPART_W {
        USR_MISO_HIGHPART_W { w: self }
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_mosi_highpart(&mut self) -> USR_MOSI_HIGHPART_W {
        USR_MOSI_HIGHPART_W { w: self }
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn usr_dummy_idle(&mut self) -> USR_DUMMY_IDLE_W {
        USR_DUMMY_IDLE_W { w: self }
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation."]
    #[inline(always)]
    pub fn usr_mosi(&mut self) -> USR_MOSI_W {
        USR_MOSI_W { w: self }
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation."]
    #[inline(always)]
    pub fn usr_miso(&mut self) -> USR_MISO_W {
        USR_MISO_W { w: self }
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn usr_dummy(&mut self) -> USR_DUMMY_W {
        USR_DUMMY_W { w: self }
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation."]
    #[inline(always)]
    pub fn usr_addr(&mut self) -> USR_ADDR_W {
        USR_ADDR_W { w: self }
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation."]
    #[inline(always)]
    pub fn usr_command(&mut self) -> USR_COMMAND_W {
        USR_COMMAND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user](index.html) module"]
pub struct USER_SPEC;
impl crate::RegisterSpec for USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user::R](R) reader structure"]
impl crate::Readable for USER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user::W](W) writer structure"]
impl crate::Writable for USER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USER to value 0x8000_0040"]
impl crate::Resettable for USER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0040
    }
}
