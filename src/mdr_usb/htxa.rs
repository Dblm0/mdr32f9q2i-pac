#[doc = "Register `HTXA` reader"]
pub struct R(crate::R<HTXA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTXA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTXA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTXA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTXA` writer"]
pub struct W(crate::W<HTXA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTXA_SPEC>;
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
impl From<crate::W<HTXA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTXA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEVADDR` reader - "]
pub type DEVADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEVADDR` writer - "]
pub type DEVADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HTXA_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DEVADDR_W<0> {
        DEVADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htxa](index.html) module"]
pub struct HTXA_SPEC;
impl crate::RegisterSpec for HTXA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htxa::R](R) reader structure"]
impl crate::Readable for HTXA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htxa::W](W) writer structure"]
impl crate::Writable for HTXA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTXA to value 0"]
impl crate::Resettable for HTXA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
