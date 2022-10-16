

pub mod kaspad_request_convert {
    use crate::protowire::*;
    use crate::protowire::kaspad_request::*;

    impl_into_kaspad_request!(GetBlockRequestMessage, Payload::GetBlockRequest);

    macro_rules! impl_into_kaspad_request {
        ($($struct_path:ident)::+, $($variant:ident)::+) => {
            impl From<$($struct_path)::+> for KaspadRequest {
                fn from(item: $($struct_path)::+) -> Self {
                    Self { payload: Some($($variant)::+(item)) }
                }
            }
        };
    }
    use impl_into_kaspad_request;
}