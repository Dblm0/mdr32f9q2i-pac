#[doc = "Register `HTXT` reader"]
pub struct R(crate::R<HTXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTXT` writer"]
pub struct W(crate::W<HTXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTXT_SPEC>;
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
impl From<crate::W<HTXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTYPE` reader - "]
pub type TTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTYPE` writer - "]
pub type TTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HTXT_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ttype(&self) -> TTYPE_R {
        TTYPE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn ttype(&mut self) -> TTYPE_W<0> {
        TTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB HTXT Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htxt](index.html) module"]
pub struct HTXT_SPEC;
impl crate::RegisterSpec for HTXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htxt::R](R) reader structure"]
impl crate::Readable for HTXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htxt::W](W) writer structure"]
impl crate::Writable for HTXT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTXT to value 0"]
impl crate::Resettable for HTXT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
