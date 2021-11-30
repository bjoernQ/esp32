#[doc = "Register `CFG_DATA1` reader"]
pub struct R(crate::R<CFG_DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_DATA1` writer"]
pub struct W(crate::W<CFG_DATA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_DATA1_SPEC>;
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
impl From<crate::W<CFG_DATA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_DATA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_ENABLE` reader - "]
pub struct SDIO_ENABLE_R(crate::FieldReader<bool, bool>);
impl SDIO_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_ENABLE` writer - "]
pub struct SDIO_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_ENABLE_W<'a> {
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
#[doc = "Field `SDIO_IOREADY1` reader - "]
pub struct SDIO_IOREADY1_R(crate::FieldReader<bool, bool>);
impl SDIO_IOREADY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_IOREADY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_IOREADY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_IOREADY1` writer - "]
pub struct SDIO_IOREADY1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_IOREADY1_W<'a> {
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
#[doc = "Field `HIGHSPEED_ENABLE` reader - "]
pub struct HIGHSPEED_ENABLE_R(crate::FieldReader<bool, bool>);
impl HIGHSPEED_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HIGHSPEED_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIGHSPEED_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIGHSPEED_ENABLE` writer - "]
pub struct HIGHSPEED_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGHSPEED_ENABLE_W<'a> {
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
#[doc = "Field `HIGHSPEED_MODE` reader - "]
pub struct HIGHSPEED_MODE_R(crate::FieldReader<bool, bool>);
impl HIGHSPEED_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HIGHSPEED_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIGHSPEED_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_CD_ENABLE` reader - "]
pub struct SDIO_CD_ENABLE_R(crate::FieldReader<bool, bool>);
impl SDIO_CD_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_CD_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_CD_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_CD_ENABLE` writer - "]
pub struct SDIO_CD_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_CD_ENABLE_W<'a> {
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
#[doc = "Field `SDIO_IOREADY2` reader - "]
pub struct SDIO_IOREADY2_R(crate::FieldReader<bool, bool>);
impl SDIO_IOREADY2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_IOREADY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_IOREADY2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_IOREADY2` writer - "]
pub struct SDIO_IOREADY2_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_IOREADY2_W<'a> {
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
#[doc = "Field `SDIO_INT_MASK` reader - "]
pub struct SDIO_INT_MASK_R(crate::FieldReader<bool, bool>);
impl SDIO_INT_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_INT_MASK` writer - "]
pub struct SDIO_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_INT_MASK_W<'a> {
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
#[doc = "Field `IOENABLE2` reader - "]
pub struct IOENABLE2_R(crate::FieldReader<bool, bool>);
impl IOENABLE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IOENABLE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOENABLE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CD_DISABLE` reader - "]
pub struct CD_DISABLE_R(crate::FieldReader<bool, bool>);
impl CD_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CD_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CD_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC1_EPS` reader - "]
pub struct FUNC1_EPS_R(crate::FieldReader<bool, bool>);
impl FUNC1_EPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FUNC1_EPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUNC1_EPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMP` reader - "]
pub struct EMP_R(crate::FieldReader<bool, bool>);
impl EMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOENABLE1` reader - "]
pub struct IOENABLE1_R(crate::FieldReader<bool, bool>);
impl IOENABLE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IOENABLE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOENABLE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO20_CONF0` reader - "]
pub struct SDIO20_CONF0_R(crate::FieldReader<u8, u8>);
impl SDIO20_CONF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO20_CONF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO20_CONF0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO20_CONF0` writer - "]
pub struct SDIO20_CONF0_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO20_CONF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `SDIO_VER` reader - "]
pub struct SDIO_VER_R(crate::FieldReader<u16, u16>);
impl SDIO_VER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SDIO_VER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_VER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_VER` writer - "]
pub struct SDIO_VER_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_VER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `FUNC2_EPS` reader - "]
pub struct FUNC2_EPS_R(crate::FieldReader<bool, bool>);
impl FUNC2_EPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FUNC2_EPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUNC2_EPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO20_CONF1` reader - "]
pub struct SDIO20_CONF1_R(crate::FieldReader<u8, u8>);
impl SDIO20_CONF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO20_CONF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO20_CONF1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO20_CONF1` writer - "]
pub struct SDIO20_CONF1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO20_CONF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sdio_enable(&self) -> SDIO_ENABLE_R {
        SDIO_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sdio_ioready1(&self) -> SDIO_IOREADY1_R {
        SDIO_IOREADY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn highspeed_enable(&self) -> HIGHSPEED_ENABLE_R {
        HIGHSPEED_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn highspeed_mode(&self) -> HIGHSPEED_MODE_R {
        HIGHSPEED_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sdio_cd_enable(&self) -> SDIO_CD_ENABLE_R {
        SDIO_CD_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sdio_ioready2(&self) -> SDIO_IOREADY2_R {
        SDIO_IOREADY2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sdio_int_mask(&self) -> SDIO_INT_MASK_R {
        SDIO_INT_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ioenable2(&self) -> IOENABLE2_R {
        IOENABLE2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cd_disable(&self) -> CD_DISABLE_R {
        CD_DISABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn func1_eps(&self) -> FUNC1_EPS_R {
        FUNC1_EPS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn emp(&self) -> EMP_R {
        EMP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ioenable1(&self) -> IOENABLE1_R {
        IOENABLE1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn sdio20_conf0(&self) -> SDIO20_CONF0_R {
        SDIO20_CONF0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn sdio_ver(&self) -> SDIO_VER_R {
        SDIO_VER_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn func2_eps(&self) -> FUNC2_EPS_R {
        FUNC2_EPS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sdio20_conf1(&self) -> SDIO20_CONF1_R {
        SDIO20_CONF1_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sdio_enable(&mut self) -> SDIO_ENABLE_W {
        SDIO_ENABLE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sdio_ioready1(&mut self) -> SDIO_IOREADY1_W {
        SDIO_IOREADY1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn highspeed_enable(&mut self) -> HIGHSPEED_ENABLE_W {
        HIGHSPEED_ENABLE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sdio_cd_enable(&mut self) -> SDIO_CD_ENABLE_W {
        SDIO_CD_ENABLE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sdio_ioready2(&mut self) -> SDIO_IOREADY2_W {
        SDIO_IOREADY2_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sdio_int_mask(&mut self) -> SDIO_INT_MASK_W {
        SDIO_INT_MASK_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn sdio20_conf0(&mut self) -> SDIO20_CONF0_W {
        SDIO20_CONF0_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn sdio_ver(&mut self) -> SDIO_VER_W {
        SDIO_VER_W { w: self }
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sdio20_conf1(&mut self) -> SDIO20_CONF1_W {
        SDIO20_CONF1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_data1](index.html) module"]
pub struct CFG_DATA1_SPEC;
impl crate::RegisterSpec for CFG_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_data1::R](R) reader structure"]
impl crate::Readable for CFG_DATA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_data1::W](W) writer structure"]
impl crate::Writable for CFG_DATA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_DATA1 to value 0x0111_0011"]
impl crate::Resettable for CFG_DATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0111_0011
    }
}
