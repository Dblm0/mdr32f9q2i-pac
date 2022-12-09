#[doc = "Register `PVDCS` reader"]
pub struct R(crate::R<PVDCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PVDCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PVDCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PVDCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PVDCS` writer"]
pub struct W(crate::W<PVDCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PVDCS_SPEC>;
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
impl From<crate::W<PVDCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PVDCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVDEN` reader - "]
pub type PVDEN_R = crate::BitReader<bool>;
#[doc = "Field `PVDEN` writer - "]
pub type PVDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PVDCS_SPEC, bool, O>;
#[doc = "Field `PBLS` reader - "]
pub type PBLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PBLS` writer - "]
pub type PBLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PVDCS_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLS` reader - "]
pub type PLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLS` writer - "]
pub type PLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PVDCS_SPEC, u8, u8, 3, O>;
#[doc = "Field `PVBD` reader - "]
pub type PVBD_R = crate::BitReader<bool>;
#[doc = "Field `PVBD` writer - "]
pub type PVBD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PVDCS_SPEC, bool, O>;
#[doc = "Field `PVD` reader - "]
pub type PVD_R = crate::BitReader<bool>;
#[doc = "Field `PVD` writer - "]
pub type PVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PVDCS_SPEC, bool, O>;
#[doc = "Field `IEPVBD` reader - "]
pub type IEPVBD_R = crate::BitReader<bool>;
#[doc = "Field `IEPVBD` writer - "]
pub type IEPVBD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PVDCS_SPEC, bool, O>;
#[doc = "Field `IEPVD` reader - "]
pub type IEPVD_R = crate::BitReader<bool>;
#[doc = "Field `IEPVD` writer - "]
pub type IEPVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PVDCS_SPEC, bool, O>;
#[doc = "Field `INVB` reader - "]
pub type INVB_R = crate::BitReader<bool>;
#[doc = "Field `INVB` writer - "]
pub type INVB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PVDCS_SPEC, bool, O>;
#[doc = "Field `INV` reader - "]
pub type INV_R = crate::BitReader<bool>;
#[doc = "Field `INV` writer - "]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PVDCS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pvden(&self) -> PVDEN_R {
        PVDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn pbls(&self) -> PBLS_R {
        PBLS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pvbd(&self) -> PVBD_R {
        PVBD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pvd(&self) -> PVD_R {
        PVD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn iepvbd(&self) -> IEPVBD_R {
        IEPVBD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn iepvd(&self) -> IEPVD_R {
        IEPVD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn invb(&self) -> INVB_R {
        INVB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pvden(&mut self) -> PVDEN_W<0> {
        PVDEN_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn pbls(&mut self) -> PBLS_W<1> {
        PBLS_W::new(self)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<3> {
        PLS_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pvbd(&mut self) -> PVBD_W<6> {
        PVBD_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pvd(&mut self) -> PVD_W<7> {
        PVD_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iepvbd(&mut self) -> IEPVBD_W<8> {
        IEPVBD_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iepvd(&mut self) -> IEPVD_W<9> {
        IEPVD_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn invb(&mut self) -> INVB_W<10> {
        INVB_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<11> {
        INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "POWER Power Detector Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pvdcs](index.html) module"]
pub struct PVDCS_SPEC;
impl crate::RegisterSpec for PVDCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pvdcs::R](R) reader structure"]
impl crate::Readable for PVDCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pvdcs::W](W) writer structure"]
impl crate::Writable for PVDCS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PVDCS to value 0"]
impl crate::Resettable for PVDCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
