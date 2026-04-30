use brain_proto::brain::brain_service_server::BrainService;
use brain_proto::brain::{ContextRequest, ContextResponse};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct MyBrainService {}

#[tonic::async_trait]
impl BrainService for MyBrainService {
    async fn push_context(
        &self,
        request: Request<ContextRequest>,
    ) -> Result<Response<ContextResponse>, Status> {
        let r = request.into_inner();
        println!("Received context from {}: {}", r.user_id, r.message);

        Ok(Response::new(ContextResponse {
            accepted: true,
            request_id: "mock-uuid".to_string(),
        }))
    }
}
