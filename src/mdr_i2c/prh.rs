#[doc = "Register `PRH` reader"]
pub struct R(crate::R<PRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRH` writer"]
pub struct W(crate::W<PRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRH_SPEC>;
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
impl From<crate::W<PRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR` reader - "]
pub type PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PR` writer - "]
pub type PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<0> {
        PR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Prescaler (high byte) Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prh](index.html) module"]
pub struct PRH_SPEC;
impl crate::RegisterSpec for PRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prh::R](R) reader structure"]
impl crate::Readable for PRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prh::W](W) writer structure"]
impl crate::Writable for PRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRH to value 0"]
impl crate::Resettable for PRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
