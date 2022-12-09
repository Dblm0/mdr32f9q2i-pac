#[doc = "Register `SEP0_CTRL` reader"]
pub struct R(crate::R<SEP0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEP0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEP0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEP0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEP0_CTRL` writer"]
pub struct W(crate::W<SEP0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEP0_CTRL_SPEC>;
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
impl From<crate::W<SEP0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEP0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPEN` reader - "]
pub type EPEN_R = crate::BitReader<bool>;
#[doc = "Field `EPEN` writer - "]
pub type EPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEP0_CTRL_SPEC, bool, O>;
#[doc = "Field `EPRDY` reader - "]
pub type EPRDY_R = crate::BitReader<bool>;
#[doc = "Field `EPRDY` writer - "]
pub type EPRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEP0_CTRL_SPEC, bool, O>;
#[doc = "Field `EPDATASEQ` reader - "]
pub type EPDATASEQ_R = crate::BitReader<bool>;
#[doc = "Field `EPDATASEQ` writer - "]
pub type EPDATASEQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEP0_CTRL_SPEC, bool, O>;
#[doc = "Field `EPSSTALL` reader - "]
pub type EPSSTALL_R = crate::BitReader<bool>;
#[doc = "Field `EPSSTALL` writer - "]
pub type EPSSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEP0_CTRL_SPEC, bool, O>;
#[doc = "Field `EPISOEN` reader - "]
pub type EPISOEN_R = crate::BitReader<bool>;
#[doc = "Field `EPISOEN` writer - "]
pub type EPISOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEP0_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn epen(&self) -> EPEN_R {
        EPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn eprdy(&self) -> EPRDY_R {
        EPRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn epdataseq(&self) -> EPDATASEQ_R {
        EPDATASEQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn epsstall(&self) -> EPSSTALL_R {
        EPSSTALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn episoen(&self) -> EPISOEN_R {
        EPISOEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn epen(&mut self) -> EPEN_W<0> {
        EPEN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn eprdy(&mut self) -> EPRDY_W<1> {
        EPRDY_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn epdataseq(&mut self) -> EPDATASEQ_W<2> {
        EPDATASEQ_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn epsstall(&mut self) -> EPSSTALL_W<3> {
        EPSSTALL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn episoen(&mut self) -> EPISOEN_W<4> {
        EPISOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_SEP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sep0_ctrl](index.html) module"]
pub struct SEP0_CTRL_SPEC;
impl crate::RegisterSpec for SEP0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sep0_ctrl::R](R) reader structure"]
impl crate::Readable for SEP0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sep0_ctrl::W](W) writer structure"]
impl crate::Writable for SEP0_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEP0_CTRL to value 0"]
impl crate::Resettable for SEP0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
