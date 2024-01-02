Help()
{
# Display Help
echo #docs-site/content/docs/getting-started/config.md:$ LOCO_ENV=qa cargo loco start
echo #LOCO_ENV=test blo-cli task
echo #LOCO_CI_MODE
echo #LOCO_DEBUG_PATH
echo #LOCO_APP_NAME= app name
echo #LOCO_TEMPLATE= {saas|lightweit-service|rest-api}
echo #LOCO_FOLDER_NAME
echo #LOCO_MYSQL_TABLE_OPTIONS
echo #LOCO_ENV={test,development,production}
echo #LOCO_DEPLOYMENT_KIND
}

echo "$0 appname template={saas|lightweit-service|rest-api} path"
echo "LOCO_APP_NAME=\$1 LOCO_TEMPLATE=\$2 loco new -p \$3 "
# perl pi -e 's!!!' config/development.yaml
# perl pi -e 's!!!' config/test.yaml
#cargo test && cargo install --path .
#$1-cli start
