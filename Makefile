dev:
	cd svelte && npm run build
	cp -r ./svelte/dist/* ./axum/static
	cd axum && LTD_SECRET_KEY=test LTD_DOMAIN=localhost LTD_NETWORK=ldap://localhost:3890 RUST_LOG=debug cargo run

build:
	sudo docker build --tag=kyoheiudev/ltd:$(VER) .

push:
	sudo docker push kyoheiudev/ltd:$(VER)
