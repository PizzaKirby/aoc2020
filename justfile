set shell := ["nu","-c"]

run DAY:
	cargo run --bin day{{DAY}}

new DAY:
	nu new.nu {{DAY}}

test DAY:
	cargo test --bin day{{DAY}}