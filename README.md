Save to zip (after committing): `zip aufgabe01.zip ./ -r`
Or: `git archive --format zip --output aufgabe01.zip HEAD`

Test run: `cargo run`

Run with tests: `cargo test`

This needs the test data in the `pubInst` folder.

The dua VM has the following rustc and cargo versions:

```bash
<user>@dua:~$ cargo --version
cargo 1.42.1
<user>@dua:~$ rustc --version
rustc 1.41.1
```

To set up pubInst, copy the pubInst.zip file to this folder.

And run:

```bash
unzip pubInst.zip -d pubInst
```

Some development environment don't really support older rust verions. Try using newer rust if that's the case.