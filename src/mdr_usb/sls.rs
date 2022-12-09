#[doc = "Register `SLS` reader"]
pub struct R(crate::R<SLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLS` writer"]
pub struct W(crate::W<SLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLS_SPEC>;
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
impl From<crate::W<SLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRXLS` reader - "]
pub type SCRXLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCRXLS` writer - "]
pub type SCRXLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLS_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn scrxls(&self) -> SCRXLS_R {
        SCRXLS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn scrxls(&mut self) -> SCRXLS_W<0> {
        SCRXLS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sls](index.html) module"]
pub struct SLS_SPEC;
impl crate::RegisterSpec for SLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sls::R](R) reader structure"]
impl crate::Readable for SLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sls::W](W) writer structure"]
impl crate::Writable for SLS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLS to value 0"]
impl crate::Resettable for SLS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}