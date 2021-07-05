#[derive(Parser, Debug)]
#[grammar = "tap.pest"]
pub(crate) struct TapParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::{consumes_to, fails_with, parses_to, Parser};

    // examples from https://testanything.org/tap-specification.html
    pub const EXAMPLE_TOP_PLAN_SUCCESS: &str = r#"1..6
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

    pub const EXAMPLE_TAIL_PLAN_FAIL: &str = r#"ok 1 - retrieving servers from the database
# need to ping 6 servers
ok 2 - pinged diamond
ok 3 - pinged ruby
not ok 4 - pinged saphire
ok 5 - pinged onyx
not ok 6 - pinged quartz
ok 7 - pinged gold
1..7
"#;

    pub const EXAMPLE_TOP_PLAN_BAILOUT: &str = r#"1..573
not ok 1 - database handle
Bail out! Couldn't connect to database.
"#;

    pub const EXAMPLE_TOP_PLAN_SKIP_SUCCESS: &str = r#"1..5
ok 1 - approved operating system
# $^0 is solaris
ok 2 - # SKIP no /sys directory
ok 3 - # SKIP no /sys directory
ok 4 - # SKIP no /sys directory
ok 5 - # SKIP no /sys directory
"#;

    pub const EXAMPLE_FULL_SKIP: &str = r"1..0 # skip because English-to-French translator isn't installed
";

    pub const EXAMPLE_TOP_PLAN_TODO_SUCCESS: &str = r"1..4
ok 1 - Creating test program
ok 2 - Test program runs, no error
not ok 3 - infinite loop # TODO halting problem unsolved
not ok 4 - infinite loop 2 # TODO halting problem unsolved
";

    pub const EXAMPLE_TAIL_PLAN_NO_DESC_TEST_SUCCESS: &str = r"ok - created Board
ok
ok
ok
ok
ok
ok
ok
# +------+------+------+------+
# |      |16G   |      |05C   |
# |      |G N C |      |C C G |
# |      |  G   |      |  C  +|
# +------+------+------+------+
# |10C   |01G   |      |03C   |
# |R N G |G A G |      |C C C |
# |  R   |  G   |      |  C  +|
# +------+------+------+------+
# |      |01G   |17C   |00C   |
# |      |G A G |G N R |R N R |
# |      |  G   |  R   |  G   |
# +------+------+------+------+
ok - board has 7 tiles + starter tile
1..9
";

    #[test]
    fn test_examples_from_TAP_website() {
        parses_to! {
            parser: TapParser,
            input: EXAMPLE_TOP_PLAN_SUCCESS,
            rule: Rule::tap,
            tokens: [
                tap(0,267,[
                    plan(0,4,[
                        nonNegativeInteger(3,4)
                    ]),
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
                        desc_text(85,105)
                    ]),
                    test(106,131,[
                        status(106,109),
                        positiveInteger(109,110),
                        desc_text(113,131)
                    ]),
                    test(132,158,[
                        status(132, 135),
                        positiveInteger(135,136),
                        desc_text(139,158)
                    ]),
                    test(159,201,[
                        status(159,162),
                        positiveInteger(162,163),
                        desc_text(166,201)
                    ]),
                    test(202,243,[
                        status(202, 205),
                        positiveInteger(205,206),
                        desc_text(209,243),
                    ]),
                    test(244,266,[
                        status(244,247),
                        positiveInteger(247,248),
                        desc_text(251,266)
                    ]),
                ])
            ]
        };

        parses_to! {
            parser: TapParser,
            input: EXAMPLE_TAIL_PLAN_FAIL,
            rule: Rule::tap,
            tokens: [
                tap(0,204,[
                    test(0,43,[
                        status(0,3),
                        positiveInteger(3,4),
                        desc_text(7,43)
                    ]),
                    comment(44,68,[
                        text_output(46,68)
                    ]),
                    test(69,90,[
                        status(69,72),
                        positiveInteger(72,73),
                        desc_text(76,90)
                    ]),
                    test(91,109,[
                        status(91, 94),
                        positiveInteger(94,95),
                        desc_text(98,109)
                    ]),
                    test(110,135,[
                        status(110,117),
                        positiveInteger(117,118),
                        desc_text(121,135)
                    ]),
                    test(136,154,[
                        status(136, 139),
                        positiveInteger(139,140),
                        desc_text(143,154),
                    ]),
                    test(155,179,[
                        status(155,162),
                        positiveInteger(162,163),
                        desc_text(166,179)
                    ]),
                    test(180,198, [
                        status(180,183),
                        positiveInteger(183,184),
                        desc_text(187,198)
                    ]),
                    plan(199,203,[
                        nonNegativeInteger(202,203)
                    ])
                ])
            ]
        };

        parses_to! {
            parser: TapParser,
            input: EXAMPLE_TOP_PLAN_BAILOUT,
            rule: Rule::tap,
            tokens: [
                tap(0,74,[
                    plan(0,6,[
                        nonNegativeInteger(3,6)
                    ]),
                    test(7,33,[
                        status(7,14),
                        positiveInteger(14,15),
                        desc_text(18,33)
                    ]),
                    bailout(34,73,[
                        text_output(44,73)
                    ])
                ])
            ]
        };

        parses_to! {
            parser: TapParser,
            input: EXAMPLE_TOP_PLAN_SKIP_SUCCESS,
            rule: Rule::tap,
            tokens: [
                tap(0,183,[
                    plan(0,4,[
                        nonNegativeInteger(3,4)
                    ]),
                    test(5,37,[
                        status(5,8),
                        positiveInteger(8,9),
                        desc_text(12,37)
                    ]),
                    comment(38,54,[
                        text_output(40,54)
                    ]),
                    test(55,86,[
                        status(55,58),
                        positiveInteger(58,59),
                        skip_directive(62,86,[
                            text_output(69,86)
                        ])
                    ]),
                    test(87,118,[
                        status(87,90),
                        positiveInteger(90,91),
                        skip_directive(94,118,[
                            text_output(101,118)
                        ])
                    ]),
                    test(119,150,[
                        status(119,122),
                        positiveInteger(122,123),
                        skip_directive(126,150,[
                            text_output(133,150)
                        ])
                    ]),
                    test(151,182,[
                        status(151,154),
                        positiveInteger(154,155),
                        skip_directive(158,182,[
                            text_output(165,182)
                        ])
                    ])
                ])
            ]
        };

        parses_to! {
            parser: TapParser,
            input: EXAMPLE_FULL_SKIP,
            rule: Rule::tap,
            tokens: [
                tap(0,65,[
                    plan(0,4,[
                        nonNegativeInteger(3,4)
                    ]),
                    skip_directive(5,64,[
                        text_output(12,64)
                    ])
                ])
            ]
        };

        parses_to! {
            parser: TapParser,
            input: EXAMPLE_TOP_PLAN_TODO_SUCCESS,
            rule: Rule::tap,
            tokens: [
                tap(0,185,[
                    plan(0,4,[
                        nonNegativeInteger(3,4)
                    ]),
                    test(5,33,[
                        status(5,8),
                        positiveInteger(8,9),
                        desc_text(12,33)
                    ]),
                    test(34,68,[
                        status(34,37),
                        positiveInteger(37,38),
                        desc_text(41,68)
                    ]),
                    test(69,125,[
                        status(69,76),
                        positiveInteger(76,77),
                        desc_text(80,94),
                        todo_directive(94,125,[
                            text_output(101,125)
                        ])
                    ]),
                    test(126,184,[
                        status(126,133),
                        positiveInteger(133,134),
                        desc_text(137,153),
                        todo_directive(153,184,[
                            text_output(160,184)
                        ])
                    ])
                ])
            ]
        };

        parses_to! {
            parser: TapParser,
            input: EXAMPLE_TAIL_PLAN_NO_DESC_TEST_SUCCESS,
            rule: Rule::tap,
            tokens: [
                tap(0,499,[
                    test(0,18,[
                        status(0,3),
                        desc_text(5,18)
                    ]),
                    test(19,21,[
                        status(19,21)
                    ]),
                    test(22,24,[
                        status(22,24)
                    ]),
                    test(25,27,[
                        status(25,27)
                    ]),
                    test(28,30,[
                        status(28,30)
                    ]),
                    test(31,33,[
                        status(31,33)
                    ]),
                    test(34,36,[
                        status(34,36)
                    ]),
                    test(37,39,[
                        status(37,39)
                    ]),
                    comment(40,71,[
                        text_output(42,71)
                    ]),
                    comment(72,103,[
                        text_output(74,103)
                    ]),
                    comment(104,135,[
                        text_output(106,135)
                    ]),
                    comment(136,167,[
                        text_output(138,167)
                    ]),
                    comment(168,199,[
                        text_output(170,199)
                    ]),
                    comment(200,231,[
                        text_output(202,231)
                    ]),
                    comment(232,263,[
                        text_output(234,263)
                    ]),
                    comment(264,295,[
                        text_output(266,295)
                    ]),
                    comment(296,327,[
                        text_output(298,327)
                    ]),
                    comment(328,359,[
                        text_output(330,359)
                    ]),
                    comment(360,391,[
                        text_output(362,391)
                    ]),
                    comment(392,423,[
                        text_output(394,423)
                    ]),
                    comment(424,455,[
                        text_output(426,455)
                    ]),
                    test(456,493,[
                        status(456,459),
                        desc_text(461,493)
                    ]),
                    plan(494,498,[
                        nonNegativeInteger(497,498)
                    ])
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
                test(0,37,[
                    status(0,3),
                    positiveInteger(3,4),
                    desc_text(5,25),
                    todo_directive(25,37,[
                        text_output(31,37)
                    ])
                ]),
                unknown(38,58),
                bailout(59,74,[
                    text_output(69,74)
                ]),
                test(75,79,[
                    status(75,78),
                    positiveInteger(78,79)
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
                    desc_text(5,24)
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
                    desc_text(5,25),
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
            input: "#todo lowercase",
            rule: Rule::todo_directive,
            tokens: [
                todo_directive(0,15,[
                    text_output(6,15)
                ])
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

        parses_to! {
            parser: TapParser,
            input: "# skip lower case",
            rule: Rule::skip_directive,
            tokens: [
                skip_directive(0,17,[
                    text_output(7,17)
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
                    text_output(10, 15)
                ])
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "Bail out! HERE IS WHY!",
            rule: Rule::bailout,
            tokens: [
                bailout(0, 22, [
                    text_output(10, 22)
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
            input: "1..0",
            rule: Rule::plan,
            tokens: [
                plan(0, 4, [
                    nonNegativeInteger(3, 4)
                ])
            ]
        };

        parses_to! {
            parser: TapParser,
            input: "1..4",
            rule: Rule::plan,
            tokens: [
                plan(0, 4, [
                    nonNegativeInteger(3, 4)
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
