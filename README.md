# TAP parser written with Pest

I had need of a [TAP](https://testanything.org/tap-version-13-specification.html) parser written in Rust. Unfortunately, all I found were TAP emitters. So, I picked up [Pest](https://github.com/pest-parser/pest) and set out to write one to fit my needs.

## Development In Progress

This library is by no means done. I just started writing the tests for the simple parser I adapted from the pseudo-EBNF found [here](https://github.com/Perl-Toolchain-Gang/Test-Harness/blob/94e8ba4c942a0f4e4eb0a483a8a8c3ee9bd9ff61/lib/TAP/Parser/Grammar.pm#L499).


- [x] basic tap functionality
- [x] tests using examples from tap spec
- [ ] more tests using real world TAP emitted from Perl tests.