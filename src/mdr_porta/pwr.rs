#[doc = "Register `PWR` reader"]
pub struct R(crate::R<PWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR` writer"]
pub struct W(crate::W<PWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_SPEC>;
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
impl From<crate::W<PWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWR_0` reader - "]
pub type PWR_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_0` writer - "]
pub type PWR_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_1` reader - "]
pub type PWR_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_1` writer - "]
pub type PWR_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_2` reader - "]
pub type PWR_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_2` writer - "]
pub type PWR_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_3` reader - "]
pub type PWR_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_3` writer - "]
pub type PWR_3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_4` reader - "]
pub type PWR_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_4` writer - "]
pub type PWR_4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_5` reader - "]
pub type PWR_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_5` writer - "]
pub type PWR_5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_6` reader - "]
pub type PWR_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_6` writer - "]
pub type PWR_6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_7` reader - "]
pub type PWR_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_7` writer - "]
pub type PWR_7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_8` reader - "]
pub type PWR_8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_8` writer - "]
pub type PWR_8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_9` reader - "]
pub type PWR_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_9` writer - "]
pub type PWR_9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_10` reader - "]
pub type PWR_10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_10` writer - "]
pub type PWR_10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_11` reader - "]
pub type PWR_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_11` writer - "]
pub type PWR_11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_12` reader - "]
pub type PWR_12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_12` writer - "]
pub type PWR_12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_13` reader - "]
pub type PWR_13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_13` writer - "]
pub type PWR_13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_14` reader - "]
pub type PWR_14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_14` writer - "]
pub type PWR_14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_15` reader - "]
pub type PWR_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_15` writer - "]
pub type PWR_15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pwr_0(&self) -> PWR_0_R {
        PWR_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pwr_1(&self) -> PWR_1_R {
        PWR_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pwr_2(&self) -> PWR_2_R {
        PWR_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pwr_3(&self) -> PWR_3_R {
        PWR_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pwr_4(&self) -> PWR_4_R {
        PWR_4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pwr_5(&self) -> PWR_5_R {
        PWR_5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pwr_6(&self) -> PWR_6_R {
        PWR_6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pwr_7(&self) -> PWR_7_R {
        PWR_7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pwr_8(&self) -> PWR_8_R {
        PWR_8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pwr_9(&self) -> PWR_9_R {
        PWR_9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pwr_10(&self) -> PWR_10_R {
        PWR_10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pwr_11(&self) -> PWR_11_R {
        PWR_11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pwr_12(&self) -> PWR_12_R {
        PWR_12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pwr_13(&self) -> PWR_13_R {
        PWR_13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pwr_14(&self) -> PWR_14_R {
        PWR_14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pwr_15(&self) -> PWR_15_R {
        PWR_15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_0(&mut self) -> PWR_0_W<0> {
        PWR_0_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_1(&mut self) -> PWR_1_W<2> {
        PWR_1_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_2(&mut self) -> PWR_2_W<4> {
        PWR_2_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_3(&mut self) -> PWR_3_W<6> {
        PWR_3_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_4(&mut self) -> PWR_4_W<8> {
        PWR_4_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_5(&mut self) -> PWR_5_W<10> {
        PWR_5_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_6(&mut self) -> PWR_6_W<12> {
        PWR_6_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_7(&mut self) -> PWR_7_W<14> {
        PWR_7_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_8(&mut self) -> PWR_8_W<16> {
        PWR_8_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_9(&mut self) -> PWR_9_W<18> {
        PWR_9_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_10(&mut self) -> PWR_10_W<20> {
        PWR_10_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_11(&mut self) -> PWR_11_W<22> {
        PWR_11_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_12(&mut self) -> PWR_12_W<24> {
        PWR_12_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_13(&mut self) -> PWR_13_W<26> {
        PWR_13_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_14(&mut self) -> PWR_14_W<28> {
        PWR_14_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_15(&mut self) -> PWR_15_W<30> {
        PWR_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Power Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr](index.html) module"]
pub struct PWR_SPEC;
impl crate::RegisterSpec for PWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr::R](R) reader structure"]
impl crate::Readable for PWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr::W](W) writer structure"]
impl crate::Writable for PWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR to value 0"]
impl crate::Resettable for PWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
