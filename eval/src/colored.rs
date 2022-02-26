

#[macro_export]
macro_rules! print_colored {
    ($color:expr, $($arg:tt)*) => {{
        use std::io::Write;

        let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
        termcolor::WriteColor::set_color(&mut stdout, termcolor::ColorSpec::new()
                .set_fg(Some($color))
                .set_intense(true)).unwrap();

        write!(stdout, $($arg)*).unwrap();
        termcolor::WriteColor::reset(&mut stdout).unwrap();
        std::io::Write::flush(&mut stdout).unwrap();
    }};
}

#[macro_export]
macro_rules! println_colored {
    ($color:expr, $($arg:tt)*) => {{
        $crate::print_colored!($color, $($arg)*);
        println!();
    }};
}

#[macro_export]
macro_rules! eprint_colored {
    ($color:expr, $($arg:tt)*) => {{
        use std::io::Write;
        let mut stdout = termcolor::StandardStream::stderr(termcolor::ColorChoice::Auto);
        termcolor::WriteColor::set_color(&mut stdout, termcolor::ColorSpec::new()
            .set_fg(Some($color))
            .set_intense(true)).unwrap();

        write!(stdout, $($arg)*).unwrap();
        termcolor::WriteColor::reset(&mut stdout).unwrap();
        std::io::Write::flush(&mut stdout).unwrap();
    }};
}

#[macro_export]
macro_rules! eprintln_colored {
    ($color:expr, $($arg:tt)*) => {{
        $crate::eprint_colored!($color, $($arg)*);
        print!("\n");
    }};
}

#[macro_export]
macro_rules! readln {
    () => {{
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf
    }};
}

#[macro_export]
macro_rules! readln_colored {
    ($color:expr) => {{
        use std::io::Write;

        let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
        termcolor::WriteColor::set_color(&mut stdout, termcolor::ColorSpec::new().set_fg(Some($color)).set_intense(true)).unwrap();

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        write!(stdout, "{}", buf).unwrap();
        termcolor::WriteColor::reset(&mut stdout).unwrap();
        std::io::Write::flush(&mut stdout).unwrap();
        buf
    }};
}