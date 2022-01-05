#[doc = "Register `PIN_CTRL` reader"]
pub struct R(crate::R<PIN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN_CTRL` writer"]
pub struct W(crate::W<PIN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_CTRL_SPEC>;
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
impl From<crate::W<PIN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK1` reader - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[11:8\\]
= 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[11:8\\]
= 0x0."]
pub struct CLK1_R(crate::FieldReader<u8, u8>);
impl CLK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK1` writer - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[11:8\\]
= 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[11:8\\]
= 0x0."]
pub struct CLK1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CLK2` reader - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[11:8\\]
= 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[11:8\\]
= 0x0."]
pub struct CLK2_R(crate::FieldReader<u8, u8>);
impl CLK2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK2` writer - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[11:8\\]
= 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[11:8\\]
= 0x0."]
pub struct CLK2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CLK3` reader - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[11:8\\]
= 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[11:8\\]
= 0x0."]
pub struct CLK3_R(crate::FieldReader<u8, u8>);
impl CLK3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK3` writer - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[11:8\\]
= 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[11:8\\]
= 0x0."]
pub struct CLK3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[11:8\\]
= 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[11:8\\]
= 0x0."]
    #[inline(always)]
    pub fn clk1(&self) -> CLK1_R {
        CLK1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[11:8\\]
= 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[11:8\\]
= 0x0."]
    #[inline(always)]
    pub fn clk2(&self) -> CLK2_R {
        CLK2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[11:8\\]
= 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[11:8\\]
= 0x0."]
    #[inline(always)]
    pub fn clk3(&self) -> CLK3_R {
        CLK3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[11:8\\]
= 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[11:8\\]
= 0x0."]
    #[inline(always)]
    pub fn clk1(&mut self) -> CLK1_W {
        CLK1_W { w: self }
    }
    #[doc = "Bits 4:7 - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[11:8\\]
= 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[11:8\\]
= 0x0."]
    #[inline(always)]
    pub fn clk2(&mut self) -> CLK2_W {
        CLK2_W { w: self }
    }
    #[doc = "Bits 8:11 - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0x0 and PIN_CTRL\\[11:8\\]
= 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\]
= 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[7:4\\]
= 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\]
= 0xF and PIN_CTRL\\[11:8\\]
= 0x0."]
    #[inline(always)]
    pub fn clk3(&mut self) -> CLK3_W {
        CLK3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin_ctrl](index.html) module"]
pub struct PIN_CTRL_SPEC;
impl crate::RegisterSpec for PIN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin_ctrl::R](R) reader structure"]
impl crate::Readable for PIN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin_ctrl::W](W) writer structure"]
impl crate::Writable for PIN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIN_CTRL to value 0"]
impl crate::Resettable for PIN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}