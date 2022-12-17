# [doc = "Register `_0_COUNT` reader"] pub struct R (crate :: R < _0_COUNT_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < _0_COUNT_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < _0_COUNT_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < _0_COUNT_SPEC >) -> Self { R (reader) } } # [doc = "Register `_0_COUNT` writer"] pub struct W (crate :: W < _0_COUNT_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < _0_COUNT_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < _0_COUNT_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < _0_COUNT_SPEC >) -> Self { W (writer) } } # [doc = "Field `PWM_0_COUNT` reader - Counter Value"] pub type PWM_0_COUNT_R = crate :: FieldReader < u16 , u16 > ; # [doc = "Field `PWM_0_COUNT` writer - Counter Value"] pub type PWM_0_COUNT_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , _0_COUNT_SPEC , u16 , u16 , 16 , O > ; impl R { # [doc = "Bits 0:15 - Counter Value"] # [inline (always)] pub fn pwm_0_count (& self) -> PWM_0_COUNT_R { PWM_0_COUNT_R :: new ((self . bits & 0xffff) as u16) } } impl W { # [doc = "Bits 0:15 - Counter Value"] # [inline (always)] # [must_use] pub fn pwm_0_count (& mut self) -> PWM_0_COUNT_W < 0 > { PWM_0_COUNT_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "PWM0 Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_count](index.html) module"] pub struct _0_COUNT_SPEC ; impl crate :: RegisterSpec for _0_COUNT_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [_0_count::R](R) reader structure"] impl crate :: Readable for _0_COUNT_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [_0_count::W](W) writer structure"] impl crate :: Writable for _0_COUNT_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets _0_COUNT to value 0"] impl crate :: Resettable for _0_COUNT_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }