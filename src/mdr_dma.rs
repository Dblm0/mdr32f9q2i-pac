#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Status Register"]
    pub status: STATUS,
    #[doc = "0x04 - DMA Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x08 - "]
    pub ctrl_base_ptr: CTRL_BASE_PTR,
    #[doc = "0x0c - "]
    pub alt_ctrl_base_ptr: ALT_CTRL_BASE_PTR,
    #[doc = "0x10 - "]
    pub waitonreq_status: WAITONREQ_STATUS,
    #[doc = "0x14 - "]
    pub chnl_sw_request: CHNL_SW_REQUEST,
    #[doc = "0x18 - "]
    pub chnl_useburst_set: CHNL_USEBURST_SET,
    #[doc = "0x1c - "]
    pub chnl_useburst_clr: CHNL_USEBURST_CLR,
    #[doc = "0x20 - "]
    pub chnl_req_mask_set: CHNL_REQ_MASK_SET,
    #[doc = "0x24 - "]
    pub chnl_req_mask_clr: CHNL_REQ_MASK_CLR,
    #[doc = "0x28 - "]
    pub chnl_enable_set: CHNL_ENABLE_SET,
    #[doc = "0x2c - "]
    pub chnl_enable_clr: CHNL_ENABLE_CLR,
    #[doc = "0x30 - "]
    pub chnl_pri_alt_set: CHNL_PRI_ALT_SET,
    #[doc = "0x34 - "]
    pub chnl_pri_alt_clr: CHNL_PRI_ALT_CLR,
    #[doc = "0x38 - "]
    pub chnl_priority_set: CHNL_PRIORITY_SET,
    #[doc = "0x3c - "]
    pub chnl_priority_clr: CHNL_PRIORITY_CLR,
    _reserved16: [u8; 0x0c],
    #[doc = "0x4c - "]
    pub err_clr: ERR_CLR,
}
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DMA Status Register"]
pub mod status;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod cfg;
#[doc = "CTRL_BASE_PTR (rw) register accessor: an alias for `Reg<CTRL_BASE_PTR_SPEC>`"]
pub type CTRL_BASE_PTR = crate::Reg<ctrl_base_ptr::CTRL_BASE_PTR_SPEC>;
#[doc = ""]
pub mod ctrl_base_ptr;
#[doc = "ALT_CTRL_BASE_PTR (rw) register accessor: an alias for `Reg<ALT_CTRL_BASE_PTR_SPEC>`"]
pub type ALT_CTRL_BASE_PTR = crate::Reg<alt_ctrl_base_ptr::ALT_CTRL_BASE_PTR_SPEC>;
#[doc = ""]
pub mod alt_ctrl_base_ptr;
#[doc = "WAITONREQ_STATUS (rw) register accessor: an alias for `Reg<WAITONREQ_STATUS_SPEC>`"]
pub type WAITONREQ_STATUS = crate::Reg<waitonreq_status::WAITONREQ_STATUS_SPEC>;
#[doc = ""]
pub mod waitonreq_status;
#[doc = "CHNL_SW_REQUEST (rw) register accessor: an alias for `Reg<CHNL_SW_REQUEST_SPEC>`"]
pub type CHNL_SW_REQUEST = crate::Reg<chnl_sw_request::CHNL_SW_REQUEST_SPEC>;
#[doc = ""]
pub mod chnl_sw_request;
#[doc = "CHNL_USEBURST_SET (rw) register accessor: an alias for `Reg<CHNL_USEBURST_SET_SPEC>`"]
pub type CHNL_USEBURST_SET = crate::Reg<chnl_useburst_set::CHNL_USEBURST_SET_SPEC>;
#[doc = ""]
pub mod chnl_useburst_set;
#[doc = "CHNL_USEBURST_CLR (rw) register accessor: an alias for `Reg<CHNL_USEBURST_CLR_SPEC>`"]
pub type CHNL_USEBURST_CLR = crate::Reg<chnl_useburst_clr::CHNL_USEBURST_CLR_SPEC>;
#[doc = ""]
pub mod chnl_useburst_clr;
#[doc = "CHNL_REQ_MASK_SET (rw) register accessor: an alias for `Reg<CHNL_REQ_MASK_SET_SPEC>`"]
pub type CHNL_REQ_MASK_SET = crate::Reg<chnl_req_mask_set::CHNL_REQ_MASK_SET_SPEC>;
#[doc = ""]
pub mod chnl_req_mask_set;
#[doc = "CHNL_REQ_MASK_CLR (rw) register accessor: an alias for `Reg<CHNL_REQ_MASK_CLR_SPEC>`"]
pub type CHNL_REQ_MASK_CLR = crate::Reg<chnl_req_mask_clr::CHNL_REQ_MASK_CLR_SPEC>;
#[doc = ""]
pub mod chnl_req_mask_clr;
#[doc = "CHNL_ENABLE_SET (rw) register accessor: an alias for `Reg<CHNL_ENABLE_SET_SPEC>`"]
pub type CHNL_ENABLE_SET = crate::Reg<chnl_enable_set::CHNL_ENABLE_SET_SPEC>;
#[doc = ""]
pub mod chnl_enable_set;
#[doc = "CHNL_ENABLE_CLR (rw) register accessor: an alias for `Reg<CHNL_ENABLE_CLR_SPEC>`"]
pub type CHNL_ENABLE_CLR = crate::Reg<chnl_enable_clr::CHNL_ENABLE_CLR_SPEC>;
#[doc = ""]
pub mod chnl_enable_clr;
#[doc = "CHNL_PRI_ALT_SET (rw) register accessor: an alias for `Reg<CHNL_PRI_ALT_SET_SPEC>`"]
pub type CHNL_PRI_ALT_SET = crate::Reg<chnl_pri_alt_set::CHNL_PRI_ALT_SET_SPEC>;
#[doc = ""]
pub mod chnl_pri_alt_set;
#[doc = "CHNL_PRI_ALT_CLR (rw) register accessor: an alias for `Reg<CHNL_PRI_ALT_CLR_SPEC>`"]
pub type CHNL_PRI_ALT_CLR = crate::Reg<chnl_pri_alt_clr::CHNL_PRI_ALT_CLR_SPEC>;
#[doc = ""]
pub mod chnl_pri_alt_clr;
#[doc = "CHNL_PRIORITY_SET (rw) register accessor: an alias for `Reg<CHNL_PRIORITY_SET_SPEC>`"]
pub type CHNL_PRIORITY_SET = crate::Reg<chnl_priority_set::CHNL_PRIORITY_SET_SPEC>;
#[doc = ""]
pub mod chnl_priority_set;
#[doc = "CHNL_PRIORITY_CLR (rw) register accessor: an alias for `Reg<CHNL_PRIORITY_CLR_SPEC>`"]
pub type CHNL_PRIORITY_CLR = crate::Reg<chnl_priority_clr::CHNL_PRIORITY_CLR_SPEC>;
#[doc = ""]
pub mod chnl_priority_clr;
#[doc = "ERR_CLR (rw) register accessor: an alias for `Reg<ERR_CLR_SPEC>`"]
pub type ERR_CLR = crate::Reg<err_clr::ERR_CLR_SPEC>;
#[doc = ""]
pub mod err_clr;
