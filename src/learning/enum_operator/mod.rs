// mod enum_operator {

pub mod operator {
    #[derive(Debug)]

    enum AppVersion {
        V1,
        V2,
    }

    enum IpAddressKind {
        V4,
        V6,
    }

    impl IpAddressKind {
        fn route(ip_kind: Self) -> String {
            match ip_kind {
                Self::V4 => "192.0.2.3".to_string(),
                Self::V6 => "::1".to_string(),
            }
        }
    }

    enum _IpAddressKind {
        V4(u8, u8, u8, u8),
        V6(u8, u8, u8, u8),
    }

    struct IpAddress {
        kind: IpAddressKind,
        address: String,
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("Message: {}", 2);
        }
    }

    enum Option<T> {
        None,
        Some(T),
    }

    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    impl Coin {
        fn cents(coin: Self) -> u8 {
            match coin {
                Self::Penny => 1,
                Self::Dime => 5,
                Self::Nickel => 10,
                Self::Quarter(state) => {
                    // println!("State quarter from {:?}!", state);
                    return match state {
                        UsState::Alabama => 24,
                        UsState::Alaska => 25,
                    };
                }
            }
        }
    }

    pub fn main() {
        let _v1: AppVersion = AppVersion::V1;
        let home: IpAddress = IpAddress {
            kind: IpAddressKind::V4,
            address: "192.0.2.3".to_string(),
        };

        let _home: _IpAddressKind = _IpAddressKind::V4(127, 0, 0, 1);

        let loopback: IpAddress = IpAddress {
            kind: IpAddressKind::V6,
            address: "::1".to_string(),
        };

        let m = Message::Write(String::from("My message"));
        m.call();

        let some_number = Option::Some(10);
        let some_char = Some('K');
        let absent_number: Option<i32> = Option::None;

        // let sum = some_number + 2;

        // println!("Some number: {:?}", some_number);

        // println!("Ip address of home is {:?}", _home);
        // println!("Ip address of loopback is {:?}", loopback);

        println!("Route for v6 {}", IpAddressKind::route(IpAddressKind::V6));
        println!("Nickel in cents: {}", Coin::cents(Coin::Nickel));
        println!(
            "Quarter in cents: {}",
            Coin::cents(Coin::Quarter(UsState::Alaska))
        );

        if_max();
    }

    fn if_max() {
        let config_max = Some(32325u16);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        }
    }
}
