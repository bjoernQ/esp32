#[doc = "Register `TIMER3` reader"]
pub struct R(crate::R<TIMER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER3` writer"]
pub struct W(crate::W<TIMER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER3_SPEC>;
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
impl From<crate::W<TIMER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIFI_WAIT_TIMER` reader - "]
pub struct WIFI_WAIT_TIMER_R(crate::FieldReader<u16, u16>);
impl WIFI_WAIT_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WIFI_WAIT_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_WAIT_TIMER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_WAIT_TIMER` writer - "]
pub struct WIFI_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `WIFI_POWERUP_TIMER` reader - "]
pub struct WIFI_POWERUP_TIMER_R(crate::FieldReader<u8, u8>);
impl WIFI_POWERUP_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WIFI_POWERUP_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_POWERUP_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_POWERUP_TIMER` writer - "]
pub struct WIFI_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u32 & 0x7f) << 9);
        self.w
    }
}
#[doc = "Field `ROM_RAM_WAIT_TIMER` reader - "]
pub struct ROM_RAM_WAIT_TIMER_R(crate::FieldReader<u16, u16>);
impl ROM_RAM_WAIT_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ROM_RAM_WAIT_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_RAM_WAIT_TIMER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_RAM_WAIT_TIMER` writer - "]
pub struct ROM_RAM_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_RAM_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Field `ROM_RAM_POWERUP_TIMER` reader - "]
pub struct ROM_RAM_POWERUP_TIMER_R(crate::FieldReader<u8, u8>);
impl ROM_RAM_POWERUP_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_RAM_POWERUP_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_RAM_POWERUP_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_RAM_POWERUP_TIMER` writer - "]
pub struct ROM_RAM_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_RAM_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn wifi_wait_timer(&self) -> WIFI_WAIT_TIMER_R {
        WIFI_WAIT_TIMER_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn wifi_powerup_timer(&self) -> WIFI_POWERUP_TIMER_R {
        WIFI_POWERUP_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rom_ram_wait_timer(&self) -> ROM_RAM_WAIT_TIMER_R {
        ROM_RAM_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rom_ram_powerup_timer(&self) -> ROM_RAM_POWERUP_TIMER_R {
        ROM_RAM_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn wifi_wait_timer(&mut self) -> WIFI_WAIT_TIMER_W {
        WIFI_WAIT_TIMER_W { w: self }
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn wifi_powerup_timer(&mut self) -> WIFI_POWERUP_TIMER_W {
        WIFI_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rom_ram_wait_timer(&mut self) -> ROM_RAM_WAIT_TIMER_W {
        ROM_RAM_WAIT_TIMER_W { w: self }
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rom_ram_powerup_timer(&mut self) -> ROM_RAM_POWERUP_TIMER_W {
        ROM_RAM_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer3](index.html) module"]
pub struct TIMER3_SPEC;
impl crate::RegisterSpec for TIMER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer3::R](R) reader structure"]
impl crate::Readable for TIMER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer3::W](W) writer structure"]
impl crate::Writable for TIMER3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER3 to value 0x1416_0a08"]
impl crate::Resettable for TIMER3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1416_0a08
    }
}
