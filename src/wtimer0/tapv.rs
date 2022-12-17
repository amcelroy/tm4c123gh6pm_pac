# [doc = "Register `TAPV` reader"] pub struct R (crate :: R < TAPV_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < TAPV_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < TAPV_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < TAPV_SPEC >) -> Self { R (reader) } } # [doc = "Register `TAPV` writer"] pub struct W (crate :: W < TAPV_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < TAPV_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < TAPV_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < TAPV_SPEC >) -> Self { W (writer) } } # [doc = "Field `TIMER_TAPV_PSV` reader - GPTM Timer A Prescaler Value"] pub type TIMER_TAPV_PSV_R = crate :: FieldReader < u16 , u16 > ; # [doc = "Field `TIMER_TAPV_PSV` writer - GPTM Timer A Prescaler Value"] pub type TIMER_TAPV_PSV_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , TAPV_SPEC , u16 , u16 , 16 , O > ; impl R { # [doc = "Bits 0:15 - GPTM Timer A Prescaler Value"] # [inline (always)] pub fn timer_tapv_psv (& self) -> TIMER_TAPV_PSV_R { TIMER_TAPV_PSV_R :: new ((self . bits & 0xffff) as u16) } } impl W { # [doc = "Bits 0:15 - GPTM Timer A Prescaler Value"] # [inline (always)] # [must_use] pub fn timer_tapv_psv (& mut self) -> TIMER_TAPV_PSV_W < 0 > { TIMER_TAPV_PSV_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "GPTM Timer A Prescale Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tapv](index.html) module"] pub struct TAPV_SPEC ; impl crate :: RegisterSpec for TAPV_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [tapv::R](R) reader structure"] impl crate :: Readable for TAPV_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [tapv::W](W) writer structure"] impl crate :: Writable for TAPV_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets TAPV to value 0"] impl crate :: Resettable for TAPV_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }