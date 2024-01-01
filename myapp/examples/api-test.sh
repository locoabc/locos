# Health checks
curl localhost:3000/api/_ping  ; echo
curl localhost:3000/api/_health ; echo
# Hello
curl localhost:3000/api/guide ; echo
curl localhost:3000/api/home/hello ; echo

# Post article
curl -X POST -H "Content-Type: application/json" -d '{
  "title": "Your Title",
  "content": "Your Content xxx"
}' localhost:3000/api/articles
curl localhost:3000/api/articles # get a list of artichle
# Users
curl localhost:3000/api/user/current;echo  # get name of current users
curl localhost:3000/api/user;echo          # get list of users

