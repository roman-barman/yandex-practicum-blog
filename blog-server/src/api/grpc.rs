mod commands;
mod errors;
pub(crate) mod grpc_handlers;
mod results;

pub(crate) mod proto {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("blog_descriptor");
}

pub(crate) mod blog {
    tonic::include_proto!("blog");
}
