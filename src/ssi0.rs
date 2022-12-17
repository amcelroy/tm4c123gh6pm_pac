# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - SSI Control 0"] pub cr0 : CR0 , # [doc = "0x04 - SSI Control 1"] pub cr1 : CR1 , # [doc = "0x08 - SSI Data"] pub dr : DR , # [doc = "0x0c - SSI Status"] pub sr : SR , # [doc = "0x10 - SSI Clock Prescale"] pub cpsr : CPSR , # [doc = "0x14 - SSI Interrupt Mask"] pub im : IM , # [doc = "0x18 - SSI Raw Interrupt Status"] pub ris : RIS , # [doc = "0x1c - SSI Masked Interrupt Status"] pub mis : MIS , # [doc = "0x20 - SSI Interrupt Clear"] pub icr : ICR , # [doc = "0x24 - SSI DMA Control"] pub dmactl : DMACTL , _reserved10 : [u8 ; 0x0fa0] , # [doc = "0xfc8 - SSI Clock Configuration"] pub cc : CC , } # [doc = "CR0 (rw) register accessor: an alias for `Reg<CR0_SPEC>`"] pub type CR0 = crate :: Reg < cr0 :: CR0_SPEC > ; # [doc = "SSI Control 0"] pub mod cr0 ; # [doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"] pub type CR1 = crate :: Reg < cr1 :: CR1_SPEC > ; # [doc = "SSI Control 1"] pub mod cr1 ; # [doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"] pub type DR = crate :: Reg < dr :: DR_SPEC > ; # [doc = "SSI Data"] pub mod dr ; # [doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"] pub type SR = crate :: Reg < sr :: SR_SPEC > ; # [doc = "SSI Status"] pub mod sr ; # [doc = "CPSR (rw) register accessor: an alias for `Reg<CPSR_SPEC>`"] pub type CPSR = crate :: Reg < cpsr :: CPSR_SPEC > ; # [doc = "SSI Clock Prescale"] pub mod cpsr ; # [doc = "IM (rw) register accessor: an alias for `Reg<IM_SPEC>`"] pub type IM = crate :: Reg < im :: IM_SPEC > ; # [doc = "SSI Interrupt Mask"] pub mod im ; # [doc = "RIS (rw) register accessor: an alias for `Reg<RIS_SPEC>`"] pub type RIS = crate :: Reg < ris :: RIS_SPEC > ; # [doc = "SSI Raw Interrupt Status"] pub mod ris ; # [doc = "MIS (rw) register accessor: an alias for `Reg<MIS_SPEC>`"] pub type MIS = crate :: Reg < mis :: MIS_SPEC > ; # [doc = "SSI Masked Interrupt Status"] pub mod mis ; # [doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"] pub type ICR = crate :: Reg < icr :: ICR_SPEC > ; # [doc = "SSI Interrupt Clear"] pub mod icr ; # [doc = "DMACTL (rw) register accessor: an alias for `Reg<DMACTL_SPEC>`"] pub type DMACTL = crate :: Reg < dmactl :: DMACTL_SPEC > ; # [doc = "SSI DMA Control"] pub mod dmactl ; # [doc = "CC (rw) register accessor: an alias for `Reg<CC_SPEC>`"] pub type CC = crate :: Reg < cc :: CC_SPEC > ; # [doc = "SSI Clock Configuration"] pub mod cc ;