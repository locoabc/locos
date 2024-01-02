# Health checks
curl localhost:3000/api/_ping
curl localhost:3000/api/_health
# Hello
curl localhost:3000/api/guide
curl localhost:3000/api/home/hello

# Post article 
curl -X POST -H "Content-Type: application/json" -d '{
  "title": "Your Title",
  "content": "Your Content xxx"
}' localhost:3000/api/articles
curl localhost:3000/api/articles # get a list of artichle
# Users
curl localhost:3000/api/user/current # get name of current users
curl localhost:3000/api/user         # get list of users
