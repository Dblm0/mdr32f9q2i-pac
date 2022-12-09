#[doc = "Register `REG_0F` reader"]
pub struct R(crate::R<REG_0F_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_0F_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_0F_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_0F_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_0F` writer"]
pub struct W(crate::W<REG_0F_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_0F_SPEC>;
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
impl From<crate::W<REG_0F_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_0F_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSE_ON` reader - "]
pub type LSE_ON_R = crate::BitReader<bool>;
#[doc = "Field `LSE_ON` writer - "]
pub type LSE_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0F_SPEC, bool, O>;
#[doc = "Field `LSE_BYP` reader - "]
pub type LSE_BYP_R = crate::BitReader<bool>;
#[doc = "Field `LSE_BYP` writer - "]
pub type LSE_BYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0F_SPEC, bool, O>;
#[doc = "Field `RTC_SEL` reader - "]
pub type RTC_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_SEL` writer - "]
pub type RTC_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_0F_SPEC, u8, u8, 2, O>;
#[doc = "Field `RTC_EN` reader - "]
pub type RTC_EN_R = crate::BitReader<bool>;
#[doc = "Field `RTC_EN` writer - "]
pub type RTC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0F_SPEC, bool, O>;
#[doc = "Field `CAL` reader - "]
pub type CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAL` writer - "]
pub type CAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_0F_SPEC, u8, u8, 8, O>;
#[doc = "Field `LSE_RDY` reader - "]
pub type LSE_RDY_R = crate::BitReader<bool>;
#[doc = "Field `LSE_RDY` writer - "]
pub type LSE_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0F_SPEC, bool, O>;
#[doc = "Field `BKP_REG` reader - "]
pub type BKP_REG_R = crate::BitReader<bool>;
#[doc = "Field `BKP_REG` writer - "]
pub type BKP_REG_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0F_SPEC, bool, O>;
#[doc = "Field `LSI_ON` reader - "]
pub type LSI_ON_R = crate::BitReader<bool>;
#[doc = "Field `LSI_ON` writer - "]
pub type LSI_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0F_SPEC, bool, O>;
#[doc = "Field `LSI_TRIM` reader - "]
pub type LSI_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSI_TRIM` writer - "]
pub type LSI_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_0F_SPEC, u8, u8, 5, O>;
#[doc = "Field `LSI_RDY` reader - "]
pub type LSI_RDY_R = crate::BitReader<bool>;
#[doc = "Field `LSI_RDY` writer - "]
pub type LSI_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0F_SPEC, bool, O>;
#[doc = "Field `HSI_ON` reader - "]
pub type HSI_ON_R = crate::BitReader<bool>;
#[doc = "Field `HSI_ON` writer - "]
pub type HSI_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0F_SPEC, bool, O>;
#[doc = "Field `HSI_RDY` reader - "]
pub type HSI_RDY_R = crate::BitReader<bool>;
#[doc = "Field `HSI_RDY` writer - "]
pub type HSI_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0F_SPEC, bool, O>;
#[doc = "Field `HSI_TRIM` reader - "]
pub type HSI_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSI_TRIM` writer - "]
pub type HSI_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_0F_SPEC, u8, u8, 6, O>;
#[doc = "Field `STANDBY` reader - "]
pub type STANDBY_R = crate::BitReader<bool>;
#[doc = "Field `STANDBY` writer - "]
pub type STANDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0F_SPEC, bool, O>;
#[doc = "Field `RTC_RESET` reader - "]
pub type RTC_RESET_R = crate::BitReader<bool>;
#[doc = "Field `RTC_RESET` writer - "]
pub type RTC_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0F_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lse_on(&self) -> LSE_ON_R {
        LSE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lse_byp(&self) -> LSE_BYP_R {
        LSE_BYP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rtc_sel(&self) -> RTC_SEL_R {
        RTC_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_en(&self) -> RTC_EN_R {
        RTC_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn lse_rdy(&self) -> LSE_RDY_R {
        LSE_RDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn bkp_reg(&self) -> BKP_REG_R {
        BKP_REG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn lsi_on(&self) -> LSI_ON_R {
        LSI_ON_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lsi_trim(&self) -> LSI_TRIM_R {
        LSI_TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn lsi_rdy(&self) -> LSI_RDY_R {
        LSI_RDY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn hsi_on(&self) -> HSI_ON_R {
        HSI_ON_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn hsi_rdy(&self) -> HSI_RDY_R {
        HSI_RDY_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn hsi_trim(&self) -> HSI_TRIM_R {
        HSI_TRIM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn standby(&self) -> STANDBY_R {
        STANDBY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_reset(&self) -> RTC_RESET_R {
        RTC_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn lse_on(&mut self) -> LSE_ON_W<0> {
        LSE_ON_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn lse_byp(&mut self) -> LSE_BYP_W<1> {
        LSE_BYP_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_sel(&mut self) -> RTC_SEL_W<2> {
        RTC_SEL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_en(&mut self) -> RTC_EN_W<4> {
        RTC_EN_W::new(self)
    }
    #[doc = "Bits 5:12"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<5> {
        CAL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn lse_rdy(&mut self) -> LSE_RDY_W<13> {
        LSE_RDY_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn bkp_reg(&mut self) -> BKP_REG_W<14> {
        BKP_REG_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn lsi_on(&mut self) -> LSI_ON_W<15> {
        LSI_ON_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn lsi_trim(&mut self) -> LSI_TRIM_W<16> {
        LSI_TRIM_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn lsi_rdy(&mut self) -> LSI_RDY_W<21> {
        LSI_RDY_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn hsi_on(&mut self) -> HSI_ON_W<22> {
        HSI_ON_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn hsi_rdy(&mut self) -> HSI_RDY_W<23> {
        HSI_RDY_W::new(self)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    #[must_use]
    pub fn hsi_trim(&mut self) -> HSI_TRIM_W<24> {
        HSI_TRIM_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn standby(&mut self) -> STANDBY_W<30> {
        STANDBY_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_reset(&mut self) -> RTC_RESET_W<31> {
        RTC_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_0f](index.html) module"]
pub struct REG_0F_SPEC;
impl crate::RegisterSpec for REG_0F_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_0f::R](R) reader structure"]
impl crate::Readable for REG_0F_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_0f::W](W) writer structure"]
impl crate::Writable for REG_0F_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_0F to value 0"]
impl crate::Resettable for REG_0F_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
