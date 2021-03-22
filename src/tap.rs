#[derive(Parser, Debug)]
#[grammar = "tap.pest"]
pub struct TapParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::{consumes_to, parses_to};


//     #[test]
//     fn test_tap_from_website() {
//         let tap = r#"1..4
// ok 1 - Input file opened
// not ok 2 - First line of the input valid
// ok 3 - Read the rest of the file
// not ok 4 - Summarized correctly # TODO Not written yet"#;

//         let out = TapParser::parse(Rule::tap, tap);

//         println!("{:#?}", out);
//     }

    #[test]
    fn test_tap_positive_integer() {
        parses_to! {
            parser: TapParser,
            input: "4",
            rule: Rule::positiveInteger,
            tokens: [
                positiveInteger(0, 1)
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "120",
            rule: Rule::positiveInteger,
            tokens: [
                positiveInteger(0, 3)
            ]
        };
    }

    #[test]
    fn test_tap_plan() {
        parses_to! {
            parser: TapParser,
            input: "1..4",
            rule: Rule::plan,
            tokens: [
                plan(0, 4, [
                    positiveInteger(3, 4)
                ])
            ]
        };
    }
}