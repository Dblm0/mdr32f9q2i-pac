#[doc = "Register `HTXE` reader"]
pub struct R(crate::R<HTXE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTXE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTXE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTXE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTXE` writer"]
pub struct W(crate::W<HTXE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTXE_SPEC>;
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
impl From<crate::W<HTXE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTXE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPADDR` reader - "]
pub type EPADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPADDR` writer - "]
pub type EPADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HTXE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn epaddr(&self) -> EPADDR_R {
        EPADDR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn epaddr(&mut self) -> EPADDR_W<0> {
        EPADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htxe](index.html) module"]
pub struct HTXE_SPEC;
impl crate::RegisterSpec for HTXE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htxe::R](R) reader structure"]
impl crate::Readable for HTXE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htxe::W](W) writer structure"]
impl crate::Writable for HTXE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTXE to value 0"]
impl crate::Resettable for HTXE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
