pub mod mesurements {
    pub fn chunk_sum_measurements(mesurements: &Vec<i32>) -> Vec<i32> {
        let chunk_size = 3;
        let chunk_step = 1;
        let mut chunk_index = 0;
        let mut chunks = Vec::new();
        mesurements.iter().for_each(|_measurement| {
            let chunk = mesurements.iter().skip(chunk_index).take(chunk_size);
            if chunk.len() == chunk_size {
                let sum = chunk.sum::<i32>();
                chunks.push(sum);
            }
            chunk_index += chunk_step;
        });
        chunks
    }
}
