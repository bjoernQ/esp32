#[doc = "Register `WIFI_CLK_EN` reader"]
pub struct R(crate::R<WIFI_CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIFI_CLK_EN` writer"]
pub struct W(crate::W<WIFI_CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_CLK_EN_SPEC>;
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
impl From<crate::W<WIFI_CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIFI_CLK_EN` reader - "]
pub struct WIFI_CLK_EN_R(crate::FieldReader<u32, u32>);
impl WIFI_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WIFI_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_CLK_EN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_CLK_EN` writer - "]
pub struct WIFI_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_CLK_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
#[doc = "Field `WIFI_CLK_WIFI_EN` reader - "]
pub struct WIFI_CLK_WIFI_EN_R(crate::FieldReader<u8, u8>);
impl WIFI_CLK_WIFI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WIFI_CLK_WIFI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_CLK_WIFI_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_CLK_WIFI_EN` writer - "]
pub struct WIFI_CLK_WIFI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_CLK_WIFI_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `WIFI_CLK_WIFI_BT_COMMON` reader - "]
pub struct WIFI_CLK_WIFI_BT_COMMON_R(crate::FieldReader<u8, u8>);
impl WIFI_CLK_WIFI_BT_COMMON_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WIFI_CLK_WIFI_BT_COMMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_CLK_WIFI_BT_COMMON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_CLK_WIFI_BT_COMMON` writer - "]
pub struct WIFI_CLK_WIFI_BT_COMMON_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_CLK_WIFI_BT_COMMON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `WIFI_CLK_BT_EN` reader - "]
pub struct WIFI_CLK_BT_EN_R(crate::FieldReader<u8, u8>);
impl WIFI_CLK_BT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WIFI_CLK_BT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_CLK_BT_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_CLK_BT_EN` writer - "]
pub struct WIFI_CLK_BT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_CLK_BT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wifi_clk_en(&self) -> WIFI_CLK_EN_R {
        WIFI_CLK_EN_R::new(self.bits as u32)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn wifi_clk_wifi_en(&self) -> WIFI_CLK_WIFI_EN_R {
        WIFI_CLK_WIFI_EN_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn wifi_clk_wifi_bt_common(&self) -> WIFI_CLK_WIFI_BT_COMMON_R {
        WIFI_CLK_WIFI_BT_COMMON_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn wifi_clk_bt_en(&self) -> WIFI_CLK_BT_EN_R {
        WIFI_CLK_BT_EN_R::new(((self.bits >> 11) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wifi_clk_en(&mut self) -> WIFI_CLK_EN_W {
        WIFI_CLK_EN_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn wifi_clk_wifi_en(&mut self) -> WIFI_CLK_WIFI_EN_W {
        WIFI_CLK_WIFI_EN_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn wifi_clk_wifi_bt_common(&mut self) -> WIFI_CLK_WIFI_BT_COMMON_W {
        WIFI_CLK_WIFI_BT_COMMON_W { w: self }
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn wifi_clk_bt_en(&mut self) -> WIFI_CLK_BT_EN_W {
        WIFI_CLK_BT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_clk_en](index.html) module"]
pub struct WIFI_CLK_EN_SPEC;
impl crate::RegisterSpec for WIFI_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_clk_en::R](R) reader structure"]
impl crate::Readable for WIFI_CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_clk_en::W](W) writer structure"]
impl crate::Writable for WIFI_CLK_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIFI_CLK_EN to value 0xfffc_e030"]
impl crate::Resettable for WIFI_CLK_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfffc_e030
    }
}
