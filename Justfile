# API := "1342"
# APP := "1369"
WEB := "4213"

# API_PACKAGE := "lyrichar-api"
# APP_PACKAGE := "lyrichar-app"
WEB_PACKAGE := "lyrichar-web"

web:
    dx run --release --port {{ WEB }} --package {{ WEB_PACKAGE }}
