#[doc = "Register `CCR11` reader"]
pub struct R(crate::R<CCR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR11` writer"]
pub struct W(crate::W<CCR11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR11_SPEC>;
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
impl From<crate::W<CCR11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR` reader - "]
pub type CCR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR` writer - "]
pub type CCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR11_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn ccr(&mut self) -> CCR_W<0> {
        CCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Capture/Compare1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr11](index.html) module"]
pub struct CCR11_SPEC;
impl crate::RegisterSpec for CCR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr11::R](R) reader structure"]
impl crate::Readable for CCR11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr11::W](W) writer structure"]
impl crate::Writable for CCR11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR11 to value 0"]
impl crate::Resettable for CCR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
