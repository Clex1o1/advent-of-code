pub mod init {
    /**
     * get_mesurements
     *
     * @return Vec<i32>
     */
    pub fn get_mesurements() -> Vec<i32> {
        use std::fs;
        // read file from  file system
        let contents = fs::read_to_string("src/assets/list.txt")
            .expect("Something went wrong reading the file");
        // parse file contents into array of i32
        let list_entries: Vec<i32> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
        list_entries
    }
}
