#[doc = "Register `SDIO_CONF` reader"]
pub struct R(crate::R<SDIO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_CONF` writer"]
pub struct W(crate::W<SDIO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_CONF_SPEC>;
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
impl From<crate::W<SDIO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_PD_EN` reader - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
pub struct SDIO_PD_EN_R(crate::FieldReader<bool, bool>);
impl SDIO_PD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_PD_EN` writer - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
pub struct SDIO_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_PD_EN_W<'a> {
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
#[doc = "Field `SDIO_FORCE` reader - 1: use SW option to control SDIO_REG 0: use state machine"]
pub struct SDIO_FORCE_R(crate::FieldReader<bool, bool>);
impl SDIO_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_FORCE` writer - 1: use SW option to control SDIO_REG 0: use state machine"]
pub struct SDIO_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_FORCE_W<'a> {
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
#[doc = "Field `SDIO_TIEH` reader - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
pub struct SDIO_TIEH_R(crate::FieldReader<bool, bool>);
impl SDIO_TIEH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_TIEH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_TIEH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_TIEH` writer - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
pub struct SDIO_TIEH_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_TIEH_W<'a> {
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
#[doc = "Field `REG1P8_READY` reader - read only register for REG1P8_READY"]
pub struct REG1P8_READY_R(crate::FieldReader<bool, bool>);
impl REG1P8_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG1P8_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG1P8_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREFL_SDIO` reader - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
pub struct DREFL_SDIO_R(crate::FieldReader<u8, u8>);
impl DREFL_SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DREFL_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DREFL_SDIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREFL_SDIO` writer - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
pub struct DREFL_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DREFL_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `DREFM_SDIO` reader - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
pub struct DREFM_SDIO_R(crate::FieldReader<u8, u8>);
impl DREFM_SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DREFM_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DREFM_SDIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREFM_SDIO` writer - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
pub struct DREFM_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DREFM_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `DREFH_SDIO` reader - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
pub struct DREFH_SDIO_R(crate::FieldReader<u8, u8>);
impl DREFH_SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DREFH_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DREFH_SDIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREFH_SDIO` writer - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
pub struct DREFH_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DREFH_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "Field `XPD_SDIO` reader - SW option for XPD_SDIO_REG. Only active when reg_sdio_force = 1"]
pub struct XPD_SDIO_R(crate::FieldReader<bool, bool>);
impl XPD_SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XPD_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_SDIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_SDIO` writer - SW option for XPD_SDIO_REG. Only active when reg_sdio_force = 1"]
pub struct XPD_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SDIO_W<'a> {
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
    #[doc = "Bit 21 - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
    #[inline(always)]
    pub fn sdio_pd_en(&self) -> SDIO_PD_EN_R {
        SDIO_PD_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 1: use SW option to control SDIO_REG 0: use state machine"]
    #[inline(always)]
    pub fn sdio_force(&self) -> SDIO_FORCE_R {
        SDIO_FORCE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn sdio_tieh(&self) -> SDIO_TIEH_R {
        SDIO_TIEH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - read only register for REG1P8_READY"]
    #[inline(always)]
    pub fn reg1p8_ready(&self) -> REG1P8_READY_R {
        REG1P8_READY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefl_sdio(&self) -> DREFL_SDIO_R {
        DREFL_SDIO_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 27:28 - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefm_sdio(&self) -> DREFM_SDIO_R {
        DREFM_SDIO_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 29:30 - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefh_sdio(&self) -> DREFH_SDIO_R {
        DREFH_SDIO_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 31 - SW option for XPD_SDIO_REG. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn xpd_sdio(&self) -> XPD_SDIO_R {
        XPD_SDIO_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
    #[inline(always)]
    pub fn sdio_pd_en(&mut self) -> SDIO_PD_EN_W {
        SDIO_PD_EN_W { w: self }
    }
    #[doc = "Bit 22 - 1: use SW option to control SDIO_REG 0: use state machine"]
    #[inline(always)]
    pub fn sdio_force(&mut self) -> SDIO_FORCE_W {
        SDIO_FORCE_W { w: self }
    }
    #[doc = "Bit 23 - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn sdio_tieh(&mut self) -> SDIO_TIEH_W {
        SDIO_TIEH_W { w: self }
    }
    #[doc = "Bits 25:26 - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefl_sdio(&mut self) -> DREFL_SDIO_W {
        DREFL_SDIO_W { w: self }
    }
    #[doc = "Bits 27:28 - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefm_sdio(&mut self) -> DREFM_SDIO_W {
        DREFM_SDIO_W { w: self }
    }
    #[doc = "Bits 29:30 - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefh_sdio(&mut self) -> DREFH_SDIO_W {
        DREFH_SDIO_W { w: self }
    }
    #[doc = "Bit 31 - SW option for XPD_SDIO_REG. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn xpd_sdio(&mut self) -> XPD_SDIO_W {
        XPD_SDIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_conf](index.html) module"]
pub struct SDIO_CONF_SPEC;
impl crate::RegisterSpec for SDIO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_conf::R](R) reader structure"]
impl crate::Readable for SDIO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_conf::W](W) writer structure"]
impl crate::Writable for SDIO_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIO_CONF to value 0x02a0_0000"]
impl crate::Resettable for SDIO_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02a0_0000
    }
}
