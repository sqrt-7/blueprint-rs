use crate::{
    logic,
    proto::{self, blueprint_server::Blueprint},
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct Handler {
    logic: Arc<logic::Controller>,
}

impl Handler {
    pub fn new(logic: Arc<logic::Controller>) -> Self {
        Handler {
            logic,
        }
    }
}

#[rustfmt::skip]
impl Handler {
    fn create_user_inner(&self, request: Request<proto::CreateUserRequest>) -> Result<Response<proto::User>, Status> {
        let request = request.into_inner();

        match self.logic.create_user(logic::dto::CreateUserRequest {
            email: request.email,
            name: request.name,
        }) {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    fn get_user_inner(&self, request: Request<String>) -> Result<Response<proto::User>, Status> {
        let request = request.into_inner();
        match self.logic.get_user(&request) {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    fn create_journal_inner(&self, request: Request<proto::CreateJournalRequest>) -> Result<Response<proto::Journal>, Status> {
        let request = request.into_inner();
        match self.logic.create_journal(logic::dto::CreateJournalRequest{ 
            title: request.title, 
            year: request.year,
         }) {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    fn get_journal_inner(&self, request: Request<String>) -> Result<Response<proto::Journal>, Status> {
        let request = request.into_inner();
        match self.logic.get_journal(&request) {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    fn create_subscription_inner(&self, request: Request<proto::CreateSubscriptionRequest>) -> Result<Response<proto::Subscription>, Status> {
        let request = request.into_inner();
        match self.logic.create_subscription(logic::dto::CreateSubscriptionRequest{
            user_id: request.user_id,
            journal_id: request.journal_id,
        }) {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }

    fn list_subscriptions_for_user_inner(&self, request: Request<String>) -> Result<Response<proto::SubscriptionList>, Status> {
        let request = request.into_inner();
        match self.logic.list_subscriptions_by_user(&request) {
            Ok(obj) => Ok(Response::new(obj.into())),
            Err(service_error) => Err(service_error.into()),
        }
    }
}

// async_trait macro is a workaround until native `async trait` becomes stable
// (it completely breaks my rust-analyzer so moved the actual code into *_inner functions)
#[rustfmt::skip]
#[tonic::async_trait]
impl Blueprint for Handler {
    async fn create_user(&self, request: Request<proto::CreateUserRequest>) -> Result<Response<proto::User>, Status> {
        self.create_user_inner(request)
    }

    async fn get_user(&self, request: Request<String>) -> Result<Response<proto::User>, Status> {
        self.get_user_inner(request)
    }

    async fn create_journal(&self, request: Request<proto::CreateJournalRequest>) -> Result<Response<proto::Journal>, Status> {
        self.create_journal_inner(request)
    }

    async fn get_journal(&self, request: Request<String>) -> Result<Response<proto::Journal>, Status> {
        self.get_journal_inner(request)
    }

    async fn create_subscription(&self, request: Request<proto::CreateSubscriptionRequest>) -> Result<Response<proto::Subscription>, Status> {
        self.create_subscription_inner(request)
    }

    async fn list_subscriptions_for_user(&self, request: Request<String>) -> Result<Response<proto::SubscriptionList>, Status> {
        self.list_subscriptions_for_user_inner(request)
    }
}
