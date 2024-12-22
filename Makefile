
lint:
	cargo clippy \
        -- \
        \
        -W clippy::all \
        -W clippy::pedantic \
        \
        -A clippy::must-use-candidate \
        -A clippy::missing_panics_doc \
        -A clippy::missing_errors_doc \
        -A clippy::needless_pass_by_value \
        -A clippy::manual_assert \
        \
        -D warnings


test:
	cargo test --all && cargo test --all --release
