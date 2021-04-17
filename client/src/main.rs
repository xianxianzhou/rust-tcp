use std::io::{self, prelude::*, BufReader, Write}; //导入io包 其中BufReader每次读取较多的内容，并且在内存中维护读取的结果，减少Read系统调用的次数，提高效率。
use std::net::TcpStream; //导入net包，tcp流
use std::str; //导入标准库str，对字符串进行处理

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?; //发起TCP连接
    for _ in 0..10 { //假设交互10次
        let mut input = String::new(); 
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin"); //从标准输入读取一行到input（字符串类型），如果失败，提示失败信息
        stream
            .write(input.as_bytes())
            .expect("Failed to write to stream"); //将input转成bytes并写入到流，如果写入失败，提示失败信息

        let mut reader = BufReader::new(&stream); //读取流
        let mut buffer: Vec<u8> = Vec::new();
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer"); //开始读取server端返回的信息，以换行\n为结束标记，如果读取失败，提示失败信息
        println!("Received from server: {}", 
            str::from_utf8(&buffer).expect("Could not write buffer as string")); //输出server端信息
        println!(""); //换行
    }
    Ok(())
}