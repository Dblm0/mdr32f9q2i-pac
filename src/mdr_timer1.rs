#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Counter Register"]
    pub cnt: CNT,
    #[doc = "0x04 - Timer Clock Prescaler Register"]
    pub psg: PSG,
    #[doc = "0x08 - Timer Auto-Reload Register"]
    pub arr: ARR,
    #[doc = "0x0c - Timer Control Register"]
    pub cntrl: CNTRL,
    #[doc = "0x10 - Timer Capture/Compare Register"]
    pub ccr1: CCR1,
    #[doc = "0x14 - Timer Capture/Compare Register"]
    pub ccr2: CCR2,
    #[doc = "0x18 - Timer Capture/Compare Register"]
    pub ccr3: CCR3,
    #[doc = "0x1c - Timer Capture/Compare Register"]
    pub ccr4: CCR4,
    #[doc = "0x20 - Timer Channel Control Register"]
    pub ch1_cntrl: CH1_CNTRL,
    #[doc = "0x24 - Timer Channel Control Register"]
    pub ch2_cntrl: CH2_CNTRL,
    #[doc = "0x28 - Timer Channel Control Register"]
    pub ch3_cntrl: CH3_CNTRL,
    #[doc = "0x2c - Timer Channel Control Register"]
    pub ch4_cntrl: CH4_CNTRL,
    #[doc = "0x30 - Timer Channel Control1 Register"]
    pub ch1_cntrl1: CH1_CNTRL1,
    #[doc = "0x34 - Timer Channel Control1 Register"]
    pub ch2_cntrl1: CH2_CNTRL1,
    #[doc = "0x38 - Timer Channel Control1 Register"]
    pub ch3_cntrl1: CH3_CNTRL1,
    #[doc = "0x3c - Timer Channel Control1 Register"]
    pub ch4_cntrl1: CH4_CNTRL1,
    #[doc = "0x40 - Timer Channel DTG Register"]
    pub ch1_dtg: CH1_DTG,
    #[doc = "0x44 - Timer Channel DTG Register"]
    pub ch2_dtg: CH2_DTG,
    #[doc = "0x48 - Timer Channel DTG Register"]
    pub ch3_dtg: CH3_DTG,
    #[doc = "0x4c - Timer Channel DTG Register"]
    pub ch4_dtg: CH4_DTG,
    #[doc = "0x50 - Timer BRK/ETR Control Register"]
    pub brketr_cntrl: BRKETR_CNTRL,
    #[doc = "0x54 - Timer Status Register"]
    pub status: STATUS,
    #[doc = "0x58 - Timer Interrupt Enable Register"]
    pub ie: IE,
    #[doc = "0x5c - Timer DMA Request Enable Register"]
    pub dma_re: DMA_RE,
    #[doc = "0x60 - Timer Channel Control2 Register"]
    pub ch1_cntrl2: CH1_CNTRL2,
    #[doc = "0x64 - Timer Channel Control2 Register"]
    pub ch2_cntrl2: CH2_CNTRL2,
    #[doc = "0x68 - Timer Channel Control2 Register"]
    pub ch3_cntrl2: CH3_CNTRL2,
    #[doc = "0x6c - Timer Channel Control2 Register"]
    pub ch4_cntrl2: CH4_CNTRL2,
    #[doc = "0x70 - Timer Capture/Compare1 Register"]
    pub ccr11: CCR11,
    #[doc = "0x74 - Timer Capture/Compare1 Register"]
    pub ccr21: CCR21,
    #[doc = "0x78 - Timer Capture/Compare1 Register"]
    pub ccr31: CCR31,
    #[doc = "0x7c - Timer Capture/Compare1 Register"]
    pub ccr41: CCR41,
}
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Timer Counter Register"]
pub mod cnt;
#[doc = "PSG (rw) register accessor: an alias for `Reg<PSG_SPEC>`"]
pub type PSG = crate::Reg<psg::PSG_SPEC>;
#[doc = "Timer Clock Prescaler Register"]
pub mod psg;
#[doc = "ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "Timer Auto-Reload Register"]
pub mod arr;
#[doc = "CNTRL (rw) register accessor: an alias for `Reg<CNTRL_SPEC>`"]
pub type CNTRL = crate::Reg<cntrl::CNTRL_SPEC>;
#[doc = "Timer Control Register"]
pub mod cntrl;
#[doc = "CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "Timer Capture/Compare Register"]
pub mod ccr1;
pub use ccr1 as ccr2;
pub use ccr1 as ccr3;
pub use ccr1 as ccr4;
pub use CCR1 as CCR2;
pub use CCR1 as CCR3;
pub use CCR1 as CCR4;
#[doc = "CH1_CNTRL (rw) register accessor: an alias for `Reg<CH1_CNTRL_SPEC>`"]
pub type CH1_CNTRL = crate::Reg<ch1_cntrl::CH1_CNTRL_SPEC>;
#[doc = "Timer Channel Control Register"]
pub mod ch1_cntrl;
pub use ch1_cntrl as ch2_cntrl;
pub use ch1_cntrl as ch3_cntrl;
pub use ch1_cntrl as ch4_cntrl;
pub use CH1_CNTRL as CH2_CNTRL;
pub use CH1_CNTRL as CH3_CNTRL;
pub use CH1_CNTRL as CH4_CNTRL;
#[doc = "CH1_CNTRL1 (rw) register accessor: an alias for `Reg<CH1_CNTRL1_SPEC>`"]
pub type CH1_CNTRL1 = crate::Reg<ch1_cntrl1::CH1_CNTRL1_SPEC>;
#[doc = "Timer Channel Control1 Register"]
pub mod ch1_cntrl1;
pub use ch1_cntrl1 as ch2_cntrl1;
pub use ch1_cntrl1 as ch3_cntrl1;
pub use ch1_cntrl1 as ch4_cntrl1;
pub use CH1_CNTRL1 as CH2_CNTRL1;
pub use CH1_CNTRL1 as CH3_CNTRL1;
pub use CH1_CNTRL1 as CH4_CNTRL1;
#[doc = "CH1_DTG (rw) register accessor: an alias for `Reg<CH1_DTG_SPEC>`"]
pub type CH1_DTG = crate::Reg<ch1_dtg::CH1_DTG_SPEC>;
#[doc = "Timer Channel DTG Register"]
pub mod ch1_dtg;
pub use ch1_dtg as ch2_dtg;
pub use ch1_dtg as ch3_dtg;
pub use ch1_dtg as ch4_dtg;
pub use CH1_DTG as CH2_DTG;
pub use CH1_DTG as CH3_DTG;
pub use CH1_DTG as CH4_DTG;
#[doc = "BRKETR_CNTRL (rw) register accessor: an alias for `Reg<BRKETR_CNTRL_SPEC>`"]
pub type BRKETR_CNTRL = crate::Reg<brketr_cntrl::BRKETR_CNTRL_SPEC>;
#[doc = "Timer BRK/ETR Control Register"]
pub mod brketr_cntrl;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Timer Status Register"]
pub mod status;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Timer Interrupt Enable Register"]
pub mod ie;
#[doc = "DMA_RE (rw) register accessor: an alias for `Reg<DMA_RE_SPEC>`"]
pub type DMA_RE = crate::Reg<dma_re::DMA_RE_SPEC>;
#[doc = "Timer DMA Request Enable Register"]
pub mod dma_re;
#[doc = "CH1_CNTRL2 (rw) register accessor: an alias for `Reg<CH1_CNTRL2_SPEC>`"]
pub type CH1_CNTRL2 = crate::Reg<ch1_cntrl2::CH1_CNTRL2_SPEC>;
#[doc = "Timer Channel Control2 Register"]
pub mod ch1_cntrl2;
pub use ch1_cntrl2 as ch2_cntrl2;
pub use ch1_cntrl2 as ch3_cntrl2;
pub use ch1_cntrl2 as ch4_cntrl2;
pub use CH1_CNTRL2 as CH2_CNTRL2;
pub use CH1_CNTRL2 as CH3_CNTRL2;
pub use CH1_CNTRL2 as CH4_CNTRL2;
#[doc = "CCR11 (rw) register accessor: an alias for `Reg<CCR11_SPEC>`"]
pub type CCR11 = crate::Reg<ccr11::CCR11_SPEC>;
#[doc = "Timer Capture/Compare1 Register"]
pub mod ccr11;
pub use ccr11 as ccr21;
pub use ccr11 as ccr31;
pub use ccr11 as ccr41;
pub use CCR11 as CCR21;
pub use CCR11 as CCR31;
pub use CCR11 as CCR41;
