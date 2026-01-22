// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // todo!()
    // v.iter().sum()
    let mid = v.len()/2;
    let left_half = v[..mid].to_vec();
    let right_half = v[mid..].to_vec();
    // thread::spawn(f)
    let left_thread = thread::spawn( move || -> i32 {
        left_half.iter().sum()
    });
    let right_thread = thread::spawn( move || -> i32 {
        right_half.iter().sum()
    });
    left_thread.join().unwrap() + right_thread.join().unwrap()
}
/*
use std::thread;

fn mat_vec_mul_scoped(matrix: &Matrix, vector: &Vector) -> Vector {
    assert_eq!(matrix[0].len(), vector.len());
    let mut result = vec![0.0; matrix.len()];
    
    // Get a mutable slice to the result vector to pass to threads
    let result_slice = &mut result[..];

    // Create a scope for the threads
    thread::scope(|s| {
        // Divide the work. Let's say we want to use 4 threads.
        let num_threads = 4;
        let rows_per_thread = (matrix.len() + num_threads - 1) / num_threads;

        for (thread_index, result_chunk) in result_slice.chunks_mut(rows_per_thread).enumerate() {
            let start_row = thread_index * rows_per_thread;
            let end_row = (start_row + result_chunk.len()).min(matrix.len());
            
            // Get a slice of the matrix rows for this thread
            let matrix_chunk = &matrix[start_row..end_row];

            // Spawn a thread to process this chunk of work.
            // We can safely borrow `matrix_chunk`, `vector`, and `result_chunk`.
            s.spawn(move || {
                for (i, row) in matrix_chunk.iter().enumerate() {
                    let dot_product = row.iter().zip(vector.iter()).map(|(a, b)| a * b).sum();
                    result_chunk[i] = dot_product;
                }
            });
        }
    }); // The scope automatically waits for all threads to finish here.

    result
}

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
