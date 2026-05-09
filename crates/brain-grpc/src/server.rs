use brain_proto::brain::v1::brain_service_server::BrainService;
use brain_proto::brain::v1::{PushContextRequest, PushContextResponse};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct MyBrainService {}

#[tonic::async_trait]
impl BrainService for MyBrainService {
    async fn push_context(
        &self,
        request: Request<PushContextRequest>,
    ) -> Result<Response<PushContextResponse>, Status> {
        let r = request.into_inner();
        tracing::info!("Received context from {}: {}", r.user_id, r.message);

        Ok(Response::new(PushContextResponse {
            accepted: true,
            request_id: "mock-uuid".to_string(),
        }))
    }
}
