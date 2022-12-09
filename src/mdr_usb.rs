#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB HTXC Register"]
    pub htxc: HTXC,
    #[doc = "0x04 - USB HTXT Register"]
    pub htxt: HTXT,
    #[doc = "0x08 - USB HTXLC Register"]
    pub htxlc: HTXLC,
    #[doc = "0x0c - "]
    pub htxse: HTXSE,
    #[doc = "0x10 - "]
    pub htxa: HTXA,
    #[doc = "0x14 - "]
    pub htxe: HTXE,
    #[doc = "0x18 - "]
    pub hfn_l: HFN_L,
    #[doc = "0x1c - "]
    pub hfn_h: HFN_H,
    #[doc = "0x20 - USB_HIS Register"]
    pub his: HIS,
    #[doc = "0x24 - USB_HIM Register"]
    pub him: HIM,
    #[doc = "0x28 - USB_HRXS Register"]
    pub hrxs: HRXS,
    #[doc = "0x2c - "]
    pub hrxp: HRXP,
    #[doc = "0x30 - "]
    pub hrxa: HRXA,
    #[doc = "0x34 - "]
    pub hrxe: HRXE,
    #[doc = "0x38 - "]
    pub hrxcs: HRXCS,
    #[doc = "0x3c - "]
    pub hstm: HSTM,
    _reserved16: [u8; 0x40],
    #[doc = "0x80 - "]
    pub hrxfd: HRXFD,
    _reserved17: [u8; 0x04],
    #[doc = "0x88 - "]
    pub hrxfdc_l: HRXFDC_L,
    #[doc = "0x8c - "]
    pub hrxfdc_h: HRXFDC_H,
    #[doc = "0x90 - "]
    pub hrxfc: HRXFC,
    _reserved20: [u8; 0x2c],
    #[doc = "0xc0 - "]
    pub htxfd: HTXFD,
    _reserved21: [u8; 0x0c],
    #[doc = "0xd0 - "]
    pub htxfc: HTXFC,
    _reserved22: [u8; 0x2c],
    #[doc = "0x100 - USB_SEP Control Register"]
    pub sep0_ctrl: SEP0_CTRL,
    #[doc = "0x104 - USB_SEP Status Register"]
    pub sep0_sts: SEP0_STS,
    #[doc = "0x108 - "]
    pub sep0_ts: SEP0_TS,
    #[doc = "0x10c - "]
    pub sep0_nts: SEP0_NTS,
    #[doc = "0x110 - USB_SEP Control Register"]
    pub sep1_ctrl: SEP1_CTRL,
    #[doc = "0x114 - USB_SEP Status Register"]
    pub sep1_sts: SEP1_STS,
    #[doc = "0x118 - "]
    pub sep1_ts: SEP1_TS,
    #[doc = "0x11c - "]
    pub sep1_nts: SEP1_NTS,
    #[doc = "0x120 - USB_SEP Control Register"]
    pub sep2_ctrl: SEP2_CTRL,
    #[doc = "0x124 - USB_SEP Status Register"]
    pub sep2_sts: SEP2_STS,
    #[doc = "0x128 - "]
    pub sep2_ts: SEP2_TS,
    #[doc = "0x12c - "]
    pub sep2_nts: SEP2_NTS,
    #[doc = "0x130 - USB_SEP Control Register"]
    pub sep3_ctrl: SEP3_CTRL,
    #[doc = "0x134 - USB_SEP Status Register"]
    pub sep3_sts: SEP3_STS,
    #[doc = "0x138 - "]
    pub sep3_ts: SEP3_TS,
    #[doc = "0x13c - "]
    pub sep3_nts: SEP3_NTS,
    #[doc = "0x140 - USB_SC Register"]
    pub sc: SC,
    #[doc = "0x144 - "]
    pub sls: SLS,
    #[doc = "0x148 - USB_SIS Register"]
    pub sis: SIS,
    #[doc = "0x14c - USB_SIM Register"]
    pub sim: SIM,
    #[doc = "0x150 - "]
    pub sa: SA,
    #[doc = "0x154 - "]
    pub sfn_l: SFN_L,
    #[doc = "0x158 - "]
    pub sfn_h: SFN_H,
    _reserved45: [u8; 0x24],
    #[doc = "0x180 - "]
    pub sep_fifo0_rxfd: SEP_FIFO0_RXFD,
    _reserved46: [u8; 0x04],
    #[doc = "0x188 - "]
    pub sep_fifo0_rxfdc_l: SEP_FIFO0_RXFDC_L,
    #[doc = "0x18c - "]
    pub sep_fifo0_rxfdc_h: SEP_FIFO0_RXFDC_H,
    #[doc = "0x190 - "]
    pub sep_fifo0_rxfc: SEP_FIFO0_RXFC,
    _reserved49: [u8; 0x04],
    #[doc = "0x198 - "]
    pub sep_fifo1_rxfd: SEP_FIFO1_RXFD,
    _reserved50: [u8; 0x04],
    #[doc = "0x1a0 - "]
    pub sep_fifo1_rxfdc_l: SEP_FIFO1_RXFDC_L,
    #[doc = "0x1a4 - "]
    pub sep_fifo1_rxfdc_h: SEP_FIFO1_RXFDC_H,
    #[doc = "0x1a8 - "]
    pub sep_fifo1_rxfc: SEP_FIFO1_RXFC,
    _reserved53: [u8; 0x04],
    #[doc = "0x1b0 - "]
    pub sep_fifo2_rxfd: SEP_FIFO2_RXFD,
    _reserved54: [u8; 0x04],
    #[doc = "0x1b8 - "]
    pub sep_fifo2_rxfdc_l: SEP_FIFO2_RXFDC_L,
    #[doc = "0x1bc - "]
    pub sep_fifo2_rxfdc_h: SEP_FIFO2_RXFDC_H,
    #[doc = "0x1c0 - "]
    pub sep_fifo0_txfd: SEP_FIFO0_TXFD,
    _reserved57: [u8; 0x04],
    #[doc = "0x1c8 - "]
    pub sep_fifo3_rxfd: SEP_FIFO3_RXFD,
    _reserved58: [u8; 0x04],
    #[doc = "0x1d0 - "]
    pub sep_fifo0_txfdc: SEP_FIFO0_TXFDC,
    #[doc = "0x1d4 - "]
    pub sep_fifo3_rxfdc_h: SEP_FIFO3_RXFDC_H,
    #[doc = "0x1d8 - "]
    pub sep_fifo1_txfd: SEP_FIFO1_TXFD,
    _reserved61: [u8; 0x0c],
    #[doc = "0x1e8 - "]
    pub sep_fifo1_txfdc: SEP_FIFO1_TXFDC,
    _reserved62: [u8; 0x04],
    #[doc = "0x1f0 - "]
    pub sep_fifo2_txfd: SEP_FIFO2_TXFD,
    _reserved63: [u8; 0x0c],
    #[doc = "0x200 - "]
    pub sep_fifo2_txfdc: SEP_FIFO2_TXFDC,
    _reserved64: [u8; 0x04],
    #[doc = "0x208 - "]
    pub sep_fifo3_txfd: SEP_FIFO3_TXFD,
    _reserved65: [u8; 0x0c],
    #[doc = "0x218 - "]
    pub sep_fifo3_txfdc: SEP_FIFO3_TXFDC,
    _reserved66: [u8; 0x74],
    #[doc = "0x290 - "]
    pub sep_fifo2_rxfc: SEP_FIFO2_RXFC,
    _reserved67: [u8; 0x74],
    #[doc = "0x308 - "]
    pub sep_fifo3_rxfdc_l: SEP_FIFO3_RXFDC_L,
    _reserved68: [u8; 0x04],
    #[doc = "0x310 - "]
    pub sep_fifo3_rxfc: SEP_FIFO3_RXFC,
    _reserved69: [u8; 0x6c],
    #[doc = "0x380 - USB_HSCR Register"]
    pub hscr: HSCR,
    #[doc = "0x384 - USB_HSVR Register"]
    pub hsvr: HSVR,
}
#[doc = "HTXC (rw) register accessor: an alias for `Reg<HTXC_SPEC>`"]
pub type HTXC = crate::Reg<htxc::HTXC_SPEC>;
#[doc = "USB HTXC Register"]
pub mod htxc;
#[doc = "HTXT (rw) register accessor: an alias for `Reg<HTXT_SPEC>`"]
pub type HTXT = crate::Reg<htxt::HTXT_SPEC>;
#[doc = "USB HTXT Register"]
pub mod htxt;
#[doc = "HTXLC (rw) register accessor: an alias for `Reg<HTXLC_SPEC>`"]
pub type HTXLC = crate::Reg<htxlc::HTXLC_SPEC>;
#[doc = "USB HTXLC Register"]
pub mod htxlc;
#[doc = "HTXSE (rw) register accessor: an alias for `Reg<HTXSE_SPEC>`"]
pub type HTXSE = crate::Reg<htxse::HTXSE_SPEC>;
#[doc = ""]
pub mod htxse;
#[doc = "HTXA (rw) register accessor: an alias for `Reg<HTXA_SPEC>`"]
pub type HTXA = crate::Reg<htxa::HTXA_SPEC>;
#[doc = ""]
pub mod htxa;
#[doc = "HTXE (rw) register accessor: an alias for `Reg<HTXE_SPEC>`"]
pub type HTXE = crate::Reg<htxe::HTXE_SPEC>;
#[doc = ""]
pub mod htxe;
#[doc = "HFN_L (rw) register accessor: an alias for `Reg<HFN_L_SPEC>`"]
pub type HFN_L = crate::Reg<hfn_l::HFN_L_SPEC>;
#[doc = ""]
pub mod hfn_l;
#[doc = "HFN_H (rw) register accessor: an alias for `Reg<HFN_H_SPEC>`"]
pub type HFN_H = crate::Reg<hfn_h::HFN_H_SPEC>;
#[doc = ""]
pub mod hfn_h;
#[doc = "HIS (rw) register accessor: an alias for `Reg<HIS_SPEC>`"]
pub type HIS = crate::Reg<his::HIS_SPEC>;
#[doc = "USB_HIS Register"]
pub mod his;
#[doc = "HIM (rw) register accessor: an alias for `Reg<HIM_SPEC>`"]
pub type HIM = crate::Reg<him::HIM_SPEC>;
#[doc = "USB_HIM Register"]
pub mod him;
#[doc = "HRXS (rw) register accessor: an alias for `Reg<HRXS_SPEC>`"]
pub type HRXS = crate::Reg<hrxs::HRXS_SPEC>;
#[doc = "USB_HRXS Register"]
pub mod hrxs;
#[doc = "HRXP (rw) register accessor: an alias for `Reg<HRXP_SPEC>`"]
pub type HRXP = crate::Reg<hrxp::HRXP_SPEC>;
#[doc = ""]
pub mod hrxp;
#[doc = "HRXA (rw) register accessor: an alias for `Reg<HRXA_SPEC>`"]
pub type HRXA = crate::Reg<hrxa::HRXA_SPEC>;
#[doc = ""]
pub mod hrxa;
#[doc = "HRXE (rw) register accessor: an alias for `Reg<HRXE_SPEC>`"]
pub type HRXE = crate::Reg<hrxe::HRXE_SPEC>;
#[doc = ""]
pub mod hrxe;
#[doc = "HRXCS (rw) register accessor: an alias for `Reg<HRXCS_SPEC>`"]
pub type HRXCS = crate::Reg<hrxcs::HRXCS_SPEC>;
#[doc = ""]
pub mod hrxcs;
#[doc = "HSTM (rw) register accessor: an alias for `Reg<HSTM_SPEC>`"]
pub type HSTM = crate::Reg<hstm::HSTM_SPEC>;
#[doc = ""]
pub mod hstm;
#[doc = "HRXFD (rw) register accessor: an alias for `Reg<HRXFD_SPEC>`"]
pub type HRXFD = crate::Reg<hrxfd::HRXFD_SPEC>;
#[doc = ""]
pub mod hrxfd;
#[doc = "HRXFDC_L (rw) register accessor: an alias for `Reg<HRXFDC_L_SPEC>`"]
pub type HRXFDC_L = crate::Reg<hrxfdc_l::HRXFDC_L_SPEC>;
#[doc = ""]
pub mod hrxfdc_l;
#[doc = "HRXFDC_H (rw) register accessor: an alias for `Reg<HRXFDC_H_SPEC>`"]
pub type HRXFDC_H = crate::Reg<hrxfdc_h::HRXFDC_H_SPEC>;
#[doc = ""]
pub mod hrxfdc_h;
#[doc = "HRXFC (rw) register accessor: an alias for `Reg<HRXFC_SPEC>`"]
pub type HRXFC = crate::Reg<hrxfc::HRXFC_SPEC>;
#[doc = ""]
pub mod hrxfc;
#[doc = "HTXFD (rw) register accessor: an alias for `Reg<HTXFD_SPEC>`"]
pub type HTXFD = crate::Reg<htxfd::HTXFD_SPEC>;
#[doc = ""]
pub mod htxfd;
#[doc = "HTXFC (rw) register accessor: an alias for `Reg<HTXFC_SPEC>`"]
pub type HTXFC = crate::Reg<htxfc::HTXFC_SPEC>;
#[doc = ""]
pub mod htxfc;
#[doc = "SEP0_CTRL (rw) register accessor: an alias for `Reg<SEP0_CTRL_SPEC>`"]
pub type SEP0_CTRL = crate::Reg<sep0_ctrl::SEP0_CTRL_SPEC>;
#[doc = "USB_SEP Control Register"]
pub mod sep0_ctrl;
#[doc = "SEP0_STS (rw) register accessor: an alias for `Reg<SEP0_STS_SPEC>`"]
pub type SEP0_STS = crate::Reg<sep0_sts::SEP0_STS_SPEC>;
#[doc = "USB_SEP Status Register"]
pub mod sep0_sts;
#[doc = "SEP0_TS (rw) register accessor: an alias for `Reg<SEP0_TS_SPEC>`"]
pub type SEP0_TS = crate::Reg<sep0_ts::SEP0_TS_SPEC>;
#[doc = ""]
pub mod sep0_ts;
#[doc = "SEP0_NTS (rw) register accessor: an alias for `Reg<SEP0_NTS_SPEC>`"]
pub type SEP0_NTS = crate::Reg<sep0_nts::SEP0_NTS_SPEC>;
#[doc = ""]
pub mod sep0_nts;
pub use sep0_ctrl as sep1_ctrl;
pub use sep0_ctrl as sep2_ctrl;
pub use sep0_ctrl as sep3_ctrl;
pub use sep0_nts as sep1_nts;
pub use sep0_nts as sep2_nts;
pub use sep0_nts as sep3_nts;
pub use sep0_sts as sep1_sts;
pub use sep0_sts as sep2_sts;
pub use sep0_sts as sep3_sts;
pub use sep0_ts as sep1_ts;
pub use sep0_ts as sep2_ts;
pub use sep0_ts as sep3_ts;
pub use SEP0_CTRL as SEP1_CTRL;
pub use SEP0_CTRL as SEP2_CTRL;
pub use SEP0_CTRL as SEP3_CTRL;
pub use SEP0_NTS as SEP1_NTS;
pub use SEP0_NTS as SEP2_NTS;
pub use SEP0_NTS as SEP3_NTS;
pub use SEP0_STS as SEP1_STS;
pub use SEP0_STS as SEP2_STS;
pub use SEP0_STS as SEP3_STS;
pub use SEP0_TS as SEP1_TS;
pub use SEP0_TS as SEP2_TS;
pub use SEP0_TS as SEP3_TS;
#[doc = "SC (rw) register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "USB_SC Register"]
pub mod sc;
#[doc = "SLS (rw) register accessor: an alias for `Reg<SLS_SPEC>`"]
pub type SLS = crate::Reg<sls::SLS_SPEC>;
#[doc = ""]
pub mod sls;
#[doc = "SIS (rw) register accessor: an alias for `Reg<SIS_SPEC>`"]
pub type SIS = crate::Reg<sis::SIS_SPEC>;
#[doc = "USB_SIS Register"]
pub mod sis;
#[doc = "SIM (rw) register accessor: an alias for `Reg<SIM_SPEC>`"]
pub type SIM = crate::Reg<sim::SIM_SPEC>;
#[doc = "USB_SIM Register"]
pub mod sim;
#[doc = "SA (rw) register accessor: an alias for `Reg<SA_SPEC>`"]
pub type SA = crate::Reg<sa::SA_SPEC>;
#[doc = ""]
pub mod sa;
#[doc = "SFN_L (rw) register accessor: an alias for `Reg<SFN_L_SPEC>`"]
pub type SFN_L = crate::Reg<sfn_l::SFN_L_SPEC>;
#[doc = ""]
pub mod sfn_l;
#[doc = "SFN_H (rw) register accessor: an alias for `Reg<SFN_H_SPEC>`"]
pub type SFN_H = crate::Reg<sfn_h::SFN_H_SPEC>;
#[doc = ""]
pub mod sfn_h;
#[doc = "SEP_FIFO0_RXFD (rw) register accessor: an alias for `Reg<SEP_FIFO0_RXFD_SPEC>`"]
pub type SEP_FIFO0_RXFD = crate::Reg<sep_fifo0_rxfd::SEP_FIFO0_RXFD_SPEC>;
#[doc = ""]
pub mod sep_fifo0_rxfd;
#[doc = "SEP_FIFO0_RXFDC_L (rw) register accessor: an alias for `Reg<SEP_FIFO0_RXFDC_L_SPEC>`"]
pub type SEP_FIFO0_RXFDC_L = crate::Reg<sep_fifo0_rxfdc_l::SEP_FIFO0_RXFDC_L_SPEC>;
#[doc = ""]
pub mod sep_fifo0_rxfdc_l;
#[doc = "SEP_FIFO0_RXFDC_H (rw) register accessor: an alias for `Reg<SEP_FIFO0_RXFDC_H_SPEC>`"]
pub type SEP_FIFO0_RXFDC_H = crate::Reg<sep_fifo0_rxfdc_h::SEP_FIFO0_RXFDC_H_SPEC>;
#[doc = ""]
pub mod sep_fifo0_rxfdc_h;
#[doc = "SEP_FIFO0_RXFC (rw) register accessor: an alias for `Reg<SEP_FIFO0_RXFC_SPEC>`"]
pub type SEP_FIFO0_RXFC = crate::Reg<sep_fifo0_rxfc::SEP_FIFO0_RXFC_SPEC>;
#[doc = ""]
pub mod sep_fifo0_rxfc;
#[doc = "SEP_FIFO0_TXFD (rw) register accessor: an alias for `Reg<SEP_FIFO0_TXFD_SPEC>`"]
pub type SEP_FIFO0_TXFD = crate::Reg<sep_fifo0_txfd::SEP_FIFO0_TXFD_SPEC>;
#[doc = ""]
pub mod sep_fifo0_txfd;
#[doc = "SEP_FIFO0_TXFDC (rw) register accessor: an alias for `Reg<SEP_FIFO0_TXFDC_SPEC>`"]
pub type SEP_FIFO0_TXFDC = crate::Reg<sep_fifo0_txfdc::SEP_FIFO0_TXFDC_SPEC>;
#[doc = ""]
pub mod sep_fifo0_txfdc;
pub use sep_fifo0_rxfc as sep_fifo1_rxfc;
pub use sep_fifo0_rxfc as sep_fifo2_rxfc;
pub use sep_fifo0_rxfc as sep_fifo3_rxfc;
pub use sep_fifo0_rxfd as sep_fifo1_rxfd;
pub use sep_fifo0_rxfd as sep_fifo2_rxfd;
pub use sep_fifo0_rxfd as sep_fifo3_rxfd;
pub use sep_fifo0_rxfdc_h as sep_fifo1_rxfdc_h;
pub use sep_fifo0_rxfdc_h as sep_fifo2_rxfdc_h;
pub use sep_fifo0_rxfdc_h as sep_fifo3_rxfdc_h;
pub use sep_fifo0_rxfdc_l as sep_fifo1_rxfdc_l;
pub use sep_fifo0_rxfdc_l as sep_fifo2_rxfdc_l;
pub use sep_fifo0_rxfdc_l as sep_fifo3_rxfdc_l;
pub use sep_fifo0_txfd as sep_fifo1_txfd;
pub use sep_fifo0_txfd as sep_fifo2_txfd;
pub use sep_fifo0_txfd as sep_fifo3_txfd;
pub use sep_fifo0_txfdc as sep_fifo1_txfdc;
pub use sep_fifo0_txfdc as sep_fifo2_txfdc;
pub use sep_fifo0_txfdc as sep_fifo3_txfdc;
pub use SEP_FIFO0_RXFC as SEP_FIFO1_RXFC;
pub use SEP_FIFO0_RXFC as SEP_FIFO2_RXFC;
pub use SEP_FIFO0_RXFC as SEP_FIFO3_RXFC;
pub use SEP_FIFO0_RXFD as SEP_FIFO1_RXFD;
pub use SEP_FIFO0_RXFD as SEP_FIFO2_RXFD;
pub use SEP_FIFO0_RXFD as SEP_FIFO3_RXFD;
pub use SEP_FIFO0_RXFDC_H as SEP_FIFO1_RXFDC_H;
pub use SEP_FIFO0_RXFDC_H as SEP_FIFO2_RXFDC_H;
pub use SEP_FIFO0_RXFDC_H as SEP_FIFO3_RXFDC_H;
pub use SEP_FIFO0_RXFDC_L as SEP_FIFO1_RXFDC_L;
pub use SEP_FIFO0_RXFDC_L as SEP_FIFO2_RXFDC_L;
pub use SEP_FIFO0_RXFDC_L as SEP_FIFO3_RXFDC_L;
pub use SEP_FIFO0_TXFD as SEP_FIFO1_TXFD;
pub use SEP_FIFO0_TXFD as SEP_FIFO2_TXFD;
pub use SEP_FIFO0_TXFD as SEP_FIFO3_TXFD;
pub use SEP_FIFO0_TXFDC as SEP_FIFO1_TXFDC;
pub use SEP_FIFO0_TXFDC as SEP_FIFO2_TXFDC;
pub use SEP_FIFO0_TXFDC as SEP_FIFO3_TXFDC;
#[doc = "HSCR (rw) register accessor: an alias for `Reg<HSCR_SPEC>`"]
pub type HSCR = crate::Reg<hscr::HSCR_SPEC>;
#[doc = "USB_HSCR Register"]
pub mod hscr;
#[doc = "HSVR (rw) register accessor: an alias for `Reg<HSVR_SPEC>`"]
pub type HSVR = crate::Reg<hsvr::HSVR_SPEC>;
#[doc = "USB_HSVR Register"]
pub mod hsvr;
