#[derive(Parser, Debug)]
#[grammar = "tap.pest"]
pub struct TapParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::{consumes_to, fails_with, parses_to};

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
    fn test_tap_test() {
        parses_to! {
            parser: TapParser,
            input: "ok ",
            rule: Rule::test,
            tokens: [
                test(0,3,[
                    status(0,3)
                ])
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "ok 2",
            rule: Rule::test,
            tokens: [
                test(0,4,[
                    status(0,3),
                    positiveInteger(3,4)
                ])
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "ok 2 some text goes here",
            rule: Rule::test,
            tokens: [
                test(0,24,[
                    status(0,3),
                    positiveInteger(3,4),
                    desc_text(4,24)
                ])
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "ok 2 some text goes here #TODO finish",
            rule: Rule::test,
            tokens: [
                test(0,37,[
                    status(0,3),
                    positiveInteger(3,4),
                    desc_text(4,25),
                    todo_directive(25,37,[
                        text_output(31,37)
                    ])
                ])
            ]
        };
    }

    #[test]
    fn test_tap_status() {
        parses_to! {
            parser: TapParser,
            input: "ok ",
            rule: Rule::status,
            tokens: [
                status(0,3)
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "not ok ",
            rule: Rule::status,
            tokens: [
                status(0,7)
            ]
        };

        fails_with! {
            parser: TapParser,
            input: " not ok",
            rule: Rule::status,
            positives: vec![Rule::status],
            negatives: vec![],
            pos: 0
        };

        fails_with! {
            parser: TapParser,
            input: " ok",
            rule: Rule::status,
            positives: vec![Rule::status],
            negatives: vec![],
            pos: 0
        };
    }

    #[test]
    fn test_tap_desc_text() {
        parses_to! {
            parser: TapParser,
            input: "And there was some text",
            rule: Rule::desc_text,
            tokens: [
                desc_text(0,23)
            ]
        };

        fails_with! {
            parser: TapParser,
            input: "\n# TODO with stuff after",
            rule: Rule::desc_text,
            positives: vec![Rule::desc_text],
            negatives: vec![],
            pos: 0
        };

        fails_with! {
            parser: TapParser,
            input: "# TODO with stuff after",
            rule: Rule::desc_text,
            positives: vec![Rule::desc_text],
            negatives: vec![],
            pos: 0
        };
    }

    #[test]
    fn test_tap_todo_directive() {
        parses_to! {
            parser: TapParser,
            input: "#TODO ",
            rule: Rule::todo_directive,
            tokens: [
                todo_directive(0,6)
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "# TODO with stuff after",
            rule: Rule::todo_directive,
            tokens: [
                todo_directive(0,23,[
                    text_output(7,23)
                ])
            ]
        };
    }

    #[test]
    fn test_tap_skip_directive() {
        parses_to! {
            parser: TapParser,
            input: "#SKIP ",
            rule: Rule::skip_directive,
            tokens: [
                skip_directive(0,6)
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "# SKIP with stuff after",
            rule: Rule::skip_directive,
            tokens: [
                skip_directive(0,23,[
                    text_output(7,23)
                ])
            ]
        };
    }

    #[test]
    fn test_tap_comment() {
        parses_to! {
            parser: TapParser,
            input: "#",
            rule: Rule::comment,
            tokens: [
                comment(0,1)
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "# ",
            rule: Rule::comment,
            tokens: [
                comment(0,2)
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "# and some more stuff",
            rule: Rule::comment,
            tokens: [
                comment(0,21,[
                    text_output(2,21)
                ])
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "#this works, too",
            rule: Rule::comment,
            tokens: [
                comment(0,16,[
                    text_output(1,16)
                ])
            ]
        };
    }

    #[test]
    fn test_tap_bailout() {
        parses_to! {
            parser: TapParser,
            input: "Bail out! HERE IS WHY!",
            rule: Rule::bailout,
            tokens: [
                bailout(0, 22, [
                    text_output(9, 22)
                ])
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "Bail out!",
            rule: Rule::bailout,
            tokens: [
                bailout(0, 9)
            ]
        };
    }

    #[test]
    fn test_tap_unknown() {
        parses_to! {
            parser: TapParser,
            input: "some any\n",
            rule: Rule::unknown,
            tokens: [
                unknown(0, 9, [
                    text_output(0, 8)
                ])
            ]
        };
    }

    #[test]
    fn test_tap_text_output() {
        parses_to! {
            parser: TapParser,
            input: "4",
            rule: Rule::text_output,
            tokens: [
                text_output(0, 1)
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "{whole}",
            rule: Rule::text_output,
            tokens: [
                text_output(0, 7)
            ]
        };

        fails_with! {
            parser: TapParser,
            input: "\nasdfasfaf",
            rule: Rule::text_output,
            positives: vec![Rule::text_output],
            negatives: vec![],
            pos: 0
        };
    }

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

        parses_to! {
            parser: TapParser,
            input: "120",
            rule: Rule::positiveInteger,
            tokens: [
                positiveInteger(0, 3)
            ]
        };

        fails_with! {
            parser: TapParser,
            input: "0",
            rule: Rule::positiveInteger,
            positives: vec![Rule::positiveInteger],
            negatives: vec![],
            pos: 0
        };

        fails_with! {
            parser: TapParser,
            input: "-11",
            rule: Rule::positiveInteger,
            positives: vec![Rule::positiveInteger],
            negatives: vec![],
            pos: 0
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

        fails_with! {
            parser: TapParser,
            input: "0..4",
            rule: Rule::plan,
            positives: vec![Rule::plan],
            negatives: vec![],
            pos: 0
        };
    }
}
