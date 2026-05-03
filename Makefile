.PHONY: dev-web dev-docs build-docs build-gallery build-web-dist

dev-web:
	cd crates/story-web && make dev

dev-docs:
	cd docs && bun install && bun run dev

build-docs:
	cd docs && bun install && bun run build

build-gallery:
	cd crates/story-web && make build-prod

build-web-dist:
	./script/build-web-dist.sh
