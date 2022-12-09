#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Data Register"]
    pub dr: DR,
    #[doc = "0x04 - UART RSR Register"]
    pub rsr_ecr: RSR_ECR,
    _reserved2: [u8; 0x10],
    #[doc = "0x18 - UART Flag Register"]
    pub fr: FR,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - "]
    pub ilpr: ILPR,
    #[doc = "0x24 - "]
    pub ibrd: IBRD,
    #[doc = "0x28 - "]
    pub fbrd: FBRD,
    #[doc = "0x2c - UART LCR_H Register"]
    pub lcr_h: LCR_H,
    #[doc = "0x30 - UART Command Register"]
    pub cr: CR,
    #[doc = "0x34 - UART IFLS Register"]
    pub ifls: IFLS,
    #[doc = "0x38 - UART Interrupt Mask Register"]
    pub imsc: IMSC,
    #[doc = "0x3c - UART Interrupt Pending Register"]
    pub ris: RIS,
    #[doc = "0x40 - UART Masked Interrupt Pending Register"]
    pub mis: MIS,
    #[doc = "0x44 - UART Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x48 - UART DMA Control Register"]
    pub dmacr: DMACR,
}
#[doc = "DR (w) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "UART Data Register"]
pub mod dr;
#[doc = "RSR_ECR (rw) register accessor: an alias for `Reg<RSR_ECR_SPEC>`"]
pub type RSR_ECR = crate::Reg<rsr_ecr::RSR_ECR_SPEC>;
#[doc = "UART RSR Register"]
pub mod rsr_ecr;
#[doc = "FR (rw) register accessor: an alias for `Reg<FR_SPEC>`"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "UART Flag Register"]
pub mod fr;
#[doc = "ILPR (rw) register accessor: an alias for `Reg<ILPR_SPEC>`"]
pub type ILPR = crate::Reg<ilpr::ILPR_SPEC>;
#[doc = ""]
pub mod ilpr;
#[doc = "IBRD (rw) register accessor: an alias for `Reg<IBRD_SPEC>`"]
pub type IBRD = crate::Reg<ibrd::IBRD_SPEC>;
#[doc = ""]
pub mod ibrd;
#[doc = "FBRD (rw) register accessor: an alias for `Reg<FBRD_SPEC>`"]
pub type FBRD = crate::Reg<fbrd::FBRD_SPEC>;
#[doc = ""]
pub mod fbrd;
#[doc = "LCR_H (rw) register accessor: an alias for `Reg<LCR_H_SPEC>`"]
pub type LCR_H = crate::Reg<lcr_h::LCR_H_SPEC>;
#[doc = "UART LCR_H Register"]
pub mod lcr_h;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "UART Command Register"]
pub mod cr;
#[doc = "IFLS (rw) register accessor: an alias for `Reg<IFLS_SPEC>`"]
pub type IFLS = crate::Reg<ifls::IFLS_SPEC>;
#[doc = "UART IFLS Register"]
pub mod ifls;
#[doc = "IMSC (rw) register accessor: an alias for `Reg<IMSC_SPEC>`"]
pub type IMSC = crate::Reg<imsc::IMSC_SPEC>;
#[doc = "UART Interrupt Mask Register"]
pub mod imsc;
#[doc = "RIS (rw) register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "UART Interrupt Pending Register"]
pub mod ris;
#[doc = "MIS (rw) register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "UART Masked Interrupt Pending Register"]
pub mod mis;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "UART Interrupt Clear Register"]
pub mod icr;
#[doc = "DMACR (rw) register accessor: an alias for `Reg<DMACR_SPEC>`"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "UART DMA Control Register"]
pub mod dmacr;
