#[doc = "Register `GEN1_A` reader"]
pub struct R(crate::R<GEN1_A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN1_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN1_A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN1_A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN1_A` writer"]
pub struct W(crate::W<GEN1_A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN1_A_SPEC>;
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
impl From<crate::W<GEN1_A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN1_A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTEZ` reader - "]
pub struct UTEZ_R(crate::FieldReader<u8, u8>);
impl UTEZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UTEZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTEZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTEZ` writer - "]
pub struct UTEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> UTEZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `UTEP` reader - "]
pub struct UTEP_R(crate::FieldReader<u8, u8>);
impl UTEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UTEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTEP` writer - "]
pub struct UTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> UTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `UTEA` reader - "]
pub struct UTEA_R(crate::FieldReader<u8, u8>);
impl UTEA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UTEA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTEA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTEA` writer - "]
pub struct UTEA_W<'a> {
    w: &'a mut W,
}
impl<'a> UTEA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `UTEB` reader - "]
pub struct UTEB_R(crate::FieldReader<u8, u8>);
impl UTEB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UTEB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTEB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTEB` writer - "]
pub struct UTEB_W<'a> {
    w: &'a mut W,
}
impl<'a> UTEB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `UT0` reader - "]
pub struct UT0_R(crate::FieldReader<u8, u8>);
impl UT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UT0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UT0` writer - "]
pub struct UT0_W<'a> {
    w: &'a mut W,
}
impl<'a> UT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `UT1` reader - "]
pub struct UT1_R(crate::FieldReader<u8, u8>);
impl UT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UT1` writer - "]
pub struct UT1_W<'a> {
    w: &'a mut W,
}
impl<'a> UT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `DTEZ` reader - "]
pub struct DTEZ_R(crate::FieldReader<u8, u8>);
impl DTEZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTEZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEZ` writer - "]
pub struct DTEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `DTEP` reader - "]
pub struct DTEP_R(crate::FieldReader<u8, u8>);
impl DTEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEP` writer - "]
pub struct DTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `DTEA` reader - "]
pub struct DTEA_R(crate::FieldReader<u8, u8>);
impl DTEA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTEA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEA` writer - "]
pub struct DTEA_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `DTEB` reader - "]
pub struct DTEB_R(crate::FieldReader<u8, u8>);
impl DTEB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTEB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEB` writer - "]
pub struct DTEB_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `DT0` reader - "]
pub struct DT0_R(crate::FieldReader<u8, u8>);
impl DT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT0` writer - "]
pub struct DT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `DT1` reader - "]
pub struct DT1_R(crate::FieldReader<u8, u8>);
impl DT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT1` writer - "]
pub struct DT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn utez(&self) -> UTEZ_R {
        UTEZ_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn utep(&self) -> UTEP_R {
        UTEP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn utea(&self) -> UTEA_R {
        UTEA_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn uteb(&self) -> UTEB_R {
        UTEB_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ut0(&self) -> UT0_R {
        UT0_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dtez(&self) -> DTEZ_R {
        DTEZ_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn dtep(&self) -> DTEP_R {
        DTEP_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dtea(&self) -> DTEA_R {
        DTEA_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn dteb(&self) -> DTEB_R {
        DTEB_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dt0(&self) -> DT0_R {
        DT0_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn utez(&mut self) -> UTEZ_W {
        UTEZ_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn utep(&mut self) -> UTEP_W {
        UTEP_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn utea(&mut self) -> UTEA_W {
        UTEA_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn uteb(&mut self) -> UTEB_W {
        UTEB_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ut0(&mut self) -> UT0_W {
        UT0_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn ut1(&mut self) -> UT1_W {
        UT1_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dtez(&mut self) -> DTEZ_W {
        DTEZ_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn dtep(&mut self) -> DTEP_W {
        DTEP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dtea(&mut self) -> DTEA_W {
        DTEA_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn dteb(&mut self) -> DTEB_W {
        DTEB_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dt0(&mut self) -> DT0_W {
        DT0_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn dt1(&mut self) -> DT1_W {
        DT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen1_a](index.html) module"]
pub struct GEN1_A_SPEC;
impl crate::RegisterSpec for GEN1_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen1_a::R](R) reader structure"]
impl crate::Readable for GEN1_A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen1_a::W](W) writer structure"]
impl crate::Writable for GEN1_A_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN1_A to value 0"]
impl crate::Resettable for GEN1_A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
