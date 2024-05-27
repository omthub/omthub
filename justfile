
watch:
	cargo leptos watch
serve:
	cargo leptos serve --release
container:
	nix build "./#container"
	docker load -i result
	@ # if you're wondering why I added `--init`, it's because I built this
	@ # for fly.io which uses firecracker, so I left `tini` out of the image
	docker run --rm --init --env-file .env -p 3000:3000 site-server
trace:
	cargo leptos serve --bin-features chrome-tracing

surreal:
	mkdir /tmp/surreal_data -p && surreal start file:/tmp/omthub_surreal_data --log=info --auth
wipe-surreal:
	rm -rf /tmp/omthub_surreal_data
apply-surreal:
	surrealdb-migrations apply
