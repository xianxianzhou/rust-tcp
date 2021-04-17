use std::io::{Error, Read, Write}; //导入io包
use std::net::{TcpListener, TcpStream}; //导入net包
use std::thread; //导入线程包
use std::time; //导入time包

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0; 512];
    for _ in 0..1000 { //永久循环，这里只循环1000次
        let bytes_read = stream.read(&mut buf)?; //从流里读取内容到buf中
        if bytes_read == 0 { //如果内容为0，说明已经结束
            return Ok(());
        }

        stream.write(&buf[..bytes_read])?; //响应client
        thread::sleep(time::Duration::from_secs(1 as u64)); //休眠一下
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?; //监听8080端口
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new(); //线程池

    for stream in listener.incoming() {
        let stream = stream.expect("failed!");
        let handle = thread::spawn(move || {
            handle_client(stream)
        .unwrap_or_else(|error| eprintln!("{:?}", error));
        }); //定义处理client请求的线程

        thread_vec.push(handle); //加入线程池
    }

    for handle in thread_vec { //等待线程结束
        handle.join().unwrap();
    }

    Ok(())
}