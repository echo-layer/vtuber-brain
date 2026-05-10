use brain_core::LoreManager;
use brain_proto::brain::v1::brain_service_server::BrainService;
use brain_proto::brain::v1::{PushContextRequest, PushContextResponse};
use tonic::{Request, Response, Status};

pub struct MyBrainService {
    pub lore: LoreManager,
}

impl Default for MyBrainService {
    fn default() -> Self {
        Self {
            lore: LoreManager::new(),
        }
    }
}

#[tonic::async_trait]
impl BrainService for MyBrainService {
    async fn push_context(
        &self,
        request: Request<PushContextRequest>,
    ) -> Result<Response<PushContextResponse>, Status> {
        let r = request.into_inner();
        tracing::info!("Reasoning with persona lore: {}", self.lore.get_lore());
        tracing::info!("Received context from {}: {}", r.user_id, r.message);

        Ok(Response::new(PushContextResponse {
            accepted: true,
            request_id: "mock-uuid".to_string(),
        }))
    }
}
