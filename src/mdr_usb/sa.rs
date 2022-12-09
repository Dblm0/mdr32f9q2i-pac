#[doc = "Register `SA` reader"]
pub struct R(crate::R<SA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SA` writer"]
pub struct W(crate::W<SA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SA_SPEC>;
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
impl From<crate::W<SA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCDEVADDR` reader - "]
pub type SCDEVADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCDEVADDR` writer - "]
pub type SCDEVADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SA_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn scdevaddr(&self) -> SCDEVADDR_R {
        SCDEVADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn scdevaddr(&mut self) -> SCDEVADDR_W<0> {
        SCDEVADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa](index.html) module"]
pub struct SA_SPEC;
impl crate::RegisterSpec for SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sa::R](R) reader structure"]
impl crate::Readable for SA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sa::W](W) writer structure"]
impl crate::Writable for SA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SA to value 0"]
impl crate::Resettable for SA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
