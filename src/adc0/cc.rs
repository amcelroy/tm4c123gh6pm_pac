# [doc = "Register `CC` reader"] pub struct R (crate :: R < CC_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < CC_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < CC_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < CC_SPEC >) -> Self { R (reader) } } # [doc = "Register `CC` writer"] pub struct W (crate :: W < CC_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < CC_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < CC_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < CC_SPEC >) -> Self { W (writer) } } # [doc = "Field `ADC_CC_CS` reader - ADC Clock Source"] pub type ADC_CC_CS_R = crate :: FieldReader < u8 , ADC_CC_CS_A > ; # [doc = "ADC Clock Source\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum ADC_CC_CS_A { # [doc = "0: PLL VCO divided by CLKDIV"] ADC_CC_CS_SYSPLL = 0 , # [doc = "1: PIOSC"] ADC_CC_CS_PIOSC = 1 , } impl From < ADC_CC_CS_A > for u8 { # [inline (always)] fn from (variant : ADC_CC_CS_A) -> Self { variant as _ } } impl ADC_CC_CS_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub fn variant (& self) -> Option < ADC_CC_CS_A > { match self . bits { 0 => Some (ADC_CC_CS_A :: ADC_CC_CS_SYSPLL) , 1 => Some (ADC_CC_CS_A :: ADC_CC_CS_PIOSC) , _ => None , } } # [doc = "Checks if the value of the field is `ADC_CC_CS_SYSPLL`"] # [inline (always)] pub fn is_adc_cc_cs_syspll (& self) -> bool { * self == ADC_CC_CS_A :: ADC_CC_CS_SYSPLL } # [doc = "Checks if the value of the field is `ADC_CC_CS_PIOSC`"] # [inline (always)] pub fn is_adc_cc_cs_piosc (& self) -> bool { * self == ADC_CC_CS_A :: ADC_CC_CS_PIOSC } } # [doc = "Field `ADC_CC_CS` writer - ADC Clock Source"] pub type ADC_CC_CS_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , CC_SPEC , u8 , ADC_CC_CS_A , 4 , O > ; impl < 'a , const O : u8 > ADC_CC_CS_W < 'a , O > { # [doc = "PLL VCO divided by CLKDIV"] # [inline (always)] pub fn adc_cc_cs_syspll (self) -> & 'a mut W { self . variant (ADC_CC_CS_A :: ADC_CC_CS_SYSPLL) } # [doc = "PIOSC"] # [inline (always)] pub fn adc_cc_cs_piosc (self) -> & 'a mut W { self . variant (ADC_CC_CS_A :: ADC_CC_CS_PIOSC) } } impl R { # [doc = "Bits 0:3 - ADC Clock Source"] # [inline (always)] pub fn adc_cc_cs (& self) -> ADC_CC_CS_R { ADC_CC_CS_R :: new ((self . bits & 0x0f) as u8) } } impl W { # [doc = "Bits 0:3 - ADC Clock Source"] # [inline (always)] # [must_use] pub fn adc_cc_cs (& mut self) -> ADC_CC_CS_W < 0 > { ADC_CC_CS_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "ADC Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](index.html) module"] pub struct CC_SPEC ; impl crate :: RegisterSpec for CC_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [cc::R](R) reader structure"] impl crate :: Readable for CC_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [cc::W](W) writer structure"] impl crate :: Writable for CC_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CC to value 0"] impl crate :: Resettable for CC_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }