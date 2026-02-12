use crate::adapters::adapters_channel;

mod adapters;
mod api;
mod ports;

fn main() -> iced::Result {
    let (adapter, daemon) = adapters_channel();

    let server_adapter = adapter.clone();

    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");

        runtime.block_on(api::run_server(server_adapter));
    });

    daemon.run()
}
