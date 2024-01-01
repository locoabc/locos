docker ps -a |egrep  'IMAGE|redis\:|postgres\:|mailtutan|devcontainer-app'
echo "BEGIN -- .env config --"
cat .env
echo 
echo "ENG   -- .env config --"
