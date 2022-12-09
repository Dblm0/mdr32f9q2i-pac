#[doc = "Register `CH1_CNTRL` reader"]
pub struct R(crate::R<CH1_CNTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1_CNTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1_CNTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1_CNTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1_CNTRL` writer"]
pub struct W(crate::W<CH1_CNTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1_CNTRL_SPEC>;
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
impl From<crate::W<CH1_CNTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1_CNTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHFLTR` reader - "]
pub type CHFLTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHFLTR` writer - "]
pub type CHFLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1_CNTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CHSEL` reader - "]
pub type CHSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHSEL` writer - "]
pub type CHSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1_CNTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CHPSC` reader - "]
pub type CHPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHPSC` writer - "]
pub type CHPSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1_CNTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `OCCE` reader - "]
pub type OCCE_R = crate::BitReader<bool>;
#[doc = "Field `OCCE` writer - "]
pub type OCCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1_CNTRL_SPEC, bool, O>;
#[doc = "Field `OCCM` reader - "]
pub type OCCM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OCCM` writer - "]
pub type OCCM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1_CNTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `BRKEN` reader - "]
pub type BRKEN_R = crate::BitReader<bool>;
#[doc = "Field `BRKEN` writer - "]
pub type BRKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1_CNTRL_SPEC, bool, O>;
#[doc = "Field `ETREN` reader - "]
pub type ETREN_R = crate::BitReader<bool>;
#[doc = "Field `ETREN` writer - "]
pub type ETREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1_CNTRL_SPEC, bool, O>;
#[doc = "Field `WR_CMPL` reader - "]
pub type WR_CMPL_R = crate::BitReader<bool>;
#[doc = "Field `WR_CMPL` writer - "]
pub type WR_CMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1_CNTRL_SPEC, bool, O>;
#[doc = "Field `CAP_nPWM` reader - "]
pub type CAP_N_PWM_R = crate::BitReader<bool>;
#[doc = "Field `CAP_nPWM` writer - "]
pub type CAP_N_PWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1_CNTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn chfltr(&self) -> CHFLTR_R {
        CHFLTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn chpsc(&self) -> CHPSC_R {
        CHPSC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn occe(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn occm(&self) -> OCCM_R {
        OCCM_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn brken(&self) -> BRKEN_R {
        BRKEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn etren(&self) -> ETREN_R {
        ETREN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wr_cmpl(&self) -> WR_CMPL_R {
        WR_CMPL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cap_n_pwm(&self) -> CAP_N_PWM_R {
        CAP_N_PWM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn chfltr(&mut self) -> CHFLTR_W<0> {
        CHFLTR_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<4> {
        CHSEL_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn chpsc(&mut self) -> CHPSC_W<6> {
        CHPSC_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn occe(&mut self) -> OCCE_W<8> {
        OCCE_W::new(self)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn occm(&mut self) -> OCCM_W<9> {
        OCCM_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn brken(&mut self) -> BRKEN_W<12> {
        BRKEN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn etren(&mut self) -> ETREN_W<13> {
        ETREN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn wr_cmpl(&mut self) -> WR_CMPL_W<14> {
        WR_CMPL_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn cap_n_pwm(&mut self) -> CAP_N_PWM_W<15> {
        CAP_N_PWM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_cntrl](index.html) module"]
pub struct CH1_CNTRL_SPEC;
impl crate::RegisterSpec for CH1_CNTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1_cntrl::R](R) reader structure"]
impl crate::Readable for CH1_CNTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1_cntrl::W](W) writer structure"]
impl crate::Writable for CH1_CNTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1_CNTRL to value 0"]
impl crate::Resettable for CH1_CNTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
