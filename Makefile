.PHONY: train get_data

train:
	cargo run --bin trainer

get_data:
	cargo run --bin get_training_data
