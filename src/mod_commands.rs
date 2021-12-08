pub mod commands {
    pub struct Position {
        x: i32,
        y: i32,
    }
    // trait Position {
    //     // This new function acts as a constructor
    //     // allowing us to add additional logic to instantiating a struct
    //     // This particular method belongs to the trait
    //     fn new (name: String) -> Self;
    //    // Signature of other functions that belong to this trait
    //     // we include a mutable version of the struct in birthday
    //     fn birthday(&mut self);
    //     fn sound (&self);
    //    }
    impl Position {
        pub fn new(x: i32, y: i32) -> Position {
            Position { x: x, y: y }
        }
        fn up(&mut self, value: i32) {
            self.y += value * -1;
        }
        fn down(&mut self, value: i32) {
            self.y -= value * -1;
        }
        fn forward(&mut self, value: i32) {
            self.x += value;
        }
        fn backward(&mut self, value: i32) {
            self.x -= value;
        }
        fn get_position(&self) -> i32 {
            self.x * self.y
        }
    }
    pub fn control(commands: Vec<String>) -> i32 {
        let mut position = Position::new(0, 0);
        commands.iter().for_each(|command| {
            let splitted_command = command.split(" ").collect::<Vec<&str>>();
            let command_value = splitted_command.last().unwrap().parse::<i32>().unwrap();
            let command_name = splitted_command.first().unwrap();
            match command_name.as_ref() {
                "up" => {
                    position.up(command_value);
                }
                "down" => {
                    position.down(command_value);
                }
                "forward" => {
                    position.forward(command_value);
                }
                "backward" => {
                    position.backward(command_value);
                }
                _ => {}
            }
        });
        return position.get_position();
    }
}
