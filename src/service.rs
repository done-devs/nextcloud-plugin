use done_provider::services::provider::provider_server::Provider;
use done_provider::services::provider::{Empty, Task, CountResponse, TaskResponse, List, ListResponse};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct NextcloudService {
	pub id: String,
	pub name: String,
	pub description: String,
	pub icon: String,
}

#[tonic::async_trait]
impl Provider for NextcloudService {
	async fn get_id(
		&self,
		_request: Request<Empty>,
	) -> Result<Response<String>, Status> {
		Ok(Response::new(self.id.clone()))
	}

	async fn get_name(
		&self,
		_request: Request<Empty>,
	) -> Result<Response<String>, Status> {
		Ok(Response::new(self.name.clone()))
	}

	async fn get_description(
		&self,
		_request: Request<Empty>,
	) -> Result<Response<String>, Status> {
		Ok(Response::new(self.description.clone()))
	}

	async fn get_icon_name(
		&self,
		_request: Request<Empty>,
	) -> Result<Response<String>, Status> {
		Ok(Response::new(self.icon.clone()))
	}

	async fn read_task_count_from_list(
		&self,
		_request: Request<String>,
	) -> Result<Response<CountResponse>, Status> {
		todo!()
	}

	type ReadAllTasksStream = ReceiverStream<Result<TaskResponse, Status>>;

	async fn read_all_tasks(&self, __request: Request<Empty>) -> Result<Response<Self::ReadAllTasksStream>, Status> {
		todo!()
	}

	type ReadTasksFromListStream = ReceiverStream<Result<TaskResponse, Status>>;

	async fn read_tasks_from_list(
		&self,
		_request: Request<String>,
	) -> Result<Response<Self::ReadTasksFromListStream>, Status> {
		todo!()
	}

	async fn create_task(
		&self,
		_request: Request<Task>,
	) -> Result<Response<TaskResponse>, Status> {
		todo!()
	}

	async fn read_task(
		&self,
		_request: Request<String>,
	) -> Result<Response<TaskResponse>, Status> {
		todo!()
	}

	async fn update_task(
		&self,
		_request: Request<Task>,
	) -> Result<Response<TaskResponse>, Status> {
		todo!()
	}

	async fn delete_task(
		&self,
		_request: Request<String>,
	) -> Result<Response<TaskResponse>, Status> {
		todo!()
	}

	type ReadAllListsStream = ReceiverStream<Result<ListResponse, Status>>;

	async fn read_all_lists(
		&self,
		__request: Request<Empty>,
	) -> Result<Response<Self::ReadAllListsStream>, Status>{
		todo!()
	}

	async fn create_list(
		&self,
		_request: Request<List>,
	) -> Result<Response<ListResponse>, Status> {
		todo!()
	}

	async fn read_list(
		&self,
		_request: Request<String>,
	) -> Result<Response<ListResponse>, Status> {
		todo!()
	}

	async fn update_list(
		&self,
		_request: Request<List>,
	) -> Result<Response<ListResponse>, Status> {
		todo!()
	}

	async fn delete_list(
		&self,
		_request: Request<String>,
	) -> Result<Response<ListResponse>, Status> {
		todo!()
	}
}
