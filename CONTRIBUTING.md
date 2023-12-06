# Chess-tui contribution guidelines

Thank you for your interest in improving Chess-tui! We'd love to have your contribution. We expect all contributors to abide by the [Rust code of conduct], which you can find at that link.

[Rust code of conduct]: https://www.rust-lang.org/policies/code-of-conduct

## License

Chess-tui is MIT licensed project and so are all
contributions. Please see the [`LICENSE-MIT`] files in
this directory for more details.

[`LICENSE-MIT`]: https://github.com/rust-lang/rust-by-example/blob/master/LICENSE-MIT


## Pull Requests

To make changes to Chess-tui, please send in pull requests on GitHub to the `main`
branch. We'll review them and either merge or request changes. Travis CI tests
everything as well, so you may get feedback from it too.

If you make additions or other changes to a pull request, feel free to either amend
previous commits or only add new ones, however you prefer. At the end the commit will be squashed.

## Issue Tracker

You can find the issue tracker [on
GitHub](https://github.com/thomas-mauran/chess-tui/issues). If you've found a
problem with Chess-tui, please open an issue there.

We use the following labels:

* `enhancement`: This is for any request for new sections or functionality.
* `bug`: This is for anything that's in Chess-tui, but incorrect or not working.
* `documentation`: This is for anything related to documentation.
* `help wanted`: This is for issues that we'd like to fix, but don't have the time
  to do ourselves. If you'd like to work on one of these, please leave a comment
  saying so, so we can help you get started.
* `good first issue`: This is for issues that are good for people who are new to the project or open-source community in general.

## Development workflow

To build Chess-tui, [install Rust](https://www.rust-lang.org/tools/install), and then:

```bash
$ git clone https://github.com/thomas-mauran/chess-tui
$ cd chess-tui
$ cargo build --release
$ ./target/release/chess-tui
```