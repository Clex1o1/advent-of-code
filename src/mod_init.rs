pub mod init {
    /**
     * get_mesurements
     *
     * @return Vec<i32>
     */
    pub fn get_mesurements(file_name: &str) -> Vec<i32> {
        let entries = get_file(file_name);
        // parse file contents into array of i32
        let list_entries: Vec<i32> = entries.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        list_entries
    }
    pub fn get_file(file_name: &str) -> Vec<String> {
        use std::fs;
        // read file from  file system
        let path = format!("src/assets/{}.txt", file_name);
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        // parse file contents into array of i32
        let list_entries: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
        list_entries
    }
}
