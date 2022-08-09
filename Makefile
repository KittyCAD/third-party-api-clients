.PHONY: gusto
gusto:
	openapitor \
		--input specs/gusto.v1.yaml \
		--version 2.1.16 \
		--output ./gusto \
		--name gusto-api \
		--base-url https://api.gusto.com \
		--description "A fully generated & opinionated API client for the Gusto API." \
		--token-endpoint "https://api.gusto.com/oauth/token" \
		--user-consent-endpoint "https://api.gusto.com/oauth/authorize"

.PHONY: front
front:
	openapitor \
		--input specs/front.json \
		--version 0.0.1 \
		--output ./front \
		--name front-api \
		--base-url https://api2.frontapp.com \
		--description "A fully generated & opinionated API client for the Front API."
