use tokio::sync::oneshot;
// #[tokio::main] 
// async fn main() {
//     let (tx1, rx1) = oneshot::channel();
//     let (tx2, rx2) = oneshot::channel();

//     tokio::spawn(async {
//         let _ = tx1.send("one");
//     });
//     tokio::spawn(async {
//         let _ = tx2.send("two");
//     });

//     tokio::select! {
//         val = rx1 => {
//             println!("rx1 receive first wit {:?}", val)
//         }
//         val = rx2 => {
//             println!("rx2 receive fist wait {:?}", val)
//         }
//     }

//     // branch will be await by select and retrun the branch which one complete first other will be drop . 
//     // the val return Ok(val) cancellation is performed by drop trait. drop trait clean the backgroud recources.
//     // receiver send the close notification to the sender 
// }
// async fn action(input: Option<i32>) -> Option<String> {
//     Some(input.unwrap().to_string())
// }


/* 
#[tokio::main]

async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(128);

    let mut done = false;
    let operation = action(Some(301));
    tokio::pin!(operation);

    // spawning message on the channel in a seperate task
    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let  _= tx.send(3).await;
        let _ = tx.send(2).await;
    });

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;
                if let Some(v) = res{
                    println!("GOT ={}", v);
                    return;
                }
            }
            Some(v)  = rx.recv() => {
                if v % 2  == 0 {
                    operation.set(action(Some(v)));
                
                    done = false;
                }
            }
        }
    }

}
 */




use std::future::Future;
 use std::io;
use std::net::TcpListener;
use std::pin::Pin;
 use std::task::{Context, Poll};

// struct MySelect {
//     rx1: oneshot::Receiver<&'static str>,
//     rx2: oneshot::Receiver<&'static str>
// }

// impl Future for MySelect {
//     type Output = ();

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
//         if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
//             println!("rx1 completed first with {:?}",  val);
//             return Poll::Ready(());
//         }
//         if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx)  {
//             println!("rx2 completed first with : {:?}", val);
//             return Poll::Ready(());
//         }
//         Poll::Pending
//     }
// }

// #[tokio::main]
// async fn main() {
//     let (tx1, rx1) = oneshot::channel();
//     let (tx2, rx2) = oneshot::channel();

//     MySelect {
//         rx1, rx2
//     }.await
// }

// async fn computation1() ->String{
//     "string".to_string()
// }

// async fn computation2() -> String{
//     "string2".to_string()
// }


// #[tokio::main]
// async fn main() {
//     let out = tokio::select! {
//         res1 = computation1() => res1,
//         res2 = computation2() => res2
//     };
//     println!("GOT = {}", out);
// }
