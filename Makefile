all: gen_bindings copy_bindings

gen_bindings:
	rm -rf ./server/bindings
	cd ./server
	cargo test export_bindings

copy_bindings:
	rm -rf ./client/bindings
	mkdir -p ./client/bindings
	cp -R ./server/bindings/* ./client/bindings/
