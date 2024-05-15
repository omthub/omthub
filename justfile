
watch:
	cargo leptos watch
serve:
	cargo leptos serve --release
container:
	nix build "./#container"
	docker load -i result
	docker run --rm --env-file .env -p 3000:3000 site-server
trace:
	cargo leptos serve --bin-features chrome-tracing
