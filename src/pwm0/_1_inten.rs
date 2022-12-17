# [doc = "Register `_1_INTEN` reader"] pub struct R (crate :: R < _1_INTEN_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < _1_INTEN_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < _1_INTEN_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < _1_INTEN_SPEC >) -> Self { R (reader) } } # [doc = "Register `_1_INTEN` writer"] pub struct W (crate :: W < _1_INTEN_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < _1_INTEN_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < _1_INTEN_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < _1_INTEN_SPEC >) -> Self { W (writer) } } # [doc = "Field `PWM_1_INTEN_INTCNTZERO` reader - Interrupt for Counter=0"] pub type PWM_1_INTEN_INTCNTZERO_R = crate :: BitReader < bool > ; # [doc = "Field `PWM_1_INTEN_INTCNTZERO` writer - Interrupt for Counter=0"] pub type PWM_1_INTEN_INTCNTZERO_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , _1_INTEN_SPEC , bool , O > ; # [doc = "Field `PWM_1_INTEN_INTCNTLOAD` reader - Interrupt for Counter=PWMnLOAD"] pub type PWM_1_INTEN_INTCNTLOAD_R = crate :: BitReader < bool > ; # [doc = "Field `PWM_1_INTEN_INTCNTLOAD` writer - Interrupt for Counter=PWMnLOAD"] pub type PWM_1_INTEN_INTCNTLOAD_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , _1_INTEN_SPEC , bool , O > ; # [doc = "Field `PWM_1_INTEN_INTCMPAU` reader - Interrupt for Counter=PWMnCMPA Up"] pub type PWM_1_INTEN_INTCMPAU_R = crate :: BitReader < bool > ; # [doc = "Field `PWM_1_INTEN_INTCMPAU` writer - Interrupt for Counter=PWMnCMPA Up"] pub type PWM_1_INTEN_INTCMPAU_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , _1_INTEN_SPEC , bool , O > ; # [doc = "Field `PWM_1_INTEN_INTCMPAD` reader - Interrupt for Counter=PWMnCMPA Down"] pub type PWM_1_INTEN_INTCMPAD_R = crate :: BitReader < bool > ; # [doc = "Field `PWM_1_INTEN_INTCMPAD` writer - Interrupt for Counter=PWMnCMPA Down"] pub type PWM_1_INTEN_INTCMPAD_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , _1_INTEN_SPEC , bool , O > ; # [doc = "Field `PWM_1_INTEN_INTCMPBU` reader - Interrupt for Counter=PWMnCMPB Up"] pub type PWM_1_INTEN_INTCMPBU_R = crate :: BitReader < bool > ; # [doc = "Field `PWM_1_INTEN_INTCMPBU` writer - Interrupt for Counter=PWMnCMPB Up"] pub type PWM_1_INTEN_INTCMPBU_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , _1_INTEN_SPEC , bool , O > ; # [doc = "Field `PWM_1_INTEN_INTCMPBD` reader - Interrupt for Counter=PWMnCMPB Down"] pub type PWM_1_INTEN_INTCMPBD_R = crate :: BitReader < bool > ; # [doc = "Field `PWM_1_INTEN_INTCMPBD` writer - Interrupt for Counter=PWMnCMPB Down"] pub type PWM_1_INTEN_INTCMPBD_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , _1_INTEN_SPEC , bool , O > ; # [doc = "Field `PWM_1_INTEN_TRCNTZERO` reader - Trigger for Counter=0"] pub type PWM_1_INTEN_TRCNTZERO_R = crate :: BitReader < bool > ; # [doc = "Field `PWM_1_INTEN_TRCNTZERO` writer - Trigger for Counter=0"] pub type PWM_1_INTEN_TRCNTZERO_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , _1_INTEN_SPEC , bool , O > ; # [doc = "Field `PWM_1_INTEN_TRCNTLOAD` reader - Trigger for Counter=PWMnLOAD"] pub type PWM_1_INTEN_TRCNTLOAD_R = crate :: BitReader < bool > ; # [doc = "Field `PWM_1_INTEN_TRCNTLOAD` writer - Trigger for Counter=PWMnLOAD"] pub type PWM_1_INTEN_TRCNTLOAD_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , _1_INTEN_SPEC , bool , O > ; # [doc = "Field `PWM_1_INTEN_TRCMPAU` reader - Trigger for Counter=PWMnCMPA Up"] pub type PWM_1_INTEN_TRCMPAU_R = crate :: BitReader < bool > ; # [doc = "Field `PWM_1_INTEN_TRCMPAU` writer - Trigger for Counter=PWMnCMPA Up"] pub type PWM_1_INTEN_TRCMPAU_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , _1_INTEN_SPEC , bool , O > ; # [doc = "Field `PWM_1_INTEN_TRCMPAD` reader - Trigger for Counter=PWMnCMPA Down"] pub type PWM_1_INTEN_TRCMPAD_R = crate :: BitReader < bool > ; # [doc = "Field `PWM_1_INTEN_TRCMPAD` writer - Trigger for Counter=PWMnCMPA Down"] pub type PWM_1_INTEN_TRCMPAD_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , _1_INTEN_SPEC , bool , O > ; # [doc = "Field `PWM_1_INTEN_TRCMPBU` reader - Trigger for Counter=PWMnCMPB Up"] pub type PWM_1_INTEN_TRCMPBU_R = crate :: BitReader < bool > ; # [doc = "Field `PWM_1_INTEN_TRCMPBU` writer - Trigger for Counter=PWMnCMPB Up"] pub type PWM_1_INTEN_TRCMPBU_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , _1_INTEN_SPEC , bool , O > ; # [doc = "Field `PWM_1_INTEN_TRCMPBD` reader - Trigger for Counter=PWMnCMPB Down"] pub type PWM_1_INTEN_TRCMPBD_R = crate :: BitReader < bool > ; # [doc = "Field `PWM_1_INTEN_TRCMPBD` writer - Trigger for Counter=PWMnCMPB Down"] pub type PWM_1_INTEN_TRCMPBD_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , _1_INTEN_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - Interrupt for Counter=0"] # [inline (always)] pub fn pwm_1_inten_intcntzero (& self) -> PWM_1_INTEN_INTCNTZERO_R { PWM_1_INTEN_INTCNTZERO_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Interrupt for Counter=PWMnLOAD"] # [inline (always)] pub fn pwm_1_inten_intcntload (& self) -> PWM_1_INTEN_INTCNTLOAD_R { PWM_1_INTEN_INTCNTLOAD_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Interrupt for Counter=PWMnCMPA Up"] # [inline (always)] pub fn pwm_1_inten_intcmpau (& self) -> PWM_1_INTEN_INTCMPAU_R { PWM_1_INTEN_INTCMPAU_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - Interrupt for Counter=PWMnCMPA Down"] # [inline (always)] pub fn pwm_1_inten_intcmpad (& self) -> PWM_1_INTEN_INTCMPAD_R { PWM_1_INTEN_INTCMPAD_R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - Interrupt for Counter=PWMnCMPB Up"] # [inline (always)] pub fn pwm_1_inten_intcmpbu (& self) -> PWM_1_INTEN_INTCMPBU_R { PWM_1_INTEN_INTCMPBU_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - Interrupt for Counter=PWMnCMPB Down"] # [inline (always)] pub fn pwm_1_inten_intcmpbd (& self) -> PWM_1_INTEN_INTCMPBD_R { PWM_1_INTEN_INTCMPBD_R :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 8 - Trigger for Counter=0"] # [inline (always)] pub fn pwm_1_inten_trcntzero (& self) -> PWM_1_INTEN_TRCNTZERO_R { PWM_1_INTEN_TRCNTZERO_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - Trigger for Counter=PWMnLOAD"] # [inline (always)] pub fn pwm_1_inten_trcntload (& self) -> PWM_1_INTEN_TRCNTLOAD_R { PWM_1_INTEN_TRCNTLOAD_R :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 10 - Trigger for Counter=PWMnCMPA Up"] # [inline (always)] pub fn pwm_1_inten_trcmpau (& self) -> PWM_1_INTEN_TRCMPAU_R { PWM_1_INTEN_TRCMPAU_R :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bit 11 - Trigger for Counter=PWMnCMPA Down"] # [inline (always)] pub fn pwm_1_inten_trcmpad (& self) -> PWM_1_INTEN_TRCMPAD_R { PWM_1_INTEN_TRCMPAD_R :: new (((self . bits >> 11) & 1) != 0) } # [doc = "Bit 12 - Trigger for Counter=PWMnCMPB Up"] # [inline (always)] pub fn pwm_1_inten_trcmpbu (& self) -> PWM_1_INTEN_TRCMPBU_R { PWM_1_INTEN_TRCMPBU_R :: new (((self . bits >> 12) & 1) != 0) } # [doc = "Bit 13 - Trigger for Counter=PWMnCMPB Down"] # [inline (always)] pub fn pwm_1_inten_trcmpbd (& self) -> PWM_1_INTEN_TRCMPBD_R { PWM_1_INTEN_TRCMPBD_R :: new (((self . bits >> 13) & 1) != 0) } } impl W { # [doc = "Bit 0 - Interrupt for Counter=0"] # [inline (always)] # [must_use] pub fn pwm_1_inten_intcntzero (& mut self) -> PWM_1_INTEN_INTCNTZERO_W < 0 > { PWM_1_INTEN_INTCNTZERO_W :: new (self) } # [doc = "Bit 1 - Interrupt for Counter=PWMnLOAD"] # [inline (always)] # [must_use] pub fn pwm_1_inten_intcntload (& mut self) -> PWM_1_INTEN_INTCNTLOAD_W < 1 > { PWM_1_INTEN_INTCNTLOAD_W :: new (self) } # [doc = "Bit 2 - Interrupt for Counter=PWMnCMPA Up"] # [inline (always)] # [must_use] pub fn pwm_1_inten_intcmpau (& mut self) -> PWM_1_INTEN_INTCMPAU_W < 2 > { PWM_1_INTEN_INTCMPAU_W :: new (self) } # [doc = "Bit 3 - Interrupt for Counter=PWMnCMPA Down"] # [inline (always)] # [must_use] pub fn pwm_1_inten_intcmpad (& mut self) -> PWM_1_INTEN_INTCMPAD_W < 3 > { PWM_1_INTEN_INTCMPAD_W :: new (self) } # [doc = "Bit 4 - Interrupt for Counter=PWMnCMPB Up"] # [inline (always)] # [must_use] pub fn pwm_1_inten_intcmpbu (& mut self) -> PWM_1_INTEN_INTCMPBU_W < 4 > { PWM_1_INTEN_INTCMPBU_W :: new (self) } # [doc = "Bit 5 - Interrupt for Counter=PWMnCMPB Down"] # [inline (always)] # [must_use] pub fn pwm_1_inten_intcmpbd (& mut self) -> PWM_1_INTEN_INTCMPBD_W < 5 > { PWM_1_INTEN_INTCMPBD_W :: new (self) } # [doc = "Bit 8 - Trigger for Counter=0"] # [inline (always)] # [must_use] pub fn pwm_1_inten_trcntzero (& mut self) -> PWM_1_INTEN_TRCNTZERO_W < 8 > { PWM_1_INTEN_TRCNTZERO_W :: new (self) } # [doc = "Bit 9 - Trigger for Counter=PWMnLOAD"] # [inline (always)] # [must_use] pub fn pwm_1_inten_trcntload (& mut self) -> PWM_1_INTEN_TRCNTLOAD_W < 9 > { PWM_1_INTEN_TRCNTLOAD_W :: new (self) } # [doc = "Bit 10 - Trigger for Counter=PWMnCMPA Up"] # [inline (always)] # [must_use] pub fn pwm_1_inten_trcmpau (& mut self) -> PWM_1_INTEN_TRCMPAU_W < 10 > { PWM_1_INTEN_TRCMPAU_W :: new (self) } # [doc = "Bit 11 - Trigger for Counter=PWMnCMPA Down"] # [inline (always)] # [must_use] pub fn pwm_1_inten_trcmpad (& mut self) -> PWM_1_INTEN_TRCMPAD_W < 11 > { PWM_1_INTEN_TRCMPAD_W :: new (self) } # [doc = "Bit 12 - Trigger for Counter=PWMnCMPB Up"] # [inline (always)] # [must_use] pub fn pwm_1_inten_trcmpbu (& mut self) -> PWM_1_INTEN_TRCMPBU_W < 12 > { PWM_1_INTEN_TRCMPBU_W :: new (self) } # [doc = "Bit 13 - Trigger for Counter=PWMnCMPB Down"] # [inline (always)] # [must_use] pub fn pwm_1_inten_trcmpbd (& mut self) -> PWM_1_INTEN_TRCMPBD_W < 13 > { PWM_1_INTEN_TRCMPBD_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "PWM1 Interrupt and Trigger Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_inten](index.html) module"] pub struct _1_INTEN_SPEC ; impl crate :: RegisterSpec for _1_INTEN_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [_1_inten::R](R) reader structure"] impl crate :: Readable for _1_INTEN_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [_1_inten::W](W) writer structure"] impl crate :: Writable for _1_INTEN_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets _1_INTEN to value 0"] impl crate :: Resettable for _1_INTEN_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }