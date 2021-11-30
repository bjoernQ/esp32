#[doc = "Register `DAC_CONF` reader"]
pub struct R(crate::R<DAC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_CONF` writer"]
pub struct W(crate::W<DAC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_CONF_SPEC>;
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
impl From<crate::W<DAC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_CLK_DIV` reader - efuse timing configure"]
pub struct DAC_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl DAC_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_CLK_DIV` writer - efuse timing configure"]
pub struct DAC_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DAC_CLK_PAD_SEL` reader - "]
pub struct DAC_CLK_PAD_SEL_R(crate::FieldReader<bool, bool>);
impl DAC_CLK_PAD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC_CLK_PAD_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CLK_PAD_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_CLK_PAD_SEL` writer - "]
pub struct DAC_CLK_PAD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_PAD_SEL_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - efuse timing configure"]
    #[inline(always)]
    pub fn dac_clk_div(&self) -> DAC_CLK_DIV_R {
        DAC_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dac_clk_pad_sel(&self) -> DAC_CLK_PAD_SEL_R {
        DAC_CLK_PAD_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - efuse timing configure"]
    #[inline(always)]
    pub fn dac_clk_div(&mut self) -> DAC_CLK_DIV_W {
        DAC_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dac_clk_pad_sel(&mut self) -> DAC_CLK_PAD_SEL_W {
        DAC_CLK_PAD_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_conf](index.html) module"]
pub struct DAC_CONF_SPEC;
impl crate::RegisterSpec for DAC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_conf::R](R) reader structure"]
impl crate::Readable for DAC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_conf::W](W) writer structure"]
impl crate::Writable for DAC_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_CONF to value 0x28"]
impl crate::Resettable for DAC_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x28
    }
}
