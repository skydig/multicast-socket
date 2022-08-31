use multicast_socket::MulticastSocket;
use std::net::SocketAddrV4;
use std::{time,thread};
use tokio::*;

#[tokio::main]
async fn main() {
   let mdns_multicast_address = SocketAddrV4::new([224, 0, 0, 251].into(), 5353);

 // let mdns_multicast_address = SocketAddrV4::new([239, 2, 2, 2].into(), 5353);

    // Validate that building with options works with the public API
    let with_options = MulticastSocket::with_options(
        mdns_multicast_address,
        multicast_socket::all_ipv4_interfaces()
            .expect("could not fetch all interfaces for options"),
        multicast_socket::MulticastOptions {
            ..Default::default()
        },
    )
    .expect("validate that we are starting with options");
    drop(with_options);


   // let data = vec![1, 2];
//    socket
  //      .broadcast(&data)
     //   .expect("could not broadcast message to ips being listened");
       // println!("{:?}",socket.interfaces);
        tokio::spawn(async move {
            let socket = MulticastSocket::all_interfaces(mdns_multicast_address)
            .expect("could not create and bind socket");
            let mut recv_cnt=0;
            loop {
          //  let _1s = time::Duration::from_secs(1);
            //thread::sleep(_1s);
            let ret = socket.receive();
         
                
            
            if let Ok(message) = ret {
                if recv_cnt%100 == 0 {
                dbg!(&message.interface);
                dbg!(&message.        origin_address);
                }
            } else {
                //dbg!("recv error{}",ret);
            }
            recv_cnt+=1;
            /*
            socket
                .send(&data, &message.interface)
                .expect("could not send data");
            */
        }
        }
        );
        let mut snd_cnt=0;
        let  mut socket = MulticastSocket::all_interfaces(mdns_multicast_address)
        .expect("could not create and bind socket");
        loop {
        if snd_cnt%100 == 0 {
            println!("sending....{:?}",socket.interfaces);  
        }
        snd_cnt+=1;
        let _1s = time::Duration::from_secs(1);

        thread::sleep(_1s);    
        let data = vec![9, 8, 7];
        let res = socket
        .broadcast(&data);
        if res.is_err() {
            println!("send broadcast error={:?}",res.err());
            socket = MulticastSocket::all_interfaces(mdns_multicast_address)
            .expect("could not create and bind socket");
        }
        //.expect("could not broadcast message to ips being listened");
        //println!("{:?}",socket.interfaces);
    }
}
