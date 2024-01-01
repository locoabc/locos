curl --location --request GET '127.0.0.1:3000/user/current' \
     --header 'Content-Type: application/json' \
     --header 'Authorization: Bearer TOKEN' \
     --data-raw '{
         "email": "user@loco.rs",
         "password": "new-password"
     }'
