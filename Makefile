.PHONY: gusto
gusto:
	openapitor \
		--input specs/gusto.v1.yaml \
		--version 2.1.17 \
		--output ./gusto \
		--name gusto-api \
		--base-url https://api.gusto.com \
		--description "A fully generated & opinionated API client for the Gusto API." \
		--token-endpoint "https://api.gusto.com/oauth/token" \
		--user-consent-endpoint "https://api.gusto.com/oauth/authorize"

# Spec is from: https://github.com/frontapp/front-api-specs/blob/main/core-api/core-api.json
.PHONY: front
front:
	openapitor \
		--input specs/front.json \
		--version 0.0.2 \
		--output ./front \
		--name front-api \
		--base-url https://api2.frontapp.com \
		--description "A fully generated & opinionated API client for the Front API."

# Spec is from: npx swagger2openapi --outfile ./specs/mailchimp.json --patch https://api.mailchimp.com/schema/3.0/Swagger.json?expand
.PHONY: mailchimp
mailchimp:
	../kittycad.rs/target/debug/openapitor \
		--input specs/mailchimp.json \
		--version 0.0.2 \
		--output ./mailchimp \
		--name mailchimp-api \
		--base-url https://us1.api.mailchimp.com \
		--description "A fully generated & opinionated API client for the MailChimp API." \
		--token-endpoint "https://login.mailchimp.com/oauth2/token" \
		--user-consent-endpoint "https://login.mailchimp.com/oauth2/authorize"

.PHONY: ramp
ramp:
	../kittycad.rs/target/debug/openapitor \
		--input specs/ramp.json \
		--version 0.0.2 \
		--output ./ramp \
		--name ramp-api \
		--base-url https://api.ramp.com \
		--description "A fully generated & opinionated API client for the ramp API." \
		--token-endpoint "https://api.ramp.com/v1/public/customer/token" \
		--user-consent-endpoint "https://app.ramp.com/v1/authorize"


