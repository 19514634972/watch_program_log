use std::ffi::CString;
use std::time::Duration;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use watch_script::APPLICATION_CONTEXT;
use whole_config::config::ApplicationConfig;

pub fn client()->FutureProducer {
    let mut config = ClientConfig::new();
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let addr = cassie_config.kafka();
    config.set("bootstrap.servers", &addr.address);

    let producer = config.create().expect("创建生产者失败");
    producer
}


//发送生产者信息
pub async fn produce(future_producer:FutureProducer,msg:String,topic:String){
    let record = FutureRecord::to(&topic)
        .payload(msg.as_str())
        .key("Test-Key");

    let status_delievery = future_producer
        .send(record, Timeout::After(Duration::from_secs(2)))
        .await;
    match status_delievery {
        Ok(report) => println!("消息成功发送: {:?}", report),
        Err(e) => println!("消息发送失败: {:?}", e),
    }

}
