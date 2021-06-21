#[derive(Parser, Debug)]
#[grammar = "tap.pest"]
pub struct TapParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::{consumes_to, fails_with, parses_to, Parser};

    #[test]
    fn test_examples_from_TAP_website() {
        // examples from https://testanything.org/tap-specification.html
        let example1 = r#"1..6
#
# Create a new Board and Tile, then place
# the Tile onto the board.
#
ok 1 - The object isa Board
ok 2 - Board size is zero
ok 3 - The object isa Tile
ok 4 - Get possible places to put the Tile
ok 5 - Placing the tile produces no error
ok 6 - Board size is 1
"#;

        parses_to! {
            parser: TapParser,
            input: example1,
            rule: Rule::tap,
            tokens: [
                tap(0,267,[
                    plan(0,4,[
                        positiveInteger(3,4)
                    ]),
                    lines(5, 267,[
                        comment(5,6),
                        comment(7, 48, [
                            text_output(9,48)
                        ]),
                        comment(49,75,[
                            text_output(51,75)
                        ]),
                        comment(76,77),
                        test(78,105,[
                            status(78,81),
                            positiveInteger(81,82),
                            desc_text(82,105)
                        ]),
                        test(106,131,[
                            status(106,109),
                            positiveInteger(109,110),
                            desc_text(110,131)
                        ]),
                        test(132,158,[
                            status(132, 135),
                            positiveInteger(135,136),
                            desc_text(136,158)
                        ]),
                        test(159,201,[
                            status(159,162),
                            positiveInteger(162,163),
                            desc_text(163,201)
                        ]),
                        test(202,243,[
                            status(202, 205),
                            positiveInteger(205,206),
                            desc_text(206,243),
                        ]),
                        test(244,266,[
                            status(244,247),
                            positiveInteger(247,248),
                            desc_text(248,266)
                        ]),
                    ]),
                ])
            ]
        };
    }

    #[test]
    fn test_lines() {
        let lines = r#"ok 2 some text goes here #TODO finish
some unknown for you
Bail out! stuff
ok 3
"#;

        parses_to! {
            parser: TapParser,
            input: lines,
            rule: Rule::lines,
            tokens: [
                lines(0,80,[
                    test(0,37,[
                        status(0,3),
                        positiveInteger(3,4),
                        desc_text(4,25),
                        todo_directive(25,37,[
                            text_output(31,37)
                        ])
                    ]),
                    unknown(38,58),
                    bailout(59,74,[
                        text_output(68,74)
                    ]),
                    test(75,79,[
                        status(75,78),
                        positiveInteger(78,79)
                    ])
                ])
            ]
        };
    }

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
            input: "Bail out! stuff",
            rule: Rule::bailout,
            tokens: [
                bailout(0, 15, [
                    text_output(9, 15)
                ])
            ]
        };

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
            input: "some any",
            rule: Rule::unknown,
            tokens: [
                unknown(0, 8)
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
