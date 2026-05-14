pub use vtuber_contracts::vtuber;

pub mod brain {
    pub mod v1 {
        // Use local definition until merged into vtuber-contracts (Issue #9)
        tonic::include_proto!("echo.vtuber.brain.v1");
    }
}

pub mod director {
    pub mod v1 {
        // Mapping to what's actually in vtuber-contracts v0.1.0
        pub use vtuber_contracts::vtuber::v1::{
            director_service_client::DirectorServiceClient,
            director_service_server::{DirectorService, DirectorServiceServer},
            EmitDirectiveRequest, EmitDirectiveResponse,
        };
    }
}
