dev:
	cd svelte && npm run build
	cp -r ./svelte/dist/* ./axum/static
	cd axum && RUST_LOG=debug cargo run

build:
	sudo docker build --tag=kyoheiudev/marks:$(VER) .

push:
	sudo docker push kyoheiudev/marks:$(VER)
