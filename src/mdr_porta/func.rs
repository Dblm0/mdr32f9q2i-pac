#[doc = "Register `FUNC` reader"]
pub struct R(crate::R<FUNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC` writer"]
pub struct W(crate::W<FUNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC_SPEC>;
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
impl From<crate::W<FUNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE_0` reader - "]
pub type MODE_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_0` writer - "]
pub type MODE_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_1` reader - "]
pub type MODE_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_1` writer - "]
pub type MODE_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_2` reader - "]
pub type MODE_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_2` writer - "]
pub type MODE_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_3` reader - "]
pub type MODE_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_3` writer - "]
pub type MODE_3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_4` reader - "]
pub type MODE_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_4` writer - "]
pub type MODE_4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_5` reader - "]
pub type MODE_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_5` writer - "]
pub type MODE_5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_6` reader - "]
pub type MODE_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_6` writer - "]
pub type MODE_6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_7` reader - "]
pub type MODE_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_7` writer - "]
pub type MODE_7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_8` reader - "]
pub type MODE_8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_8` writer - "]
pub type MODE_8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_9` reader - "]
pub type MODE_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_9` writer - "]
pub type MODE_9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_10` reader - "]
pub type MODE_10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_10` writer - "]
pub type MODE_10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_11` reader - "]
pub type MODE_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_11` writer - "]
pub type MODE_11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_12` reader - "]
pub type MODE_12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_12` writer - "]
pub type MODE_12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_13` reader - "]
pub type MODE_13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_13` writer - "]
pub type MODE_13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_14` reader - "]
pub type MODE_14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_14` writer - "]
pub type MODE_14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE_15` reader - "]
pub type MODE_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE_15` writer - "]
pub type MODE_15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn mode_0(&self) -> MODE_0_R {
        MODE_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn mode_1(&self) -> MODE_1_R {
        MODE_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn mode_2(&self) -> MODE_2_R {
        MODE_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn mode_3(&self) -> MODE_3_R {
        MODE_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn mode_4(&self) -> MODE_4_R {
        MODE_4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn mode_5(&self) -> MODE_5_R {
        MODE_5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn mode_6(&self) -> MODE_6_R {
        MODE_6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn mode_7(&self) -> MODE_7_R {
        MODE_7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn mode_8(&self) -> MODE_8_R {
        MODE_8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn mode_9(&self) -> MODE_9_R {
        MODE_9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn mode_10(&self) -> MODE_10_R {
        MODE_10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn mode_11(&self) -> MODE_11_R {
        MODE_11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn mode_12(&self) -> MODE_12_R {
        MODE_12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn mode_13(&self) -> MODE_13_R {
        MODE_13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn mode_14(&self) -> MODE_14_R {
        MODE_14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn mode_15(&self) -> MODE_15_R {
        MODE_15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn mode_0(&mut self) -> MODE_0_W<0> {
        MODE_0_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn mode_1(&mut self) -> MODE_1_W<2> {
        MODE_1_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn mode_2(&mut self) -> MODE_2_W<4> {
        MODE_2_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn mode_3(&mut self) -> MODE_3_W<6> {
        MODE_3_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn mode_4(&mut self) -> MODE_4_W<8> {
        MODE_4_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn mode_5(&mut self) -> MODE_5_W<10> {
        MODE_5_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn mode_6(&mut self) -> MODE_6_W<12> {
        MODE_6_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn mode_7(&mut self) -> MODE_7_W<14> {
        MODE_7_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn mode_8(&mut self) -> MODE_8_W<16> {
        MODE_8_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn mode_9(&mut self) -> MODE_9_W<18> {
        MODE_9_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn mode_10(&mut self) -> MODE_10_W<20> {
        MODE_10_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn mode_11(&mut self) -> MODE_11_W<22> {
        MODE_11_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn mode_12(&mut self) -> MODE_12_W<24> {
        MODE_12_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn mode_13(&mut self) -> MODE_13_W<26> {
        MODE_13_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn mode_14(&mut self) -> MODE_14_W<28> {
        MODE_14_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn mode_15(&mut self) -> MODE_15_W<30> {
        MODE_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Function Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func](index.html) module"]
pub struct FUNC_SPEC;
impl crate::RegisterSpec for FUNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func::R](R) reader structure"]
impl crate::Readable for FUNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func::W](W) writer structure"]
impl crate::Writable for FUNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUNC to value 0"]
impl crate::Resettable for FUNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
