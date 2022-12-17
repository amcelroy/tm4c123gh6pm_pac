# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Hibernation RTC Counter"] pub rtcc : RTCC , # [doc = "0x04 - Hibernation RTC Match 0"] pub rtcm0 : RTCM0 , _reserved2 : [u8 ; 0x04] , # [doc = "0x0c - Hibernation RTC Load"] pub rtcld : RTCLD , # [doc = "0x10 - Hibernation Control"] pub ctl : CTL , # [doc = "0x14 - Hibernation Interrupt Mask"] pub im : IM , # [doc = "0x18 - Hibernation Raw Interrupt Status"] pub ris : RIS , # [doc = "0x1c - Hibernation Masked Interrupt Status"] pub mis : MIS , # [doc = "0x20 - Hibernation Interrupt Clear"] pub ic : IC , # [doc = "0x24 - Hibernation RTC Trim"] pub rtct : RTCT , # [doc = "0x28 - Hibernation RTC Sub Seconds"] pub rtcss : RTCSS , _reserved10 : [u8 ; 0x04] , # [doc = "0x30 - Hibernation Data"] pub data : DATA , } # [doc = "RTCC (rw) register accessor: an alias for `Reg<RTCC_SPEC>`"] pub type RTCC = crate :: Reg < rtcc :: RTCC_SPEC > ; # [doc = "Hibernation RTC Counter"] pub mod rtcc ; # [doc = "RTCM0 (rw) register accessor: an alias for `Reg<RTCM0_SPEC>`"] pub type RTCM0 = crate :: Reg < rtcm0 :: RTCM0_SPEC > ; # [doc = "Hibernation RTC Match 0"] pub mod rtcm0 ; # [doc = "RTCLD (rw) register accessor: an alias for `Reg<RTCLD_SPEC>`"] pub type RTCLD = crate :: Reg < rtcld :: RTCLD_SPEC > ; # [doc = "Hibernation RTC Load"] pub mod rtcld ; # [doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"] pub type CTL = crate :: Reg < ctl :: CTL_SPEC > ; # [doc = "Hibernation Control"] pub mod ctl ; # [doc = "IM (rw) register accessor: an alias for `Reg<IM_SPEC>`"] pub type IM = crate :: Reg < im :: IM_SPEC > ; # [doc = "Hibernation Interrupt Mask"] pub mod im ; # [doc = "RIS (rw) register accessor: an alias for `Reg<RIS_SPEC>`"] pub type RIS = crate :: Reg < ris :: RIS_SPEC > ; # [doc = "Hibernation Raw Interrupt Status"] pub mod ris ; # [doc = "MIS (rw) register accessor: an alias for `Reg<MIS_SPEC>`"] pub type MIS = crate :: Reg < mis :: MIS_SPEC > ; # [doc = "Hibernation Masked Interrupt Status"] pub mod mis ; # [doc = "IC (rw) register accessor: an alias for `Reg<IC_SPEC>`"] pub type IC = crate :: Reg < ic :: IC_SPEC > ; # [doc = "Hibernation Interrupt Clear"] pub mod ic ; # [doc = "RTCT (rw) register accessor: an alias for `Reg<RTCT_SPEC>`"] pub type RTCT = crate :: Reg < rtct :: RTCT_SPEC > ; # [doc = "Hibernation RTC Trim"] pub mod rtct ; # [doc = "RTCSS (rw) register accessor: an alias for `Reg<RTCSS_SPEC>`"] pub type RTCSS = crate :: Reg < rtcss :: RTCSS_SPEC > ; # [doc = "Hibernation RTC Sub Seconds"] pub mod rtcss ; # [doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"] pub type DATA = crate :: Reg < data :: DATA_SPEC > ; # [doc = "Hibernation Data"] pub mod data ;