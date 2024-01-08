use crate::{
    logic,
    proto::{self, blueprint_server},
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct BlueprintServerImpl {
    controller: Arc<logic::Controller>,
}

impl BlueprintServerImpl {
    pub fn new(controller: Arc<logic::Controller>) -> Self {
        BlueprintServerImpl {
            controller,
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

        match self.controller.create_user(logic::dto::CreateUserRequest {
            email: request.email,
            name: request.name,
        }) {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    async fn get_user(&self, request: Request<String>) -> Result<Response<proto::User>, Status> {
        let request = request.into_inner();
        match self.controller.get_user(&request) {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    async fn create_journal(&self, request: Request<proto::CreateJournalRequest>) -> Result<Response<proto::Journal>, Status> {
        let request = request.into_inner();
        match self.controller.create_journal(logic::dto::CreateJournalRequest{ 
            title: request.title, 
            year: request.year,
         }) {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    async fn get_journal(&self, request: Request<String>) -> Result<Response<proto::Journal>, Status> {
        let request = request.into_inner();
        match self.controller.get_journal(&request) {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    async fn create_subscription(&self, request: Request<proto::CreateSubscriptionRequest>) -> Result<Response<proto::Subscription>, Status> {
        let request = request.into_inner();
        match self.controller.create_subscription(logic::dto::CreateSubscriptionRequest{
            user_id: request.user_id,
            journal_id: request.journal_id,
        }) {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    async fn list_subscriptions_for_user(&self, request: Request<String>) -> Result<Response<proto::SubscriptionList>, Status> {
        let request = request.into_inner();
        match self.controller.list_subscriptions_by_user(&request) {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }
}
