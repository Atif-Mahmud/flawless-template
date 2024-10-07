use hello::{start_job, Job};

#[tokio::main]
async fn main() {
    let flawless = flawless_utils::Server::new("http://localhost:27288", None);
    let flawless_module = flawless_utils::load_module_from_build!("hello");
    let module = flawless.deploy(flawless_module).await.unwrap();

    module
        .start::<start_job>(Job {
            name: String::from("My Job"),
        })
        .await
        .unwrap();
}
