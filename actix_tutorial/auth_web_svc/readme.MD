# Actix Auth Web Microservice
https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/

## Postgres
```shell script
docker run --name test-pg -e POSTGRES_PASSWORD=mysecretpassword -d -p 5432:5432 postgres
```

## Testing Endpoints
### Create Invitation
```shell script
curl --request POST \
  --url http://localhost:3000/api/invitation \
  --header 'content-type: application/json' \
  --data '{"email":"test@test.com"}'
```

### Register User
Note: as the invitation must be valid (issued within the last 24 hours) you'll need to adjust the `invitation-id` with the UUID from the Invitation request made above

```shell script
curl --request POST \
  --url http://localhost:3000/api/register/f6d0a474-d1ec-4ec5-b7e2-1723dd811b27 \
  --header 'content-type: application/json' \
  --data '{"password":"password"}'
```

### Auth Server

#### Login
```shell script
curl -i --request POST \
  --url http://localhost:3000/api/auth \
  --header 'content-type: application/json' \
  --data '{"email": "test@test.com","password":"password"}'
```

#### Logout
```shell script
curl -i --request DELETE \
  --url http://localhost:3000/auth
```

#### Get User
_Need to use the cookie from the login request_
```shell script
curl -i --request GET \
  --url http://localhost:3000/auth \
  --cookie auth=sJkqaUG/pC2oPVVitQlodlmo/5B7Y7YoDtLfyafOen5V3i3FygrooAqSCkpTh4LiN2NhJik=
```