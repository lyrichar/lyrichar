API := "1313"
# APP := "1369"
WEB := "1342"

API_PACKAGE := "lyrichar-api"
# APP_PACKAGE := "lyrichar-app"
WEB_PACKAGE := "lyrichar-web"

api:
    cargo run --release --package {{ API_PACKAGE }} -- --port {{ API }}

web:
    dx run --release --package {{ WEB_PACKAGE }} --port {{ WEB }}

fmt:
    dx fmt
