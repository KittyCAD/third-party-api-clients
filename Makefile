openapitor_exe = kittycad.rs/target/debug/openapitor

.PHONY: commonroom
commonroom: openapitor
	pwd
	$(openapitor_exe) \
		--input specs/commonroom.json \
		--target-version 0.1.0 \
		--output ./commonroom \
		--name commonroom-api \
		--description "A fully generated & opinionated API client for the Common Room API." \
		--request-timeout-seconds 60 \
		--base-url "https://api.commonroom.io/community/v1" $(EXTRA_ARGS)

# Spec is from: https://github.com/frontapp/front-api-specs/blob/main/core-api/core-api.json
.PHONY: front
front: openapitor
	$(openapitor_exe) \
		--input specs/front.json \
		--target-version 0.0.3 \
		--output ./front \
		--name front-api \
		--base-url https://api2.frontapp.com \
		--request-timeout-seconds 60 \
		--description "A fully generated & opinionated API client for the Front API."

.PHONY: gusto
gusto: openapitor
	$(openapitor_exe) \
		--input specs/gusto.v1.yaml \
		--target-version 2.1.17 \
		--output ./gusto \
		--name gusto-api \
		--base-url https://api.gusto.com \
		--description "A fully generated & opinionated API client for the Gusto API." \
		--token-endpoint "https://api.gusto.com/oauth/token" \
		--request-timeout-seconds 60 \
		--user-consent-endpoint "https://api.gusto.com/oauth/authorize"

# root spec for hubspot api:
# https://api.hubspot.com/api-catalog-public/v1/apis
# We've just plucked crm -> contacts api spec below.
# https://api.hubspot.com/api-catalog-public/v1/apis/crm/v3/objects/contacts
.PHONY: hubspot-contacts
hubspot-contacts: openapitor
	$(openapitor_exe) \
		--input specs/hubspot-contacts.json \
		--target-version 0.1.2 \
		--output ./hubspot-contacts \
		--name hubspot-contacts \
		--base-url https://api.hubapi.com \
		--description "A fully generated & opinionated API client for the Hubspot Contacts API." \
		--request-timeout-seconds 60 \

# root spec for hubspot api:
# https://api.hubspot.com/api-catalog-public/v1/apis
# We've just plucked crm -> users api spec below.
# https://api.hubspot.com/api-catalog-public/v1/apis/users
.PHONY: hubspot-users
hubspot-users: openapitor
	$(openapitor_exe) \
		--input specs/hubspot-users.json \
		--target-version 0.1.2 \
		--output ./hubspot-users \
		--name hubspot-users \
		--base-url https://api.hubspot.com \
		--description "A fully generated & opinionated API client for the Hubspot Users API." \
		--request-timeout-seconds 60 \

# Spec is from: npx swagger2openapi --outfile ./specs/mailchimp.json --patch https://api.mailchimp.com/schema/3.0/Swagger.json?expand
.PHONY: mailchimp
mailchimp: openapitor
	$(openapitor_exe) \
		--input specs/mailchimp.json \
		--target-version 0.0.2 \
		--output ./mailchimp \
		--name mailchimp-api \
		--base-url https://us1.api.mailchimp.com \
		--description "A fully generated & opinionated API client for the MailChimp API." \
		--token-endpoint "https://login.mailchimp.com/oauth2/token" \
		--request-timeout-seconds 60 \
		--user-consent-endpoint "https://login.mailchimp.com/oauth2/authorize"

.PHONY: ramp
ramp: openapitor
	$(openapitor_exe) \
		--input specs/ramp.json \
		--target-version 0.0.2 \
		--output ./ramp \
		--name ramp-api \
		--base-url https://api.ramp.com \
		--description "A fully generated & opinionated API client for the ramp API." \
		--token-endpoint "https://api.ramp.com/v1/public/customer/token" \
		--request-timeout-seconds 60 \
		--user-consent-endpoint "https://app.ramp.com/v1/authorize"

.PHONY: remote
remote: openapitor
	$(openapitor_exe) \
		--input specs/remote.json \
		--target-version 0.1.1 \
		--output ./remote \
		--name remote-api \
		--description "A fully generated & opinionated API client for the Remote API." \
		--base-url "https://gateway.remote.com" \
		--request-timeout-seconds 60 \
		--date-time-format "%Y-%m-%dT%H:%M:%S" $(EXTRA_ARGS)

.PHONY: rippling
rippling: openapitor
	$(openapitor_exe) \
		--input specs/rippling.yaml \
		--target-version 0.1.8 \
		--output ./rippling \
		--name rippling-api \
		--description "A fully generated & opinionated API client for the Rippling API." \
		--base-url "https://rest.ripplingapis.com" \
		--request-timeout-seconds 60 \
		--date-time-format "%Y-%m-%dT%H:%M:%S" $(EXTRA_ARGS)

# Spec is from https://raw.githubusercontent.com/twilio/twilio-oai/main/spec/json/twilio_api_v2010.json
.PHONY: twilio
twilio: openapitor
	$(openapitor_exe) \
		--input specs/twilio.json \
		--target-version 0.1.0 \
		--output ./twilio \
		--name twilio-api \
		--description "A fully generated & opinionated API client for the Twilio API." \
		--base-url "https://api.twilio.com" \
		--basic-auth \
		--request-timeout-seconds 60 \
		--date-time-format "%a, %d %b %Y %H:%M:%S %z" $(EXTRA_ARGS)

.PHONY: vercel
vercel: openapitor
	$(openapitor_exe) \
		--input specs/vercel.json \
		--target-version 0.1.1 \
		--output ./vercel \
		--name vercel-api \
		--description "A fully generated & opinionated API client for the Vercel API." \
		--request-timeout-seconds 60 \
		--base-url "https://api.vercel.com"  $(EXTRA_ARGS)


.PHONY: openapitor
openapitor:
	cd kittycad.rs/openapitor && cargo build

# Mailchimp is currently broken so it's not in this list.
.PHONY: all
all:
	make openapitor
	make commonroom
	make front
	make gusto
	make ramp
	make remote
	make twilio
