#[doc = "Register `HTXSE` reader"]
pub struct R(crate::R<HTXSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTXSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTXSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTXSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTXSE` writer"]
pub struct W(crate::W<HTXSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTXSE_SPEC>;
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
impl From<crate::W<HTXSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTXSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFEN` reader - "]
pub type SOFEN_R = crate::BitReader<bool>;
#[doc = "Field `SOFEN` writer - "]
pub type SOFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HTXSE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sofen(&self) -> SOFEN_R {
        SOFEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sofen(&mut self) -> SOFEN_W<0> {
        SOFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htxse](index.html) module"]
pub struct HTXSE_SPEC;
impl crate::RegisterSpec for HTXSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htxse::R](R) reader structure"]
impl crate::Readable for HTXSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htxse::W](W) writer structure"]
impl crate::Writable for HTXSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTXSE to value 0"]
impl crate::Resettable for HTXSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
