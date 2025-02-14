// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    // pub fn transformer(input: ???) -> ??? { ??? }

    // create a public function called transformer
    // as input we get a Vector that is a tuple of a string and a Command
    // the output is a Vector array of strings we append to
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // create a mutable vector to store the output that we will return
        let mut output = Vec::new();

        // iterate over the input vector by destructuring the tuple into a string and a command
        for (mut string, command) in input {
            // use match pattern to select action based on the command
            // so for given command we will correctly transform the string from the tuple
            match command {
                // if the command matched is Uppercase we will transform the string to uppercase
                Command::Uppercase => {
                    string = string.to_uppercase();
                }
                // if the command matched is Trim we will trim the string
                Command::Trim => {
                    string = string.trim().to_string();
                }
                // if the command matched is Append we will append "bar" to the string a specified usize times
                Command::Append(times) => {
                    for _ in 0..times {
                        string.push_str("bar");
                    }
                }
            }
            // push the transformed string to the output vector
            output.push(string);
        }
        output
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
