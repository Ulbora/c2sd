// Simple Rust implementation of C2SD
pub struct ControllerData<'a> {
    pub oauth2: Vec<&'a str>,
    pub controllers: Vec<&'a str>,
    pub servers: Vec<&'a str>,
}

pub struct Controller<'a> {
    pub list: ControllerData<'a>,
}

impl Controller<'_> {
    pub fn get_controller_server_list(&self) -> ControllerData {
        let rtn = ControllerData {
            oauth2: self.list.oauth2.clone(),
            controllers: self.list.controllers.clone(),
            servers: self.list.servers.clone(),
        };
        rtn
    }
}

pub struct OAuth2<'a> {
    pub token: &'a str,
}

impl OAuth2<'_> {
    pub fn get_token(&self) -> &str {
        self.token
    }
}

pub struct Server<'a> {
    pub data: &'a str,
}

impl Server<'_> {
    pub fn get_data(&self) -> &str {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use crate::Controller;
    use crate::ControllerData;
    use crate::OAuth2;
    use crate::Server;
    #[test]

    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn get_controller_server_list() {
        let list = ControllerData {
            oauth2: vec!["firstUrl", "secondUrl", "thirdUrl", "fourthUrl"],
            controllers: vec!["firstUrl", "secondUrl", "thirdUrl", "fourthUrl"],
            servers: vec!["firstUrl", "secondUrl", "thirdUrl", "fourthUrl"],
        };
        let c = Controller { list: list };
        let new_list = c.get_controller_server_list();
        assert_eq!(new_list.oauth2.len(), 4);
        assert_eq!(new_list.controllers.len(), 4);
        assert_eq!(new_list.servers.len(), 4);
    }

    #[test]
    fn get_token() {
        let o = OAuth2 {
            token: "sdffsffdfss61fs",
        };
        let tk = o.get_token();
        assert_eq!(tk, "sdffsffdfss61fs");
    }

    #[test]
    fn get_data() {
        let s = Server { data: "12345" };
        let d = s.get_data();
        assert_eq!(d, "12345");
    }
}
