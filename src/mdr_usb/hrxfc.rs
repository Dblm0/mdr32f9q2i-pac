#[doc = "Register `HRXFC` reader"]
pub struct R(crate::R<HRXFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRXFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRXFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRXFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRXFC` writer"]
pub struct W(crate::W<HRXFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRXFC_SPEC>;
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
impl From<crate::W<HRXFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRXFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOFORCEEMPTY` reader - "]
pub type FIFOFORCEEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFOFORCEEMPTY` writer - "]
pub type FIFOFORCEEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRXFC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifoforceempty(&self) -> FIFOFORCEEMPTY_R {
        FIFOFORCEEMPTY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fifoforceempty(&mut self) -> FIFOFORCEEMPTY_W<0> {
        FIFOFORCEEMPTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrxfc](index.html) module"]
pub struct HRXFC_SPEC;
impl crate::RegisterSpec for HRXFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrxfc::R](R) reader structure"]
impl crate::Readable for HRXFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrxfc::W](W) writer structure"]
impl crate::Writable for HRXFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HRXFC to value 0"]
impl crate::Resettable for HRXFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
