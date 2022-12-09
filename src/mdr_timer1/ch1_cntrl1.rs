#[doc = "Register `CH1_CNTRL1` reader"]
pub struct R(crate::R<CH1_CNTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1_CNTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1_CNTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1_CNTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1_CNTRL1` writer"]
pub struct W(crate::W<CH1_CNTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1_CNTRL1_SPEC>;
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
impl From<crate::W<CH1_CNTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1_CNTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELOE` reader - "]
pub type SELOE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELOE` writer - "]
pub type SELOE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1_CNTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SELO` reader - "]
pub type SELO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELO` writer - "]
pub type SELO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1_CNTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `INV` reader - "]
pub type INV_R = crate::BitReader<bool>;
#[doc = "Field `INV` writer - "]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1_CNTRL1_SPEC, bool, O>;
#[doc = "Field `NSELOE` reader - "]
pub type NSELOE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NSELOE` writer - "]
pub type NSELOE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1_CNTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `NSELO` reader - "]
pub type NSELO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NSELO` writer - "]
pub type NSELO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1_CNTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `NINV` reader - "]
pub type NINV_R = crate::BitReader<bool>;
#[doc = "Field `NINV` writer - "]
pub type NINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1_CNTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn seloe(&self) -> SELOE_R {
        SELOE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn selo(&self) -> SELO_R {
        SELO_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn nseloe(&self) -> NSELOE_R {
        NSELOE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn nselo(&self) -> NSELO_R {
        NSELO_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ninv(&self) -> NINV_R {
        NINV_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn seloe(&mut self) -> SELOE_W<0> {
        SELOE_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn selo(&mut self) -> SELO_W<2> {
        SELO_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<4> {
        INV_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn nseloe(&mut self) -> NSELOE_W<8> {
        NSELOE_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn nselo(&mut self) -> NSELO_W<10> {
        NSELO_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ninv(&mut self) -> NINV_W<12> {
        NINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Control1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_cntrl1](index.html) module"]
pub struct CH1_CNTRL1_SPEC;
impl crate::RegisterSpec for CH1_CNTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1_cntrl1::R](R) reader structure"]
impl crate::Readable for CH1_CNTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1_cntrl1::W](W) writer structure"]
impl crate::Writable for CH1_CNTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1_CNTRL1 to value 0"]
impl crate::Resettable for CH1_CNTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
