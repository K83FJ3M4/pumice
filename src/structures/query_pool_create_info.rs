use crate::{QueryPipelineStatisticFlags, QueryPoolCreateFlags, QueryType, StructureType};
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone, Copy)]
pub struct QueryPoolCreateInfo {
    pub query_type: QueryType,
    pub query_count: u32,
    pub pipeline_statistics: QueryPipelineStatisticFlags
}

#[repr(C)]
pub(crate) struct RawQueryPoolCreateInfo {
    pub(crate) s_type: StructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: QueryPoolCreateFlags,
    pub(crate) query_type: QueryType,
    pub(crate) query_count: u32,
    pub(crate) pipeline_statistics: QueryPipelineStatisticFlags
}

impl QueryPoolCreateInfo {
    pub(crate) fn into_raw(&self) -> RawQueryPoolCreateInfo {
        RawQueryPoolCreateInfo {
            s_type: StructureType::QueryPoolCreateInfo,
            p_next: null(),
            flags: QueryPoolCreateFlags::empty(),
            query_type: self.query_type,
            query_count: self.query_count,
            pipeline_statistics: self.pipeline_statistics
        }
    }
}