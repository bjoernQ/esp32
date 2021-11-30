#[doc = "Register `FH2_CFG0` reader"]
pub struct R(crate::R<FH2_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FH2_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FH2_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FH2_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FH2_CFG0` writer"]
pub struct W(crate::W<FH2_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FH2_CFG0_SPEC>;
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
impl From<crate::W<FH2_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FH2_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FH2_SW_CBC` reader - "]
pub struct FH2_SW_CBC_R(crate::FieldReader<bool, bool>);
impl FH2_SW_CBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH2_SW_CBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_SW_CBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_SW_CBC` writer - "]
pub struct FH2_SW_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_SW_CBC_W<'a> {
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
#[doc = "Field `FH2_F2_CBC` reader - "]
pub struct FH2_F2_CBC_R(crate::FieldReader<bool, bool>);
impl FH2_F2_CBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH2_F2_CBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_F2_CBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_F2_CBC` writer - "]
pub struct FH2_F2_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_F2_CBC_W<'a> {
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
#[doc = "Field `FH2_F1_CBC` reader - "]
pub struct FH2_F1_CBC_R(crate::FieldReader<bool, bool>);
impl FH2_F1_CBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH2_F1_CBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_F1_CBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_F1_CBC` writer - "]
pub struct FH2_F1_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_F1_CBC_W<'a> {
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
#[doc = "Field `FH2_F0_CBC` reader - "]
pub struct FH2_F0_CBC_R(crate::FieldReader<bool, bool>);
impl FH2_F0_CBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH2_F0_CBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_F0_CBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_F0_CBC` writer - "]
pub struct FH2_F0_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_F0_CBC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `FH2_SW_OST` reader - "]
pub struct FH2_SW_OST_R(crate::FieldReader<bool, bool>);
impl FH2_SW_OST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH2_SW_OST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_SW_OST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_SW_OST` writer - "]
pub struct FH2_SW_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_SW_OST_W<'a> {
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
#[doc = "Field `FH2_F2_OST` reader - "]
pub struct FH2_F2_OST_R(crate::FieldReader<bool, bool>);
impl FH2_F2_OST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH2_F2_OST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_F2_OST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_F2_OST` writer - "]
pub struct FH2_F2_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_F2_OST_W<'a> {
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
#[doc = "Field `FH2_F1_OST` reader - "]
pub struct FH2_F1_OST_R(crate::FieldReader<bool, bool>);
impl FH2_F1_OST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH2_F1_OST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_F1_OST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_F1_OST` writer - "]
pub struct FH2_F1_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_F1_OST_W<'a> {
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
#[doc = "Field `FH2_F0_OST` reader - "]
pub struct FH2_F0_OST_R(crate::FieldReader<bool, bool>);
impl FH2_F0_OST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH2_F0_OST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_F0_OST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_F0_OST` writer - "]
pub struct FH2_F0_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_F0_OST_W<'a> {
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
#[doc = "Field `FH2_A_CBC_D` reader - "]
pub struct FH2_A_CBC_D_R(crate::FieldReader<u8, u8>);
impl FH2_A_CBC_D_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FH2_A_CBC_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_A_CBC_D_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_A_CBC_D` writer - "]
pub struct FH2_A_CBC_D_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_A_CBC_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `FH2_A_CBC_U` reader - "]
pub struct FH2_A_CBC_U_R(crate::FieldReader<u8, u8>);
impl FH2_A_CBC_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FH2_A_CBC_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_A_CBC_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_A_CBC_U` writer - "]
pub struct FH2_A_CBC_U_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_A_CBC_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `FH2_A_OST_D` reader - "]
pub struct FH2_A_OST_D_R(crate::FieldReader<u8, u8>);
impl FH2_A_OST_D_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FH2_A_OST_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_A_OST_D_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_A_OST_D` writer - "]
pub struct FH2_A_OST_D_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_A_OST_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `FH2_A_OST_U` reader - "]
pub struct FH2_A_OST_U_R(crate::FieldReader<u8, u8>);
impl FH2_A_OST_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FH2_A_OST_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_A_OST_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_A_OST_U` writer - "]
pub struct FH2_A_OST_U_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_A_OST_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `FH2_B_CBC_D` reader - "]
pub struct FH2_B_CBC_D_R(crate::FieldReader<u8, u8>);
impl FH2_B_CBC_D_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FH2_B_CBC_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_B_CBC_D_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_B_CBC_D` writer - "]
pub struct FH2_B_CBC_D_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_B_CBC_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `FH2_B_CBC_U` reader - "]
pub struct FH2_B_CBC_U_R(crate::FieldReader<u8, u8>);
impl FH2_B_CBC_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FH2_B_CBC_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_B_CBC_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_B_CBC_U` writer - "]
pub struct FH2_B_CBC_U_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_B_CBC_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `FH2_B_OST_D` reader - "]
pub struct FH2_B_OST_D_R(crate::FieldReader<u8, u8>);
impl FH2_B_OST_D_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FH2_B_OST_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_B_OST_D_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_B_OST_D` writer - "]
pub struct FH2_B_OST_D_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_B_OST_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `FH2_B_OST_U` reader - "]
pub struct FH2_B_OST_U_R(crate::FieldReader<u8, u8>);
impl FH2_B_OST_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FH2_B_OST_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_B_OST_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_B_OST_U` writer - "]
pub struct FH2_B_OST_U_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_B_OST_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh2_sw_cbc(&self) -> FH2_SW_CBC_R {
        FH2_SW_CBC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fh2_f2_cbc(&self) -> FH2_F2_CBC_R {
        FH2_F2_CBC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fh2_f1_cbc(&self) -> FH2_F1_CBC_R {
        FH2_F1_CBC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fh2_f0_cbc(&self) -> FH2_F0_CBC_R {
        FH2_F0_CBC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fh2_sw_ost(&self) -> FH2_SW_OST_R {
        FH2_SW_OST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fh2_f2_ost(&self) -> FH2_F2_OST_R {
        FH2_F2_OST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn fh2_f1_ost(&self) -> FH2_F1_OST_R {
        FH2_F1_OST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fh2_f0_ost(&self) -> FH2_F0_OST_R {
        FH2_F0_OST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn fh2_a_cbc_d(&self) -> FH2_A_CBC_D_R {
        FH2_A_CBC_D_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn fh2_a_cbc_u(&self) -> FH2_A_CBC_U_R {
        FH2_A_CBC_U_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn fh2_a_ost_d(&self) -> FH2_A_OST_D_R {
        FH2_A_OST_D_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn fh2_a_ost_u(&self) -> FH2_A_OST_U_R {
        FH2_A_OST_U_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn fh2_b_cbc_d(&self) -> FH2_B_CBC_D_R {
        FH2_B_CBC_D_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn fh2_b_cbc_u(&self) -> FH2_B_CBC_U_R {
        FH2_B_CBC_U_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn fh2_b_ost_d(&self) -> FH2_B_OST_D_R {
        FH2_B_OST_D_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn fh2_b_ost_u(&self) -> FH2_B_OST_U_R {
        FH2_B_OST_U_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh2_sw_cbc(&mut self) -> FH2_SW_CBC_W {
        FH2_SW_CBC_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fh2_f2_cbc(&mut self) -> FH2_F2_CBC_W {
        FH2_F2_CBC_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fh2_f1_cbc(&mut self) -> FH2_F1_CBC_W {
        FH2_F1_CBC_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fh2_f0_cbc(&mut self) -> FH2_F0_CBC_W {
        FH2_F0_CBC_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fh2_sw_ost(&mut self) -> FH2_SW_OST_W {
        FH2_SW_OST_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fh2_f2_ost(&mut self) -> FH2_F2_OST_W {
        FH2_F2_OST_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn fh2_f1_ost(&mut self) -> FH2_F1_OST_W {
        FH2_F1_OST_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fh2_f0_ost(&mut self) -> FH2_F0_OST_W {
        FH2_F0_OST_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn fh2_a_cbc_d(&mut self) -> FH2_A_CBC_D_W {
        FH2_A_CBC_D_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn fh2_a_cbc_u(&mut self) -> FH2_A_CBC_U_W {
        FH2_A_CBC_U_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn fh2_a_ost_d(&mut self) -> FH2_A_OST_D_W {
        FH2_A_OST_D_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn fh2_a_ost_u(&mut self) -> FH2_A_OST_U_W {
        FH2_A_OST_U_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn fh2_b_cbc_d(&mut self) -> FH2_B_CBC_D_W {
        FH2_B_CBC_D_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn fh2_b_cbc_u(&mut self) -> FH2_B_CBC_U_W {
        FH2_B_CBC_U_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn fh2_b_ost_d(&mut self) -> FH2_B_OST_D_W {
        FH2_B_OST_D_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn fh2_b_ost_u(&mut self) -> FH2_B_OST_U_W {
        FH2_B_OST_U_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fh2_cfg0](index.html) module"]
pub struct FH2_CFG0_SPEC;
impl crate::RegisterSpec for FH2_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fh2_cfg0::R](R) reader structure"]
impl crate::Readable for FH2_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fh2_cfg0::W](W) writer structure"]
impl crate::Writable for FH2_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FH2_CFG0 to value 0"]
impl crate::Resettable for FH2_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
