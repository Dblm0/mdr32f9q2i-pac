#[doc = "Register `BUF_00_DLC` reader"]
pub struct R(crate::R<BUF_00_DLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_00_DLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_00_DLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_00_DLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_00_DLC` writer"]
pub struct W(crate::W<BUF_00_DLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_00_DLC_SPEC>;
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
impl From<crate::W<BUF_00_DLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_00_DLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLC` reader - "]
pub type DLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLC` writer - "]
pub type DLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_00_DLC_SPEC, u8, u8, 4, O>;
#[doc = "Field `RTR` reader - "]
pub type RTR_R = crate::BitReader<bool>;
#[doc = "Field `RTR` writer - "]
pub type RTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_00_DLC_SPEC, bool, O>;
#[doc = "Field `R1` reader - "]
pub type R1_R = crate::BitReader<bool>;
#[doc = "Field `R1` writer - "]
pub type R1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_00_DLC_SPEC, bool, O>;
#[doc = "Field `R0` reader - "]
pub type R0_R = crate::BitReader<bool>;
#[doc = "Field `R0` writer - "]
pub type R0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_00_DLC_SPEC, bool, O>;
#[doc = "Field `SSR` reader - "]
pub type SSR_R = crate::BitReader<bool>;
#[doc = "Field `SSR` writer - "]
pub type SSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_00_DLC_SPEC, bool, O>;
#[doc = "Field `IDE` reader - "]
pub type IDE_R = crate::BitReader<bool>;
#[doc = "Field `IDE` writer - "]
pub type IDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_00_DLC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn r1(&self) -> R1_R {
        R1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ssr(&self) -> SSR_R {
        SSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<0> {
        DLC_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<8> {
        RTR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn r1(&mut self) -> R1_W<9> {
        R1_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn r0(&mut self) -> R0_W<10> {
        R0_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ssr(&mut self) -> SSR_W<11> {
        SSR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ide(&mut self) -> IDE_W<12> {
        IDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Buffer DLC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_00_dlc](index.html) module"]
pub struct BUF_00_DLC_SPEC;
impl crate::RegisterSpec for BUF_00_DLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_00_dlc::R](R) reader structure"]
impl crate::Readable for BUF_00_DLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_00_dlc::W](W) writer structure"]
impl crate::Writable for BUF_00_DLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUF_00_DLC to value 0"]
impl crate::Resettable for BUF_00_DLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
