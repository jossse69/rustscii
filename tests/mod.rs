
#[cfg(test)]
mod tests {
    // Import necessary symbols from your crate
    extern crate rustscii;

    use std::thread;
    use rustscii::rendering::run; // Import the run function from your crate

    #[test]
    #[should_panic]
    fn test_rendering_initialization() {
        // Create a new thread to run the rendering initialization.
        let handle = thread::spawn(|| {
            run();
        });

        // Wait for the thread to complete and check if it panics.
        handle.join().unwrap();
    }
}
