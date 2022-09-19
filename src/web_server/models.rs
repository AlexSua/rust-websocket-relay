pub mod websocket_server_messages {
    pub mod remote {
        pub const OPEN: &str = "/remote:open";
        pub const CLOSE: &str = "/remote:close";
    }
    pub mod status {
        pub const CONNECTED: &str = "/status:connected";
        pub const WAITING: &str = "/status:waiting";
    }
    pub mod error {
        pub fn message_not_received (message:String)->String{
			let result = "/error:0001:The message was not received by anyone.:".to_owned()+message.as_str();
			return result;
		}
    }
}
