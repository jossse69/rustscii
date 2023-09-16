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
        let mut terminal = Terminal::new("assets/terminal8x8_gs_ro.png", 8, 8);
        
        let handle = thread::spawn(move || {
            
            terminal.set_char(0, 0, 'L');

            // render the terminal
            let render = terminal.render();

            let terminal_rc = terminal.get_rc();
            run(terminal_rc);
        });

        // Wait for the thread to complete and check if it panics.
        handle.join().unwrap();
    }
}
