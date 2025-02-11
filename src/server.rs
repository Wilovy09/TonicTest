use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest, ByeRequest, ByeResponse};

// Incluimos el archivo `.proto`
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

// Creamos una implementación para nuestras funciones del gRPC
#[tonic::async_trait]
// Greeter es lo que tenemos en el `.proto`
impl Greeter for MyGreeter {
    // rpc SayHello -> say_hello
    async fn say_hello(
        &self,
        // El request es de tipo:
        request: Request<HelloRequest>,
        // El return es de tipo:
    ) -> Result<Response<HelloReply>, Status> {
        // minilog
        println!("Got a request: {:?}", request);

        // Creamos el response
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        // Regresamos la respuesta
        Ok(Response::new(reply))
    }

    async fn say_goodbye(
        &self,
        request: Request<ByeRequest>,
    ) -> Result<Response<ByeResponse>, Status> {
        println!("Got a bye request: {:?}", request);

        let reply = ByeResponse {
            message: format!("Goodbye {}", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Asignamos un puerto al localhost
    let addr = "[::1]:50051".parse()?;
    // Creamos una "instancia" de la estructura de MyGreeter
    let greeter = MyGreeter::default();

    Server::builder()
        // Agregamos el servicio
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
