// macro should appear before module declaration
#[macro_use]
mod macros {
    macro_rules! synchronous_request {
        ($tx:expr, $msg:expr, $rx:expr, $response:path, $result:expr, $err:expr) => {{
            $tx.send($msg).await?;
            match $rx.recv().await.ok_or_else(|| $err)? {
                $response(..) => Ok($result),
                _ => Err($err),
            }
        }};
    }
}

////////////////////////////////////////////7
pub mod channel;
pub mod connection;
pub mod error;