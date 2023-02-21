.PHONY: commonroom
commonroom:
	openapitor \
		--input specs/commonroom.json \
		--version 0.1.0 \
		--output ./commonroom \
		--name commonroom-api \
		--description "A fully generated & opinionated API client for the Common Room API." \
		--base-url "https://api.commonroom.io/community/v1" $(EXTRA_ARGS)

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

.PHONY: remote
remote:
	openapitor \
		--input specs/remote.json \
		--version 0.1.0 \
		--output ./remote \
		--name remote-api \
		--description "A fully generated & opinionated API client for the Remote API." \
		--base-url "https://gateway.remote.com" \
		--date-time-format "%Y-%m-%dT%H:%M:%S" $(EXTRA_ARGS)

# Spec is from https://raw.githubusercontent.com/twilio/twilio-oai/main/spec/json/twilio_api_v2010.json
.PHONY: twilio
twilio:
	openapitor \
		--input specs/twilio.json \
		--version 0.1.0 \
		--output ./twilio \
		--name twilio-api \
		--description "A fully generated & opinionated API client for the Twilio API." \
		--base-url "https://api.twilio.com" \
		--date-time-format "%a, %d %b %Y %H:%M:%S %z" $(EXTRA_ARGS)
