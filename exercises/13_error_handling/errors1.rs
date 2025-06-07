// use std::fs::File;
// use std::io::ErrorKind; 

// TODO: This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was instead of just returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Change
// the function signature and body to return `Result<String, String>` instead
// of `Option<String>`.
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed
        // None
        Err("Empty names aren't allowed".to_string())
    } else {
        // Some(format!("Hi! My name is {name}"))
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // You can optionally experiment here.
    // let f = File::open("hello.txt");
    // 
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match file::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     }
    // };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
