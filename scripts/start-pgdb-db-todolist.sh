docker run -d -p 5432:5432 -e POSTGRES_USER=loco -e POSTGRES_DB=todolist_development -e POSTGRES_PASSWORD="loco" postgres:15.3-alpine
docker ps -a |egrep 'CONTAIN|alpine'
