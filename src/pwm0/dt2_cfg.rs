#[doc = "Register `DT2_CFG` reader"]
pub struct R(crate::R<DT2_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DT2_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DT2_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DT2_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DT2_CFG` writer"]
pub struct W(crate::W<DT2_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DT2_CFG_SPEC>;
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
impl From<crate::W<DT2_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DT2_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT2_FED_UPMETHOD` reader - "]
pub struct DT2_FED_UPMETHOD_R(crate::FieldReader<u8, u8>);
impl DT2_FED_UPMETHOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DT2_FED_UPMETHOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2_FED_UPMETHOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2_FED_UPMETHOD` writer - "]
pub struct DT2_FED_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_FED_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `DT2_RED_UPMETHOD` reader - "]
pub struct DT2_RED_UPMETHOD_R(crate::FieldReader<u8, u8>);
impl DT2_RED_UPMETHOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DT2_RED_UPMETHOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2_RED_UPMETHOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2_RED_UPMETHOD` writer - "]
pub struct DT2_RED_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_RED_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `DT2_DEB_MODE` reader - "]
pub struct DT2_DEB_MODE_R(crate::FieldReader<bool, bool>);
impl DT2_DEB_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DT2_DEB_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2_DEB_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2_DEB_MODE` writer - "]
pub struct DT2_DEB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_DEB_MODE_W<'a> {
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
#[doc = "Field `DT2_A_OUTSWAP` reader - "]
pub struct DT2_A_OUTSWAP_R(crate::FieldReader<bool, bool>);
impl DT2_A_OUTSWAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DT2_A_OUTSWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2_A_OUTSWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2_A_OUTSWAP` writer - "]
pub struct DT2_A_OUTSWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_A_OUTSWAP_W<'a> {
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
#[doc = "Field `DT2_B_OUTSWAP` reader - "]
pub struct DT2_B_OUTSWAP_R(crate::FieldReader<bool, bool>);
impl DT2_B_OUTSWAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DT2_B_OUTSWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2_B_OUTSWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2_B_OUTSWAP` writer - "]
pub struct DT2_B_OUTSWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_B_OUTSWAP_W<'a> {
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
#[doc = "Field `DT2_RED_INSEL` reader - "]
pub struct DT2_RED_INSEL_R(crate::FieldReader<bool, bool>);
impl DT2_RED_INSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DT2_RED_INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2_RED_INSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2_RED_INSEL` writer - "]
pub struct DT2_RED_INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_RED_INSEL_W<'a> {
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
#[doc = "Field `DT2_FED_INSEL` reader - "]
pub struct DT2_FED_INSEL_R(crate::FieldReader<bool, bool>);
impl DT2_FED_INSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DT2_FED_INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2_FED_INSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2_FED_INSEL` writer - "]
pub struct DT2_FED_INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_FED_INSEL_W<'a> {
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
#[doc = "Field `DT2_RED_OUTINVERT` reader - "]
pub struct DT2_RED_OUTINVERT_R(crate::FieldReader<bool, bool>);
impl DT2_RED_OUTINVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DT2_RED_OUTINVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2_RED_OUTINVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2_RED_OUTINVERT` writer - "]
pub struct DT2_RED_OUTINVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_RED_OUTINVERT_W<'a> {
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
#[doc = "Field `DT2_FED_OUTINVERT` reader - "]
pub struct DT2_FED_OUTINVERT_R(crate::FieldReader<bool, bool>);
impl DT2_FED_OUTINVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DT2_FED_OUTINVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2_FED_OUTINVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2_FED_OUTINVERT` writer - "]
pub struct DT2_FED_OUTINVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_FED_OUTINVERT_W<'a> {
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
#[doc = "Field `DT2_A_OUTBYPASS` reader - "]
pub struct DT2_A_OUTBYPASS_R(crate::FieldReader<bool, bool>);
impl DT2_A_OUTBYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DT2_A_OUTBYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2_A_OUTBYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2_A_OUTBYPASS` writer - "]
pub struct DT2_A_OUTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_A_OUTBYPASS_W<'a> {
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
#[doc = "Field `DT2_B_OUTBYPASS` reader - "]
pub struct DT2_B_OUTBYPASS_R(crate::FieldReader<bool, bool>);
impl DT2_B_OUTBYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DT2_B_OUTBYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2_B_OUTBYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2_B_OUTBYPASS` writer - "]
pub struct DT2_B_OUTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_B_OUTBYPASS_W<'a> {
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
#[doc = "Field `DT2_CLK_SEL` reader - "]
pub struct DT2_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl DT2_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DT2_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2_CLK_SEL` writer - "]
pub struct DT2_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_CLK_SEL_W<'a> {
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
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dt2_fed_upmethod(&self) -> DT2_FED_UPMETHOD_R {
        DT2_FED_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dt2_red_upmethod(&self) -> DT2_RED_UPMETHOD_R {
        DT2_RED_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dt2_deb_mode(&self) -> DT2_DEB_MODE_R {
        DT2_DEB_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dt2_a_outswap(&self) -> DT2_A_OUTSWAP_R {
        DT2_A_OUTSWAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dt2_b_outswap(&self) -> DT2_B_OUTSWAP_R {
        DT2_B_OUTSWAP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dt2_red_insel(&self) -> DT2_RED_INSEL_R {
        DT2_RED_INSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dt2_fed_insel(&self) -> DT2_FED_INSEL_R {
        DT2_FED_INSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dt2_red_outinvert(&self) -> DT2_RED_OUTINVERT_R {
        DT2_RED_OUTINVERT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dt2_fed_outinvert(&self) -> DT2_FED_OUTINVERT_R {
        DT2_FED_OUTINVERT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dt2_a_outbypass(&self) -> DT2_A_OUTBYPASS_R {
        DT2_A_OUTBYPASS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dt2_b_outbypass(&self) -> DT2_B_OUTBYPASS_R {
        DT2_B_OUTBYPASS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dt2_clk_sel(&self) -> DT2_CLK_SEL_R {
        DT2_CLK_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dt2_fed_upmethod(&mut self) -> DT2_FED_UPMETHOD_W {
        DT2_FED_UPMETHOD_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dt2_red_upmethod(&mut self) -> DT2_RED_UPMETHOD_W {
        DT2_RED_UPMETHOD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dt2_deb_mode(&mut self) -> DT2_DEB_MODE_W {
        DT2_DEB_MODE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dt2_a_outswap(&mut self) -> DT2_A_OUTSWAP_W {
        DT2_A_OUTSWAP_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dt2_b_outswap(&mut self) -> DT2_B_OUTSWAP_W {
        DT2_B_OUTSWAP_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dt2_red_insel(&mut self) -> DT2_RED_INSEL_W {
        DT2_RED_INSEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dt2_fed_insel(&mut self) -> DT2_FED_INSEL_W {
        DT2_FED_INSEL_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dt2_red_outinvert(&mut self) -> DT2_RED_OUTINVERT_W {
        DT2_RED_OUTINVERT_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dt2_fed_outinvert(&mut self) -> DT2_FED_OUTINVERT_W {
        DT2_FED_OUTINVERT_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dt2_a_outbypass(&mut self) -> DT2_A_OUTBYPASS_W {
        DT2_A_OUTBYPASS_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dt2_b_outbypass(&mut self) -> DT2_B_OUTBYPASS_W {
        DT2_B_OUTBYPASS_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dt2_clk_sel(&mut self) -> DT2_CLK_SEL_W {
        DT2_CLK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt2_cfg](index.html) module"]
pub struct DT2_CFG_SPEC;
impl crate::RegisterSpec for DT2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dt2_cfg::R](R) reader structure"]
impl crate::Readable for DT2_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dt2_cfg::W](W) writer structure"]
impl crate::Writable for DT2_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DT2_CFG to value 0x0001_8000"]
impl crate::Resettable for DT2_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_8000
    }
}
