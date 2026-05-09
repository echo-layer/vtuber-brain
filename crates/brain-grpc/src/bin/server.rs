use brain_grpc::server::MyBrainService;
use brain_proto::brain::v1::brain_service_server::BrainServiceServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let brain_service = MyBrainService::default();

    println!("Brain server listening on {}", addr);

    Server::builder()
        .add_service(BrainServiceServer::new(brain_service))
        .serve(addr)
        .await?;

    Ok(())
}
