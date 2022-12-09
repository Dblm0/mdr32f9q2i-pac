#[doc = "Register `PSG` reader"]
pub struct R(crate::R<PSG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSG` writer"]
pub struct W(crate::W<PSG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSG_SPEC>;
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
impl From<crate::W<PSG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSG` reader - "]
pub type PSG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PSG` writer - "]
pub type PSG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn psg(&self) -> PSG_R {
        PSG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn psg(&mut self) -> PSG_W<0> {
        PSG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psg](index.html) module"]
pub struct PSG_SPEC;
impl crate::RegisterSpec for PSG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psg::R](R) reader structure"]
impl crate::Readable for PSG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psg::W](W) writer structure"]
impl crate::Writable for PSG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSG to value 0"]
impl crate::Resettable for PSG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
