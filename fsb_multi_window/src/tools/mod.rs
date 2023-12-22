use std::sync::RwLock;

use once_cell::sync::Lazy;

static UUID: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));

pub mod run_app {
    tonic::include_proto!("runapp");
}

pub fn get_uuid() -> String {
    let r = UUID.read().unwrap();

    return (*r).clone();
}

pub fn set_uuid(s: String) {
    let mut r = UUID.write().unwrap();
    *r = s;
}

pub fn send_message(msg: String) -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let mut client =
            run_app::run_new_app_client::RunNewAppClient::connect("http://127.0.0.1:54321").await?;
        let request = tonic::Request::new(run_app::RunAppRequest {
            uuid: get_uuid().into(),
            msg,
        });

        let _ = client.send_msg(request).await?;

        anyhow::Ok(())
    })
}

pub fn initial_fetch()-> anyhow::Result<String>{
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let mut client =
            run_app::run_new_app_client::RunNewAppClient::connect("http://127.0.0.1:54321").await?;
        let request = tonic::Request::new(run_app::RunAppRequest {
            uuid: get_uuid().into(),
            msg:"fetch".to_string(),
        });

        let r = client.send_msg(request).await?;

        anyhow::Ok(String::from(r.into_inner().message.clone()))
    })
}
