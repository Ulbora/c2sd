use c2sd::Controller;
use c2sd::ControllerData;
use c2sd::OAuth2;
use c2sd::Server;
fn main() {
    // simple implementation of OAuth2
    let o = OAuth2 {
        token: "sdffsffdfss61fs",
    };
    let tk = o.get_token();
    println!("Got new token: {}", tk);

    let list = ControllerData {
        oauth2: vec!["firstUrl", "secondUrl", "thirdUrl", "fourthUrl"],
        controllers: vec!["firstUrl", "secondUrl", "thirdUrl", "fourthUrl"],
        servers: vec!["firstUrl", "secondUrl", "thirdUrl", "fourthUrl"],
    };

    // simple implementation of a Controller
    let c = Controller { list: list };
    let new_list = c.get_controller_server_list();
    println!("Got new oauth2 list from controller: {:?}", new_list.oauth2);
    println!(
        "Got new controller list from controller: {:?}",
        new_list.controllers
    );
    println!(
        "Got new servers list from controller: {:?}",
        new_list.servers
    );

    // simple implementation of a Server
    let s = Server { data: "12345" };
    let d = s.get_data();
    println!("Got some server data: {:?}", d);
}
