/*
 * This file is part of Lotus Project, an Web Security Scanner written in Rust based on Lua Scripts
 * For details, please see https://github.com/rusty-sec/lotus/
 *
 * Copyright (c) 2022 - Khaled Nassar
 *
 * Please note that this file was originally released under the
 * GNU General Public License as published by the Free Software Foundation;
 * either version 2 of the License, or (at your option) any later version.
 *
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use lotus::{
    cli::{
        args::Opts,
        startup::{new::new_args, urls::args_urls},
    },
    ScanTypes,
};
use tokio::sync::RwLock;
use futures::stream::{self, StreamExt};
use futures::{channel::mpsc, sink::SinkExt};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    match Opts::from_args() {
        Opts::URLS { .. } => {
            let opts = args_urls();
            let scan_futures = vec![
                opts.lotus_obj.start(
                    opts.target_data.urls,
                    opts.req_opts.clone(),
                    ScanTypes::URLS,
                    opts.exit_after,
                ),
                opts.lotus_obj.start(
                    opts.target_data.hosts,
                    opts.req_opts,
                    ScanTypes::HOSTS,
                    opts.exit_after,
                ),
            ];
            let (mut sink, futures_stream) = mpsc::unbounded();
            let num_futures = RwLock::new(scan_futures.len());
            sink.send_all(&mut stream::iter(scan_futures.into_iter().map(Ok)))
                .await
                .unwrap();
            let sink_lock = RwLock::new(sink);
            futures_stream
                .for_each_concurrent(2, |fut| async {
                    fut.await;
                    let mut num_futures = num_futures.write().await;
                    *num_futures -= 1;
                    if *num_futures <= 0 {
                        // Close the sink to exit the for_each_concurrent
                        sink_lock.write().await.close().await.unwrap();
                    }
                })
                .await;
        }
        Opts::NEW {
            scan_type,
            file_name,
        } => {
            new_args(scan_type, file_name);
            std::process::exit(0);
        }
    };
    Ok(())
}
