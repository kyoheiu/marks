lint:
	cd axum && cargo fmt
	cd svelte && npm run lint:fix

fe:
	cd svelte && npm run dev

dev:
	cd svelte && npm run build
	cp -r ./svelte/dist/* ./axum/static
	cd axum && RUST_LOG=debug cargo run

build:
	sudo docker build --tag=kyoheiudev/marks:$(VER) .

push:
	sudo docker push kyoheiudev/marks:$(VER)
