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

# Spec is from: https://github.com/frontapp/front-api-specs/blob/main/core-api/core-api.json
.PHONY: front
front:
	../kittycad.rs/target/debug/openapitor \
		--input specs/front.json \
		--version 0.0.1 \
		--output ./front \
		--name front-api \
		--base-url https://api2.frontapp.com \
		--description "A fully generated & opinionated API client for the Front API."

# Spec is from: https://github.com/PagerDuty/api-schema/blob/main/reference/REST/openapiv3.json
.PHONY: pagerduty
pagerduty:
	../kittycad.rs/target/debug/openapitor \
		--input specs/pagerduty.json \
		--version 0.0.1 \
		--output ./pagerduty \
		--name pagerduty-api \
		--base-url https://api.pagerduty.com \
		--description "A fully generated & opinionated API client for the PagerDuty API."
