use crate::{
    err::TransactionProcessingError,
    proto::{
        self,
        capacity_server::{Capacity, CapacityServer},
        transaction_processing_server::{TransactionProcessing, TransactionProcessingServer},
        BlockhashResponse, CapacityRequest, CapacityStatus, Empty, RentRequest, RentResponse,
        SendRequest, Status, TransactionResult,
    },
    safe_divide_as_f32, Processor,
};