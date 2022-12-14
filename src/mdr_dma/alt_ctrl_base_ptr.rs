#[doc = "Register `ALT_CTRL_BASE_PTR` reader"]
pub struct R(crate::R<ALT_CTRL_BASE_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALT_CTRL_BASE_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALT_CTRL_BASE_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALT_CTRL_BASE_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALT_CTRL_BASE_PTR` writer"]
pub struct W(crate::W<ALT_CTRL_BASE_PTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALT_CTRL_BASE_PTR_SPEC>;
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
impl From<crate::W<ALT_CTRL_BASE_PTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALT_CTRL_BASE_PTR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alt_ctrl_base_ptr](index.html) module"]
pub struct ALT_CTRL_BASE_PTR_SPEC;
impl crate::RegisterSpec for ALT_CTRL_BASE_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alt_ctrl_base_ptr::R](R) reader structure"]
impl crate::Readable for ALT_CTRL_BASE_PTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alt_ctrl_base_ptr::W](W) writer structure"]
impl crate::Writable for ALT_CTRL_BASE_PTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALT_CTRL_BASE_PTR to value 0"]
impl crate::Resettable for ALT_CTRL_BASE_PTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
