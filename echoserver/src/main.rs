use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, str, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let addr = &args[1];
    echo_server(addr)?;

    Ok(())
}

fn echo_server(address: &str) -> Result<(), Box<dyn Error>> {
    // [1]
    // リッスンモードのソケットを指定のアドレスで作成する
    let listener = TcpListener::bind(address)?;

    loop {
        // [2]
        // スレッドをブロックし、クライアントからのコネクション確立要求を待機する
        // TCPスリーハンドシェイクを通してコネクションが確立したらブロックを解除する
        let (mut stream, _) = listener.accept()?;

        // [3]
        // 新たにスレッドAを作成し、起動する
        // メインスレッドは[2]に戻り、スレッドAは[4]に移る
        //  [2]で作られたstreamが新たなスレッドへmoveされる
        thread::spawn(move || {
            let mut buffer = [0u8; 1024];
            loop {
                // [4]
                // スレッドをブロックしてデータの受信を待機する
                // 受信が完了するとブロックを解除し、確認応答をクライアントに送信する
                let nbytes = stream.read(&mut buffer).unwrap();
                // [6]
                // クライアントのCtrl+Cにより切断要求が送られている
                // 生成された各スレッドが終了処理を行い、[2]で生成されたソケットを破棄する
                if nbytes == 0 {
                    return;
                }
                print!("{}", str::from_utf8(&buffer[..nbytes]).unwrap());
                // [5]
                // クライアントにデータを送信する
                // クライアントからは確認応答を受信する
                stream.write_all(&buffer[..nbytes]).unwrap();
            }
        });
    }
}
