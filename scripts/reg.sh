curl --location '127.0.0.1:3000/auth/register' \
     --header 'Content-Type: application/json' \
     --data-raw '{
         "name": "Loco user",
         "email": "user@loco.rs",
         "password": "12341234"
     }'
