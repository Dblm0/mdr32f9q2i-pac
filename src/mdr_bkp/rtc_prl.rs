#[doc = "Register `RTC_PRL` reader"]
pub struct R(crate::R<RTC_PRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_PRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_PRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_PRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_PRL` writer"]
pub struct W(crate::W<RTC_PRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_PRL_SPEC>;
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
impl From<crate::W<RTC_PRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_PRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_PRL` reader - "]
pub type RTC_PRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RTC_PRL` writer - "]
pub type RTC_PRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_PRL_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rtc_prl(&self) -> RTC_PRL_R {
        RTC_PRL_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_prl(&mut self) -> RTC_PRL_W<0> {
        RTC_PRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Realtime Prescaler Auto-Reload Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_prl](index.html) module"]
pub struct RTC_PRL_SPEC;
impl crate::RegisterSpec for RTC_PRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_prl::R](R) reader structure"]
impl crate::Readable for RTC_PRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_prl::W](W) writer structure"]
impl crate::Writable for RTC_PRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_PRL to value 0"]
impl crate::Resettable for RTC_PRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
