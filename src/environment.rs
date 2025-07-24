pub mod retreive {
    // #TO-DO: Implement CLI arguments for these
    pub mod system {
        use local_ip_address::local_ip;
        use std::net::{IpAddr, Ipv4Addr};

        pub fn get_ip() -> IpAddr {
            match local_ip() {
                Ok(ip) => ip,
                Err(err) => {
                    println!("Failed to get ipv4 address: {err},\ndefaulting to 127.0.0.1");
                    return IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
                }
            }
        }
    }
    pub mod mariadb {
        use std::env::VarError::{NotPresent, NotUnicode};
        use std::env::var;

        pub fn get_host() -> String {
            match var("MARIAHOST") {
                Err(NotUnicode(_)) => {
                    println!(
                        "host in $MARIAHOST is not readable to string, defaulting to localhost"
                    );
                    String::from("localhost")
                }
                Err(NotPresent) => {
                    println!("$MARIAHOST not set, defaulting to localhost");
                    String::from("localhost")
                }
                Ok(host) => host,
            }
        }

        pub fn get_user() -> String {
            match var("MARIAUSER") {
                Err(NotUnicode(_)) => {
                    println!("user in $MARIAUSER is not readable to string, defaulting to root");
                    String::from("root")
                }
                Err(NotPresent) => {
                    println!("$MARIAUSER is not set, defaulting to root");
                    String::from("root")
                }
                Ok(user) => user,
            }
        }

        pub fn get_password() -> String {
            match std::env::var("MARIAPASS") {
                Err(NotUnicode(_)) => {
                    println!(
                        "password in $MARIAPASS is not readable to string, defaulting to admin"
                    );
                    String::from("admin")
                }
                Err(NotPresent) => {
                    println!("$MARIAPASS is not set, defaulting to admin");
                    String::from("admin")
                }
                Ok(x) => x,
            }
        }

        pub fn get_database() -> String {
            match std::env::var("MARIADATABASE") {
                Err(NotUnicode(_)) => {
                    println!(
                        "database in $MARIADATABASE is not readable to string, defaulting to server"
                    );
                    String::from("server")
                }
                Err(NotPresent) => {
                    println!("$MARIADATABASE is not set, defaulting to server");
                    String::from("server")
                }
                Ok(x) => x,
            }
        }
    }
}
