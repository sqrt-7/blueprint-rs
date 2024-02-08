use crate::{
    logic,
    proto::{self, blueprint_server},
    toolbox::context,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct BlueprintServerImpl {
    logic: Arc<logic::Logic>,
}

impl BlueprintServerImpl {
    pub fn new(logic: Arc<logic::Logic>) -> Self {
        BlueprintServerImpl {
            logic,
        }
    }
}

#[rustfmt::skip]
impl BlueprintServerImpl {}

// async_trait macro is a workaround until native `async trait` becomes stable
#[rustfmt::skip]
#[tonic::async_trait]
impl blueprint_server::Blueprint for BlueprintServerImpl {
    async fn create_user(&self, request: Request<proto::CreateUserRequest>) -> Result<Response<proto::User>, Status> {
        let request = request.into_inner();
        let tid = uuid::Uuid::new_v4().to_string(); // todo
        let ctx = context::Context::new();
        ctx.store("trace_id", tid);
        
        let req = logic::dto::CreateUserRequest {
            email: request.email,
            name: request.name,
        };

        match self.logic.create_user(&ctx, req).await {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    async fn get_user(&self, request: Request<String>) -> Result<Response<proto::User>, Status> {
        let request = request.into_inner();
        let tid = uuid::Uuid::new_v4().to_string(); // todo
        let ctx = context::Context::new();
        ctx.store("trace_id", tid);

        match self.logic.get_user(&ctx, &request).await {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    async fn create_journal(&self, request: Request<proto::CreateJournalRequest>) -> Result<Response<proto::Journal>, Status> {
        let request = request.into_inner();
        let tid = uuid::Uuid::new_v4().to_string(); // todo
        let ctx = context::Context::new();
        ctx.store("trace_id", tid);
        
        let req = logic::dto::CreateJournalRequest{ 
            title: request.title, 
            year: request.year,
         };

        match self.logic.create_journal(&ctx, req).await {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    async fn get_journal(&self, request: Request<String>) -> Result<Response<proto::Journal>, Status> {
        let request = request.into_inner();
        let tid = uuid::Uuid::new_v4().to_string(); // todo
        let ctx = context::Context::new();
        ctx.store("trace_id", tid);
        
        match self.logic.get_journal(&ctx, &request).await {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    async fn create_subscription(&self, request: Request<proto::CreateSubscriptionRequest>) -> Result<Response<proto::Subscription>, Status> {
        let request = request.into_inner();
        let tid = uuid::Uuid::new_v4().to_string(); // todo
        let ctx = context::Context::new();
        ctx.store("trace_id", tid);
        
        let req = logic::dto::CreateSubscriptionRequest{
            user_id: request.user_id,
            journal_id: request.journal_id,
        };

        match self.logic.create_subscription(&ctx, req).await {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    async fn list_subscriptions_for_user(&self, request: Request<String>) -> Result<Response<proto::SubscriptionList>, Status> {
        let request = request.into_inner();
        let tid = uuid::Uuid::new_v4().to_string(); // todo
        let ctx = context::Context::new();
        ctx.store("trace_id", tid);
        
        match self.logic.list_subscriptions_by_user(&ctx, &request).await {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }
}
