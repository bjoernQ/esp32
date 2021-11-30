#[doc = "Register `ESCO_CONF0` reader"]
pub struct R(crate::R<ESCO_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESCO_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESCO_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESCO_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESCO_CONF0` writer"]
pub struct W(crate::W<ESCO_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESCO_CONF0_SPEC>;
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
impl From<crate::W<ESCO_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESCO_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESCO_EN` reader - "]
pub struct ESCO_EN_R(crate::FieldReader<bool, bool>);
impl ESCO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESCO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESCO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESCO_EN` writer - "]
pub struct ESCO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCO_EN_W<'a> {
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
#[doc = "Field `ESCO_CHAN_MOD` reader - "]
pub struct ESCO_CHAN_MOD_R(crate::FieldReader<bool, bool>);
impl ESCO_CHAN_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESCO_CHAN_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESCO_CHAN_MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESCO_CHAN_MOD` writer - "]
pub struct ESCO_CHAN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCO_CHAN_MOD_W<'a> {
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
#[doc = "Field `ESCO_CVSD_DEC_PACK_ERR` reader - "]
pub struct ESCO_CVSD_DEC_PACK_ERR_R(crate::FieldReader<bool, bool>);
impl ESCO_CVSD_DEC_PACK_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESCO_CVSD_DEC_PACK_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESCO_CVSD_DEC_PACK_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESCO_CVSD_DEC_PACK_ERR` writer - "]
pub struct ESCO_CVSD_DEC_PACK_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCO_CVSD_DEC_PACK_ERR_W<'a> {
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
#[doc = "Field `ESCO_CVSD_PACK_LEN_8K` reader - "]
pub struct ESCO_CVSD_PACK_LEN_8K_R(crate::FieldReader<u8, u8>);
impl ESCO_CVSD_PACK_LEN_8K_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ESCO_CVSD_PACK_LEN_8K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESCO_CVSD_PACK_LEN_8K_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESCO_CVSD_PACK_LEN_8K` writer - "]
pub struct ESCO_CVSD_PACK_LEN_8K_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCO_CVSD_PACK_LEN_8K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `ESCO_CVSD_INF_EN` reader - "]
pub struct ESCO_CVSD_INF_EN_R(crate::FieldReader<bool, bool>);
impl ESCO_CVSD_INF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESCO_CVSD_INF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESCO_CVSD_INF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESCO_CVSD_INF_EN` writer - "]
pub struct ESCO_CVSD_INF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCO_CVSD_INF_EN_W<'a> {
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
#[doc = "Field `CVSD_DEC_START` reader - "]
pub struct CVSD_DEC_START_R(crate::FieldReader<bool, bool>);
impl CVSD_DEC_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CVSD_DEC_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVSD_DEC_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVSD_DEC_START` writer - "]
pub struct CVSD_DEC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> CVSD_DEC_START_W<'a> {
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
#[doc = "Field `CVSD_DEC_RESET` reader - "]
pub struct CVSD_DEC_RESET_R(crate::FieldReader<bool, bool>);
impl CVSD_DEC_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CVSD_DEC_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVSD_DEC_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVSD_DEC_RESET` writer - "]
pub struct CVSD_DEC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CVSD_DEC_RESET_W<'a> {
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
#[doc = "Field `PLC_EN` reader - "]
pub struct PLC_EN_R(crate::FieldReader<bool, bool>);
impl PLC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLC_EN` writer - "]
pub struct PLC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLC_EN_W<'a> {
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
#[doc = "Field `PLC2DMA_EN` reader - "]
pub struct PLC2DMA_EN_R(crate::FieldReader<bool, bool>);
impl PLC2DMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLC2DMA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLC2DMA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLC2DMA_EN` writer - "]
pub struct PLC2DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLC2DMA_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn esco_en(&self) -> ESCO_EN_R {
        ESCO_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn esco_chan_mod(&self) -> ESCO_CHAN_MOD_R {
        ESCO_CHAN_MOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn esco_cvsd_dec_pack_err(&self) -> ESCO_CVSD_DEC_PACK_ERR_R {
        ESCO_CVSD_DEC_PACK_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn esco_cvsd_pack_len_8k(&self) -> ESCO_CVSD_PACK_LEN_8K_R {
        ESCO_CVSD_PACK_LEN_8K_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn esco_cvsd_inf_en(&self) -> ESCO_CVSD_INF_EN_R {
        ESCO_CVSD_INF_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cvsd_dec_start(&self) -> CVSD_DEC_START_R {
        CVSD_DEC_START_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cvsd_dec_reset(&self) -> CVSD_DEC_RESET_R {
        CVSD_DEC_RESET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn plc_en(&self) -> PLC_EN_R {
        PLC_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn plc2dma_en(&self) -> PLC2DMA_EN_R {
        PLC2DMA_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn esco_en(&mut self) -> ESCO_EN_W {
        ESCO_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn esco_chan_mod(&mut self) -> ESCO_CHAN_MOD_W {
        ESCO_CHAN_MOD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn esco_cvsd_dec_pack_err(&mut self) -> ESCO_CVSD_DEC_PACK_ERR_W {
        ESCO_CVSD_DEC_PACK_ERR_W { w: self }
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn esco_cvsd_pack_len_8k(&mut self) -> ESCO_CVSD_PACK_LEN_8K_W {
        ESCO_CVSD_PACK_LEN_8K_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn esco_cvsd_inf_en(&mut self) -> ESCO_CVSD_INF_EN_W {
        ESCO_CVSD_INF_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cvsd_dec_start(&mut self) -> CVSD_DEC_START_W {
        CVSD_DEC_START_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cvsd_dec_reset(&mut self) -> CVSD_DEC_RESET_W {
        CVSD_DEC_RESET_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn plc_en(&mut self) -> PLC_EN_W {
        PLC_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn plc2dma_en(&mut self) -> PLC2DMA_EN_W {
        PLC2DMA_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esco_conf0](index.html) module"]
pub struct ESCO_CONF0_SPEC;
impl crate::RegisterSpec for ESCO_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esco_conf0::R](R) reader structure"]
impl crate::Readable for ESCO_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esco_conf0::W](W) writer structure"]
impl crate::Writable for ESCO_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ESCO_CONF0 to value 0"]
impl crate::Resettable for ESCO_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
