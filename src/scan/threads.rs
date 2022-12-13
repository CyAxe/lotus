use crossbeam_channel::bounded;
use std::time::Instant;

pub struct Threader {
    pub errors_limit: i32,
    pub threads: usize,
    errors: i32
}


impl Threader {
    pub fn init(threads: usize,errors_limit: i32) -> Threader {
        Threader {
            threads,
            errors_limit,
            errors: 0
        }
    }

    pub async fn start(&self,urls: Vec<String>) {
        let (script_sender, script_reciver) = bounded(32);
        let mut threads: Vec<tokio::task::JoinHandle<_>> = vec![];

        for url in urls {
            script_sender.send(url).unwrap();
        }
        for task_id in 0..self.threads {
            let new_script_reciver = script_reciver.clone();
            threads.push(
                tokio::spawn(async move {
                    while let Ok(recive_data) = new_script_reciver.recv_deadline(Instant::now()) {
                            println!("Done {} : {:?}",task_id,recive_data);
                    }
                })
            );

        }

        for i in threads {
            i.await.unwrap();
        }
        

    }
}
