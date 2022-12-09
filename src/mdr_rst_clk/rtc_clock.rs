#[doc = "Register `RTC_CLOCK` reader"]
pub struct R(crate::R<RTC_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CLOCK` writer"]
pub struct W(crate::W<RTC_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CLOCK_SPEC>;
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
impl From<crate::W<RTC_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSE_SEL` reader - "]
pub type HSE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSE_SEL` writer - "]
pub type HSE_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_CLOCK_SPEC, u8, u8, 4, O>;
#[doc = "Field `HSI_SEL` reader - "]
pub type HSI_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSI_SEL` writer - "]
pub type HSI_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_CLOCK_SPEC, u8, u8, 4, O>;
#[doc = "Field `HSE_RTC_EN` reader - "]
pub type HSE_RTC_EN_R = crate::BitReader<bool>;
#[doc = "Field `HSE_RTC_EN` writer - "]
pub type HSE_RTC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_CLOCK_SPEC, bool, O>;
#[doc = "Field `HSI_RTC_EN` reader - "]
pub type HSI_RTC_EN_R = crate::BitReader<bool>;
#[doc = "Field `HSI_RTC_EN` writer - "]
pub type HSI_RTC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_CLOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hse_sel(&self) -> HSE_SEL_R {
        HSE_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn hsi_sel(&self) -> HSI_SEL_R {
        HSI_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hse_rtc_en(&self) -> HSE_RTC_EN_R {
        HSE_RTC_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn hsi_rtc_en(&self) -> HSI_RTC_EN_R {
        HSI_RTC_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn hse_sel(&mut self) -> HSE_SEL_W<0> {
        HSE_SEL_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn hsi_sel(&mut self) -> HSI_SEL_W<4> {
        HSI_SEL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn hse_rtc_en(&mut self) -> HSE_RTC_EN_W<8> {
        HSE_RTC_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn hsi_rtc_en(&mut self) -> HSI_RTC_EN_W<9> {
        HSI_RTC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_clock](index.html) module"]
pub struct RTC_CLOCK_SPEC;
impl crate::RegisterSpec for RTC_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_clock::R](R) reader structure"]
impl crate::Readable for RTC_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_clock::W](W) writer structure"]
impl crate::Writable for RTC_CLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_CLOCK to value 0"]
impl crate::Resettable for RTC_CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
