# [doc = "Register `LOCK` reader"] pub struct R (crate :: R < LOCK_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < LOCK_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < LOCK_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < LOCK_SPEC >) -> Self { R (reader) } } # [doc = "Register `LOCK` writer"] pub struct W (crate :: W < LOCK_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < LOCK_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < LOCK_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < LOCK_SPEC >) -> Self { W (writer) } } # [doc = "Field `GPIO_LOCK` reader - GPIO Lock"] pub type GPIO_LOCK_R = crate :: FieldReader < u32 , GPIO_LOCK_A > ; # [doc = "GPIO Lock\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u32)] pub enum GPIO_LOCK_A { # [doc = "0: The GPIOCR register is unlocked and may be modified"] GPIO_LOCK_UNLOCKED = 0 , # [doc = "1: The GPIOCR register is locked and may not be modified"] GPIO_LOCK_LOCKED = 1 , # [doc = "1280262987: Unlocks the GPIO_CR register"] GPIO_LOCK_KEY = 1280262987 , } impl From < GPIO_LOCK_A > for u32 { # [inline (always)] fn from (variant : GPIO_LOCK_A) -> Self { variant as _ } } impl GPIO_LOCK_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub fn variant (& self) -> Option < GPIO_LOCK_A > { match self . bits { 0 => Some (GPIO_LOCK_A :: GPIO_LOCK_UNLOCKED) , 1 => Some (GPIO_LOCK_A :: GPIO_LOCK_LOCKED) , 1280262987 => Some (GPIO_LOCK_A :: GPIO_LOCK_KEY) , _ => None , } } # [doc = "Checks if the value of the field is `GPIO_LOCK_UNLOCKED`"] # [inline (always)] pub fn is_gpio_lock_unlocked (& self) -> bool { * self == GPIO_LOCK_A :: GPIO_LOCK_UNLOCKED } # [doc = "Checks if the value of the field is `GPIO_LOCK_LOCKED`"] # [inline (always)] pub fn is_gpio_lock_locked (& self) -> bool { * self == GPIO_LOCK_A :: GPIO_LOCK_LOCKED } # [doc = "Checks if the value of the field is `GPIO_LOCK_KEY`"] # [inline (always)] pub fn is_gpio_lock_key (& self) -> bool { * self == GPIO_LOCK_A :: GPIO_LOCK_KEY } } # [doc = "Field `GPIO_LOCK` writer - GPIO Lock"] pub type GPIO_LOCK_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , LOCK_SPEC , u32 , GPIO_LOCK_A , 32 , O > ; impl < 'a , const O : u8 > GPIO_LOCK_W < 'a , O > { # [doc = "The GPIOCR register is unlocked and may be modified"] # [inline (always)] pub fn gpio_lock_unlocked (self) -> & 'a mut W { self . variant (GPIO_LOCK_A :: GPIO_LOCK_UNLOCKED) } # [doc = "The GPIOCR register is locked and may not be modified"] # [inline (always)] pub fn gpio_lock_locked (self) -> & 'a mut W { self . variant (GPIO_LOCK_A :: GPIO_LOCK_LOCKED) } # [doc = "Unlocks the GPIO_CR register"] # [inline (always)] pub fn gpio_lock_key (self) -> & 'a mut W { self . variant (GPIO_LOCK_A :: GPIO_LOCK_KEY) } } impl R { # [doc = "Bits 0:31 - GPIO Lock"] # [inline (always)] pub fn gpio_lock (& self) -> GPIO_LOCK_R { GPIO_LOCK_R :: new (self . bits) } } impl W { # [doc = "Bits 0:31 - GPIO Lock"] # [inline (always)] # [must_use] pub fn gpio_lock (& mut self) -> GPIO_LOCK_W < 0 > { GPIO_LOCK_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "GPIO Lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](index.html) module"] pub struct LOCK_SPEC ; impl crate :: RegisterSpec for LOCK_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [lock::R](R) reader structure"] impl crate :: Readable for LOCK_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [lock::W](W) writer structure"] impl crate :: Writable for LOCK_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets LOCK to value 0"] impl crate :: Resettable for LOCK_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }