#[cfg(test)]
mod tests {
    // Import necessary symbols from your crate
    extern crate rustscii;

    use std::cell::RefCell;
    use std::rc::Rc;
    use std::thread;
    use std::sync::{Arc, Mutex};
    use rustscii::rendering::run; // Import the run function from your crate
    use rustscii::rendering::terminal::Terminal;

    #[test]
    #[should_panic]
    fn test_rendering_initialization() {
        // Create a Terminal instance and wrap it in an Arc<Mutex<Rc<RefCell<Terminal>>>> for thread safety.
        let terminal = Terminal::new("assets/terminal8x8_gs_ro.png", 8, 8);



        // Create a new thread to run the rendering initialization.
        let handle = thread::spawn(move || {
            // Clone the Arc to get an Rc.
            let terminal_rc = terminal.get_rc();
            // Lock the Mutex to get a reference to Terminal.
            run(terminal_rc);
        });

        // Wait for the thread to complete and check if it panics.
        handle.join().unwrap();
    }
}
