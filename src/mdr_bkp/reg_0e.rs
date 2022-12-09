#[doc = "Register `REG_0E` reader"]
pub struct R(crate::R<REG_0E_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_0E_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_0E_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_0E_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_0E` writer"]
pub struct W(crate::W<REG_0E_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_0E_SPEC>;
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
impl From<crate::W<REG_0E_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_0E_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOW` reader - "]
pub type LOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOW` writer - "]
pub type LOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_0E_SPEC, u8, u8, 3, O>;
#[doc = "Field `SelectRI` reader - "]
pub type SELECT_RI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SelectRI` writer - "]
pub type SELECT_RI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_0E_SPEC, u8, u8, 3, O>;
#[doc = "Field `JTAGA` reader - "]
pub type JTAGA_R = crate::BitReader<bool>;
#[doc = "Field `JTAGA` writer - "]
pub type JTAGA_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0E_SPEC, bool, O>;
#[doc = "Field `JTAGB` reader - "]
pub type JTAGB_R = crate::BitReader<bool>;
#[doc = "Field `JTAGB` writer - "]
pub type JTAGB_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0E_SPEC, bool, O>;
#[doc = "Field `Trim` reader - "]
pub type TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Trim` writer - "]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_0E_SPEC, u8, u8, 3, O>;
#[doc = "Field `FPOR` reader - "]
pub type FPOR_R = crate::BitReader<bool>;
#[doc = "Field `FPOR` writer - "]
pub type FPOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_0E_SPEC, bool, O>;
#[doc = "Field `BKP_REG` reader - "]
pub type BKP_REG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BKP_REG` writer - "]
pub type BKP_REG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_0E_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn select_ri(&self) -> SELECT_RI_R {
        SELECT_RI_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn jtaga(&self) -> JTAGA_R {
        JTAGA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn jtagb(&self) -> JTAGB_R {
        JTAGB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn fpor(&self) -> FPOR_R {
        FPOR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    pub fn bkp_reg(&self) -> BKP_REG_R {
        BKP_REG_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn low(&mut self) -> LOW_W<0> {
        LOW_W::new(self)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    #[must_use]
    pub fn select_ri(&mut self) -> SELECT_RI_W<3> {
        SELECT_RI_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn jtaga(&mut self) -> JTAGA_W<6> {
        JTAGA_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn jtagb(&mut self) -> JTAGB_W<7> {
        JTAGB_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<8> {
        TRIM_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn fpor(&mut self) -> FPOR_W<11> {
        FPOR_W::new(self)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    #[must_use]
    pub fn bkp_reg(&mut self) -> BKP_REG_W<12> {
        BKP_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_0e](index.html) module"]
pub struct REG_0E_SPEC;
impl crate::RegisterSpec for REG_0E_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_0e::R](R) reader structure"]
impl crate::Readable for REG_0E_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_0e::W](W) writer structure"]
impl crate::Writable for REG_0E_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_0E to value 0"]
impl crate::Resettable for REG_0E_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
