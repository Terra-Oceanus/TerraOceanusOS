//! Macros

#[macro_export]
macro_rules! init_start {
    () => {{
        let path = module_path!();
        let count = path.matches("::").count();
        for _ in 0..(if path.starts_with("kernel::arch::x86_64::dt::") {
            count - 2
        } else if path.starts_with("kernel::arch::") {
            count - 1
        } else {
            count
        }) {
            '\t'.output();
        }
        "Initiating ".output();
        for c in path.rsplit("::").next().unwrap_or("Unknown").chars() {
            match c {
                lower if lower >= 'a' && lower <= 'z' => {
                    ((lower as u8 - 'a' as u8 + 'A' as u8) as char).output()
                }
                other => other.output(),
            }
        }
        ".\n".output();
    }};
}

#[macro_export]
macro_rules! init_check {
    ($addr:expr) => {{
        if $addr == 0 {
            let path = module_path!();
            for _ in 0..path.matches("::").count() {
                '\t'.output();
            }
            "No ".output();
            for c in path.rsplit("::").next().unwrap_or("Unknown").chars() {
                match c {
                    lower if lower >= 'a' && lower <= 'z' => {
                        ((lower as u8 - 'a' as u8 + 'A' as u8) as char).output()
                    }
                    other => other.output(),
                }
            }
            " found.\n".output();
            return Ok(());
        }
    }};
}

#[macro_export]
/// - head for n * \t
/// - tail for .\n
macro_rules! init_message {
    ($head:expr, $tail:expr, $message:expr $(, $content:expr)* $(,)?) => {{
        if $head {
            let path = module_path!();
            let count = path.matches("::").count();
            for _ in 0..(
                if path.starts_with("kernel::arch::x86_64::apic") {
                    count
                } else if path.starts_with("kernel::arch::") {
                    count - 1
                } else if path.starts_with("kernel::acpi::madt::ics::") {
                    count
                } else {
                    count + 1
                }
            ) {
                '\t'.output();
            }
        }
        $message.output();
        $(
            $content.output();
        )*
        if $tail {
            ".\n".output();
        }
    }};
}

#[macro_export]
macro_rules! init_end {
    () => {{
        let path = module_path!();
        let count = path.matches("::").count();
        for _ in 0..(if path.starts_with("kernel::arch::x86_64::dt::") {
            count - 2
        } else if path.starts_with("kernel::arch::") {
            count - 1
        } else {
            count
        }) {
            '\t'.output();
        }
        for c in path.rsplit("::").next().unwrap_or("Unknown").chars() {
            match c {
                lower if lower >= 'a' && lower <= 'z' => {
                    ((lower as u8 - 'a' as u8 + 'A' as u8) as char).output()
                }
                other => other.output(),
            }
        }
        " initialization complete.\n".output();
    }};
}
