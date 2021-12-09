# Hello-world example for KolibriOS

Project uses [cargo-make](https://github.com/sagiegurari/cargo-make) for building steps.
Also you need a working [NASM](https://nasm.us/).

Once installed building is trivial then: `cargo make --profile production` produces
a ready-to-use binary at `target/i686-kolibri/release/hw_kolibri`.
