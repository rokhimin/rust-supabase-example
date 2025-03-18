### About

Example Rust connection Supabase.com and make simple REST Api

### Preparation

.env
```
SUPABASE_URL=xxx
SUPABASE_KEY=xxx
```

### Use

index 
```
http://127.0.0.1:3000
```

GET
```
http://127.0.0.1:3000/users
```

POST
(use curl)
```
curl -X POST http://127.0.0.1:3000/add_user   -H "Content-Type: application/json"   -d '{"name":"whdzera","email":"whdzera@example.com"}'
```

